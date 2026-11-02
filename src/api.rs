use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{bail, Context as _};
use cargo_metadata::{Dependency as CargoDependency, Package, PackageId};
use cargo_smart_release_core::api_check::{ReleaseDependency, ReleaseRequest};
use serde_json::Value;

use crate::{traverse::Dependency, utils::package_by_id, Context};

struct Policy {
    command: Vec<String>,
    unstable_features: Vec<String>,
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
            if let Some(policy) = Policy::from_metadata(&package.metadata)
                .with_context(|| format!("Invalid API-check metadata for '{}'", package.name))?
            {
                for feature in &policy.unstable_features {
                    if !package.features.contains_key(feature) {
                        bail!("Unknown unstable feature '{feature}' in '{}'", package.name);
                    }
                }
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
            for experimental in [false, true] {
                if self.skipped.contains(&(&package.id, experimental)) {
                    continue;
                }
                if experimental
                    && (self.incompatible.contains(&package.id) || self.experimental_incompatible.contains(&package.id))
                {
                    continue;
                }
                let request = ReleaseRequest {
                    package: package.name.to_string(),
                    manifest_path: package.manifest_path.clone().into_std_path_buf(),
                    baseline_revision: baseline_revision.clone(),
                    baseline_version: Some(baseline_version.clone()),
                    unstable_features: policy.unstable_features.clone(),
                    dependencies: dependencies.clone(),
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
                            policy.unstable_features.join(", ")
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

pub(crate) fn dependency_uses_unstable(
    consumer: &Package,
    dependency: &CargoDependency,
    package: &Package,
) -> anyhow::Result<bool> {
    Ok(Policy::from_metadata(&package.metadata)?
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
    fn from_metadata(metadata: &Value) -> anyhow::Result<Option<Self>> {
        let Some(metadata) = metadata.get("smart-release") else {
            return Ok(None);
        };
        let metadata = metadata
            .as_object()
            .context("package.metadata.smart-release must be a table")?;
        for key in metadata.keys() {
            if !["api-check", "unstable-features"].contains(&key.as_str()) {
                bail!("Unknown package.metadata.smart-release key '{key}'");
            }
        }
        let Some(command) = metadata.get("api-check") else {
            if metadata.contains_key("unstable-features") {
                bail!("unstable-features requires api-check");
            }
            return Ok(None);
        };
        let strings = |value: &Value| -> anyhow::Result<Vec<String>> {
            value
                .as_array()
                .context("Expected an array of strings")?
                .iter()
                .map(|value| value.as_str().map(str::to_owned).context("Expected a string"))
                .collect()
        };
        let command = strings(command).context("Invalid api-check command")?;
        if command.first().is_none_or(String::is_empty) {
            bail!("api-check needs a nonempty executable name");
        }
        Ok(Some(Self {
            command,
            unstable_features: metadata
                .get("unstable-features")
                .map(strings)
                .transpose()?
                .unwrap_or_default(),
        }))
    }
}
