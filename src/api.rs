use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{bail, Context as _};
use cargo_metadata::{Dependency as CargoDependency, Package, PackageId};
use cargo_smart_release_core::api_check::{ReleaseDependency, ReleaseRequest};
use serde_json::Value;

use crate::{
    command::parallel,
    traverse::Dependency,
    utils::{package_by_id, try_to_published_crate_and_new_version},
    Context,
};

struct Policy {
    command: Vec<String>,
    unstable_features: Vec<String>,
}

pub(crate) fn has_parallel_release(package: &Package, version: &semver::Version) -> bool {
    version.major > 0 && version.pre.is_empty() && package.features.contains_key("unstable")
}

/// A successful check bounds the stable API, not consumers of excluded features.
pub(crate) struct Checks<'meta> {
    policies: BTreeMap<&'meta PackageId, Policy>,
    pub incompatible: BTreeSet<&'meta PackageId>,
    pub experimental_incompatible: BTreeSet<&'meta PackageId>,
    results: BTreeMap<String, bool>,
    skipped: BTreeSet<(&'meta PackageId, bool)>,
    skip_failed_api_checks: bool,
}

impl<'meta> Checks<'meta> {
    pub fn new(
        ctx: &'meta Context,
        allow_auto_publish_of_stable_crates: bool,
        skip_api_check: bool,
    ) -> anyhow::Result<Self> {
        let mut policies = BTreeMap::new();
        for id in &ctx.meta.workspace_members {
            let package = package_by_id(&ctx.meta, id);
            if let Some(policy) = Policy::from_package(package)
                .with_context(|| format!("Invalid smart-release metadata for '{}'", package.name))?
            {
                if package.publish.is_none()
                    && (package.version.major == 0
                        || allow_auto_publish_of_stable_crates
                        || ctx.crate_names.contains(&package.name))
                {
                    policies.insert(&package.id, policy);
                }
            }
        }
        Ok(Self {
            policies,
            incompatible: BTreeSet::new(),
            experimental_incompatible: BTreeSet::new(),
            results: BTreeMap::new(),
            skipped: BTreeSet::new(),
            skip_failed_api_checks: skip_api_check,
        })
    }

    pub fn is_boundary(&self, package: &Package) -> bool {
        self.policies.contains_key(&package.id)
            && !self.incompatible.contains(&package.id)
            && !self.skipped.contains(&(&package.id, false))
    }

    pub fn requires_minor_release(&self, package: &Package) -> bool {
        self.experimental_incompatible.contains(&package.id) || self.skipped.contains(&(&package.id, true))
    }

    pub fn has_policy(&self, package: &Package) -> bool {
        self.policies.contains_key(&package.id)
    }

    pub fn uses_unstable(&self, consumer: &Package, dependency: &CargoDependency, package: &Package) -> bool {
        let Some(policy) = self.policies.get(&package.id) else {
            return false;
        };
        uses_unstable(consumer, dependency, package, &policy.unstable_features)
    }

    /// Recheck changed candidate versions until no further API boundary is rejected.
    pub fn verify(&mut self, ctx: &Context, crates: &[Dependency<'meta>]) -> anyhow::Result<bool> {
        if self.policies.is_empty() {
            return Ok(false);
        }
        let parallels = parallel::Plan::new(
            crates.iter().filter_map(try_to_published_crate_and_new_version),
            ctx,
            None,
        )?;
        let dependencies: Vec<_> = ctx
            .meta
            .workspace_members
            .iter()
            .map(|id| {
                let package = package_by_id(&ctx.meta, id);
                let bump = crates
                    .iter()
                    .find(|dep| dep.package.id == package.id)
                    .and_then(|dep| dep.mode.version_adjustment_bump());
                let next_version = bump.map_or(&package.version, |bump| &bump.next_release);
                let latest = ctx.crates_index.crate_(&package.name)?;
                let baseline_version = latest
                    .as_ref()
                    .map(|krate| semver::Version::parse(krate.highest_version().version()))
                    .transpose()?
                    .unwrap_or_else(|| package.version.clone());
                Ok(ReleaseDependency {
                    name: package.name.to_string(),
                    current_version: package.version.clone(),
                    baseline_version: baseline_version.clone(),
                    next_version: next_version.clone(),
                    breaking: crate::version::rhs_is_breaking_bump_for_lhs(&baseline_version, next_version),
                    experimental_breaking: self.requires_minor_release(package),
                })
            })
            .collect::<anyhow::Result<_>>()?;
        let mut changed = false;
        for dep in crates {
            let package = dep.package;
            let Some(policy) = self.policies.get(&package.id) else {
                continue;
            };
            if self.incompatible.contains(&package.id) {
                continue;
            }
            let latest = ctx.crates_index.crate_(&package.name)?;
            let baseline_version = latest
                .as_ref()
                .map(|krate| semver::Version::parse(krate.highest_version().version()))
                .transpose()?
                .unwrap_or_else(|| package.version.clone());
            let baseline_revision = crate::utils::release_tag_name(package, &baseline_version, &ctx.repo)?;
            let next_version = dep
                .mode
                .version_adjustment_bump()
                .map_or(&package.version, |bump| &bump.next_release);
            let parallel = has_parallel_release(package, next_version);
            let mut unstable_features = policy.unstable_features.clone();
            if parallel && !unstable_features.iter().any(|feature| feature == "unstable") {
                unstable_features.push("unstable".into());
            }
            for experimental in [false, true] {
                if self.skipped.contains(&(&package.id, experimental)) {
                    continue;
                }
                if experimental
                    && (parallel
                        || self.incompatible.contains(&package.id)
                        || self.experimental_incompatible.contains(&package.id))
                {
                    continue;
                }
                let request = ReleaseRequest {
                    package: package.name.to_string(),
                    manifest_path: package.manifest_path.clone().into_std_path_buf(),
                    baseline_revision: baseline_revision.clone(),
                    baseline_version: Some(baseline_version.clone()),
                    unstable_features: unstable_features.clone(),
                    dependencies: routed_dependencies(
                        dependencies.clone(),
                        parallels.api_routes(package, ctx, experimental)?,
                        latest.as_ref().map(|entry| entry.highest_version()),
                    )?,
                    experimental,
                };
                let request = serde_json::to_string(&request)?;
                let compatible = match self.results.get(&request) {
                    Some(result) => *result,
                    None => {
                        log::info!(
                            "Checking {} API compatibility of '{}' against {} (experimental features: {})",
                            if experimental { "experimental" } else { "stable" },
                            package.name,
                            baseline_revision,
                            unstable_features.join(", ")
                        );
                        let compatible = (|| {
                            let mut child = Command::new(&policy.command[0])
                                .args(&policy.command[1..])
                                .current_dir(&ctx.root)
                                .stdin(Stdio::piped())
                                .spawn()
                                .context("Could not start API checker")?;
                            let input_result = child
                                .stdin
                                .take()
                                .context("API checker stdin was not piped")?
                                .write_all(request.as_bytes());
                            let status = child.wait().context("Could not wait for API checker")?;
                            input_result.context("Could not send the API check request")?;
                            match status.code() {
                                Some(0) => Ok(true),
                                Some(10) => Ok(false),
                                _ => bail!(
                                    "API checker for '{}' failed ({status}); expected exit 0 (compatible) or 10 (incompatible)",
                                    package.name
                                ),
                            }
                        })().with_context(|| format!(
                            "API checker for '{}' could not complete against '{}'",
                            package.name,
                            baseline_revision
                        ));
                        let compatible = match compatible {
                            Ok(compatible) => compatible,
                            Err(err) if self.skip_failed_api_checks => {
                                log::warn!(
                                    "Skipping incomplete {} API check for '{}'; using conservative version and dependency rules: {err:#}",
                                    if experimental { "experimental" } else { "stable" },
                                    package.name
                                );
                                changed |= self.skipped.insert((&package.id, experimental));
                                continue;
                            }
                            Err(err) => return Err(err).context(
                                "If the old release cannot build (for example, missing forwarded Cargo features), and this release fixes it, manually review compatibility and retry cargo smart-release with --no-api-check. This only skips checks that cannot complete for this invocation and uses conservative version and dependency rules for them; completed compatibility and incompatibility verdicts are retained. Check the dry-run plan first; a failed build does not establish API compatibility."
                            ),
                        };
                        self.results.insert(request, compatible);
                        compatible
                    }
                };
                if !compatible {
                    if experimental {
                        log::info!(
                            "Experimental API of '{}' requires at least a minor release",
                            package.name
                        );
                        changed |= self.experimental_incompatible.insert(&package.id);
                    } else {
                        log::info!("Stable API of '{}' requires a breaking release", package.name);
                        changed |= self.incompatible.insert(&package.id);
                    }
                }
            }
        }
        Ok(changed)
    }
}

fn routed_dependencies(
    mut dependencies: Vec<ReleaseDependency>,
    routes: Vec<(&CargoDependency, Option<semver::Version>)>,
    baseline: Option<&crates_index::Version>,
) -> anyhow::Result<Vec<ReleaseDependency>> {
    for (edge, routed_version) in routes {
        let Some(dependency) = dependencies.iter_mut().find(|dependency| dependency.name == edge.name) else {
            continue;
        };
        let previous = baseline
            .into_iter()
            .flat_map(crates_index::Version::dependencies)
            .find(|previous| {
                previous.kind() == crates_index::DependencyKind::Normal
                    && previous.crate_name() == edge.name
                    && previous.name() == edge.rename.as_deref().unwrap_or(&edge.name)
                    && previous.target().and_then(|target| target.parse().ok()) == edge.target
            })
            .map(|previous| semver::VersionReq::parse(previous.requirement()))
            .transpose()?;
        let comparator = previous.as_ref().and_then(|req| req.comparators.first());
        let Some(version) = routed_version.or_else(|| {
            (dependency.next_version.major > 0 && comparator.is_some_and(|previous| previous.major == 0))
                .then(|| dependency.next_version.clone())
        }) else {
            continue;
        };
        let breaking = previous.as_ref().map_or_else(
            || {
                dependency.baseline_version.major > 0
                    || crate::version::rhs_is_breaking_bump_for_lhs(&dependency.baseline_version, &version)
            },
            |previous| !previous.matches(&version),
        );
        if dependency.next_version.major > 0 || breaking {
            if let Some(comparator) = comparator {
                dependency.baseline_version = semver::Version::new(
                    comparator.major,
                    comparator.minor.unwrap_or_default(),
                    comparator.patch.unwrap_or_default(),
                );
            }
            dependency.next_version = version;
            dependency.breaking = breaking;
        }
    }
    Ok(dependencies)
}

pub(crate) fn dependency_uses_unstable(
    consumer: &Package,
    dependency: &CargoDependency,
    package: &Package,
) -> anyhow::Result<bool> {
    Ok(Policy::from_package(package)?
        .is_some_and(|policy| uses_unstable(consumer, dependency, package, &policy.unstable_features)))
}

fn uses_unstable(
    consumer: &Package,
    dependency: &CargoDependency,
    package: &Package,
    unstable_features: &[String],
) -> bool {
    let dependency_name = dependency.rename.as_deref().unwrap_or(&dependency.name);
    let mut pending = dependency.features.clone();
    if dependency.uses_default_features {
        pending.push("default".into());
    }
    // Any consumer feature can expose unstable API, including optional forwarding.
    for feature in consumer.features.values().flatten() {
        if let Some((name, feature)) = feature.split_once('/') {
            if name.trim_end_matches('?') == dependency_name {
                pending.push(feature.into());
            }
        }
    }
    let mut seen = BTreeSet::new();
    while let Some(feature) = pending.pop() {
        if unstable_features.contains(&feature) {
            return true;
        }
        if seen.insert(feature.clone()) {
            if let Some(features) = package.features.get(&feature) {
                pending.extend(features.iter().cloned());
            }
        }
    }
    false
}

impl Policy {
    fn from_package(package: &Package) -> anyhow::Result<Option<Self>> {
        let Some(metadata) = package.metadata.get("smart-release") else {
            return Ok(None);
        };
        let metadata = metadata
            .as_object()
            .context("package.metadata.smart-release must be a table")?;
        for key in metadata.keys() {
            if ![
                "api-check",
                "unstable-features",
                "github-release",
                "unstable-stable-dependency",
            ]
            .contains(&key.as_str())
            {
                bail!("Unknown package.metadata.smart-release key '{key}'");
            }
        }
        if let Some(value) = metadata.get("github-release") {
            value
                .as_bool()
                .context("package.metadata.smart-release.github-release must be a boolean")?;
        }
        if let Some(value) = metadata.get("unstable-stable-dependency") {
            value
                .as_bool()
                .context("package.metadata.smart-release.unstable-stable-dependency must be a boolean")?;
        }
        let strings = |value: &Value| -> anyhow::Result<Vec<String>> {
            value
                .as_array()
                .context("Expected an array of strings")?
                .iter()
                .map(|value| value.as_str().map(str::to_owned).context("Expected a string"))
                .collect()
        };
        let mut unstable_features = metadata
            .get("unstable-features")
            .map(strings)
            .transpose()?
            .unwrap_or_default();
        for feature in &unstable_features {
            if !package.features.contains_key(feature) {
                bail!("Unknown unstable feature '{feature}' in '{}'", package.name);
            }
        }
        let Some(command) = metadata.get("api-check") else {
            if metadata.contains_key("unstable-features") && !package.features.contains_key("unstable") {
                bail!("unstable-features requires api-check or a parallel unstable release");
            }
            return Ok(None);
        };
        if has_parallel_release(package, &package.version)
            && !unstable_features.iter().any(|feature| feature == "unstable")
        {
            unstable_features.push("unstable".into());
        }
        let command = strings(command).context("Invalid api-check command")?;
        if command.first().is_none_or(String::is_empty) {
            bail!("api-check needs a nonempty executable name");
        }
        Ok(Some(Self {
            command,
            unstable_features,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_consumer_already_on_the_same_companion_series_stays_compatible() -> anyhow::Result<()> {
        let edge = serde_json::from_value::<CargoDependency>(serde_json::json!({
            "name": "api", "req": "^1.0", "kind": "normal", "optional": false,
            "uses_default_features": true, "features": [],
        }))?;
        let previous = serde_json::from_value::<crates_index::Version>(serde_json::json!({
            "name": "consumer", "vers": "1.2.3", "features": {}, "cksum": "0".repeat(64),
            "deps": [{ "name": "api", "req": "^0.3.0", "features": [], "optional": false, "default_features": true }],
        }))?;
        for (routed, breaking) in [(Some("0.3.1"), false), (Some("0.4.0"), true), (None, true)] {
            let dependencies = vec![ReleaseDependency {
                name: "api".into(),
                current_version: semver::Version::new(1, 0, 0),
                baseline_version: semver::Version::new(1, 0, 0),
                next_version: semver::Version::new(1, 0, 1),
                breaking: false,
                experimental_breaking: false,
            }];
            let planned = routed_dependencies(
                dependencies,
                vec![(&edge, routed.map(semver::Version::parse).transpose()?)],
                Some(&previous),
            )?;
            assert_eq!(planned[0].breaking, breaking, "published series ^0.3.0 vs. {routed:?}");
        }
        Ok(())
    }
}
