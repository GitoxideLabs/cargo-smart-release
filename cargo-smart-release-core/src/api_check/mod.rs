//! Supported API audits and compatibility checks, shared by local checks and release hooks.
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context as _};
use cargo_metadata::{MetadataCommand, Package};

mod check;
mod types;
pub use check::{audit, boundary, release};
pub use types::{
    Audit, Baseline, FeatureProfile, Incompatibility, Incompatible, Reason, ReleaseDependency, ReleaseRequest,
};

/// Resolve Cargo metadata while preserving an existing workspace lockfile.
///
/// `features` contains Cargo feature-selection arguments such as `--no-default-features`.
pub fn metadata(manifest: Option<&Path>, features: &[String]) -> anyhow::Result<cargo_metadata::Metadata> {
    let mut probe = MetadataCommand::new();
    probe.no_deps();
    let mut command = MetadataCommand::new();
    if let Some(manifest) = manifest {
        probe.manifest_path(manifest);
        command.manifest_path(manifest);
    }
    let workspace = probe.exec()?;
    let mut options = features.to_vec();
    // Preserve existing lockfiles; Cargo may create one for a workspace that has never resolved dependencies.
    if workspace.workspace_root.join("Cargo.lock").exists() {
        options.push("--locked".into());
    }
    command.other_options(options);
    Ok(command.exec()?)
}

/// Check a workspace crate, including uncommitted changes, against its supported release baseline.
///
/// Without an explicit revision, select the newest reachable release tag whose manifest declares
/// `package.metadata.api-check`. If none exists yet, audit the current boundary alone.
/// This compares manifest dependency versions; use the release planner to check proposed versions.
///
/// Uses the `**Unstable.**` and `**Unstable:**` documentation markers described in [`boundary`]
/// to exclude experimental items from the supported API and compatibility comparisons.
/// Supported APIs that expose experimental types are still reported as boundary violations.
///
/// Returns `Ok(None)` if all supported API checks pass, or `Ok(Some(report))` with all
/// detected incompatibilities and boundary violations. Without a baseline, this reflects
/// only the current boundary audit, not compatibility with a previous release.
/// Returns `Err` if the check cannot complete, for example due to invalid configuration or a tool failure.
pub fn check(
    crate_name: &str,
    manifest: Option<&Path>,
    baseline: Option<&str>,
) -> anyhow::Result<Option<Incompatible>> {
    let mut command = MetadataCommand::new();
    command.no_deps();
    if let Some(manifest) = manifest {
        command.manifest_path(manifest);
    }
    let metadata = command.exec()?;
    let package = metadata
        .workspace_packages()
        .into_iter()
        .find(|package| package.name == crate_name)
        .with_context(|| format!("Workspace member '{crate_name}' was not found"))?;
    if package.metadata.get("api-check").is_none() {
        bail!("'{crate_name}' needs package.metadata.api-check.profiles to declare its supported API");
    }
    let root = git_root(metadata.workspace_root.as_std_path())?;
    let revision = match baseline {
        Some(revision) => Some(resolve_revision(&root, revision)?),
        None => supported_baseline(package, &root)?,
    };
    let incompatible = if let Some(revision) = revision {
        tracing::info!("Comparing {crate_name} with {revision} (including uncommitted changes)");
        let request = ReleaseRequest {
            package: package.name.to_string(),
            manifest_path: package.manifest_path.clone().into_std_path_buf(),
            baseline_version: None,
            baseline_revision: revision,
            unstable_features: package.metadata["smart-release"]
                .get("unstable-features")
                .map(|features| serde_json::from_value(features.clone()))
                .transpose()?
                .unwrap_or_default(),
            dependencies: Vec::new(),
        };
        release(&request, None)?
    } else {
        tracing::info!("No supported release baseline for {crate_name}; checking the current boundary only");
        boundary(package.manifest_path.as_std_path())?
    };
    tracing::info!(
        "{crate_name}: {}",
        if incompatible.is_none() {
            "API checks passed"
        } else {
            "API incompatibility detected"
        }
    );
    Ok(incompatible)
}

fn resolve_revision(root: &Path, revision: &str) -> anyhow::Result<String> {
    let output = Command::new("git")
        .current_dir(root)
        .args([
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{revision}^{{commit}}"),
        ])
        .output()?;
    if !output.status.success() {
        bail!(
            "Could not resolve baseline '{revision}': {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

fn supported_baseline(package: &Package, root: &Path) -> anyhow::Result<Option<String>> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["tag", "--list", "--merged", "HEAD"])
        .output()?;
    if !output.status.success() {
        bail!(
            "Could not list release tags: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let tags = String::from_utf8(output.stdout)?;
    let manifest = package.manifest_path.canonicalize()?;
    let prefix = (manifest.parent() != Some(root)).then_some(package.name.as_str());
    let mut candidates: Vec<_> = tags
        .lines()
        .filter_map(|tag| tag_version(prefix, tag).map(|version| (version, tag)))
        .collect();
    candidates.sort_unstable();
    let path = manifest
        .strip_prefix(root)?
        .to_str()
        .context("Manifest path is not UTF-8")?
        .replace('\\', "/");
    for (_, tag) in candidates.into_iter().rev() {
        let revision = resolve_revision(root, tag)?;
        let output = Command::new("git")
            .current_dir(root)
            .args(["show", &format!("{revision}:{path}")])
            .output()?;
        if !output.status.success() {
            bail!(
                "Could not read {path} in release tag '{tag}': {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        let manifest: toml_edit::DocumentMut = String::from_utf8(output.stdout)?
            .parse()
            .with_context(|| format!("Invalid Cargo manifest in release tag '{tag}'"))?;
        if manifest
            .get("package")
            .and_then(|package| package.get("metadata"))
            .and_then(|metadata| metadata.get("api-check"))
            .is_some()
        {
            tracing::info!("Using supported release tag {tag}");
            return Ok(Some(revision));
        }
    }
    Ok(None)
}

fn tag_version(package: Option<&str>, tag: &str) -> Option<semver::Version> {
    let tag = match package {
        Some(name) => tag.strip_prefix(name)?.strip_prefix('-')?,
        None => tag,
    };
    semver::Version::parse(
        tag.strip_prefix("vers")
            .or_else(|| tag.strip_prefix('v'))
            .unwrap_or(tag),
    )
    .ok()
}

fn git_root(directory: &Path) -> anyhow::Result<PathBuf> {
    let output = Command::new("git")
        .current_dir(directory)
        .args(["rev-parse", "--show-toplevel"])
        .output()?;
    if !output.status.success() {
        bail!(
            "Could not find a Git working tree: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let root = String::from_utf8(output.stdout)?;
    let root = root.strip_suffix('\n').unwrap_or(&root);
    let root = if cfg!(windows) {
        root.strip_suffix('\r').unwrap_or(root)
    } else {
        root
    };
    Ok(Path::new(root).canonicalize()?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn tags_follow_release_version_prefixes() -> anyhow::Result<()> {
        let expected = semver::Version::parse("1.2.3")?;
        for tag in ["1.2.3", "v1.2.3", "vers1.2.3"] {
            assert_eq!(super::tag_version(None, tag), Some(expected.clone()));
            assert_eq!(
                super::tag_version(Some("example"), &format!("example-{tag}")),
                Some(expected.clone())
            );
            assert_eq!(super::tag_version(Some("example"), &format!("other-{tag}")), None);
        }
        Ok(())
    }
}
