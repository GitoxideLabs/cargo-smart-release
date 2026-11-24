use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::bail;
use cargo_metadata::Package;

use super::Options;
use crate::utils::will;

pub(in crate::command::release_impl) fn publish_crate(
    publishee: &Package,
    manifest_path: &Path,
    staged: bool,
    experimental: bool,
    prevent_default_members: bool,
    dry_run_config: Option<&Path>,
    Options {
        skip_publish,
        dry_run,
        dry_run_cargo_publish,
        allow_dirty,
        no_verify,
        verbose,
        registry,
        target,
        publish_uses_docs_rs_metadata,
        ..
    }: Options,
) -> anyhow::Result<()> {
    if skip_publish {
        return Ok(());
    }
    let max_attempts = 3;
    let uses_cargo_dry_run = dry_run && dry_run_cargo_publish;
    let cargo_must_run = !dry_run || uses_cargo_dry_run;
    let docs_args = if !publish_uses_docs_rs_metadata {
        Vec::new()
    } else if staged {
        let manifest = cargo_toml::Manifest::from_path(manifest_path)?;
        let metadata = manifest.package.and_then(|package| package.metadata);
        docs_rs_metadata_publish_args_from_value(&publishee.name, &serde_json::to_value(metadata)?)?
    } else {
        docs_rs_metadata_publish_args_from_value(&publishee.name, &publishee.metadata)?
    };
    for attempt in 1..=max_attempts {
        let mut c = Command::new("cargo");
        c.arg("publish");

        if let Some(ref registry) = registry {
            c.arg("--registry").arg(registry);
        }
        if let Some(ref target) = target {
            c.arg("--target").arg(target);
        }
        c.args(&docs_args);
        if experimental && docs_args.iter().any(|arg| arg == "--no-default-features") {
            c.args(["--features", "unstable"]);
        }

        if allow_dirty {
            c.arg("--allow-dirty");
        }
        if no_verify {
            c.arg("--no-verify");
        }
        if uses_cargo_dry_run {
            c.arg("--dry-run");
            if let Some(config) = dry_run_config {
                c.arg("--config").arg(config);
            }
        }
        c.arg("--manifest-path").arg(manifest_path);
        if prevent_default_members {
            c.arg("--package").arg(publishee.name.as_str());
        }
        if verbose {
            log::trace!("{} run {:?}", will(!cargo_must_run), c);
        }
        if !cargo_must_run || c.status()?.success() {
            break;
        } else if attempt == max_attempts || dry_run {
            bail!("Could not successfully execute 'cargo publish'.")
        } else {
            log::warn!(
                "'cargo publish' run {attempt} failed but we retry up to {max_attempts} times to rule out flakiness"
            );
        }
    }
    Ok(())
}

pub(super) fn dry_run_patches<'a>(
    staging: &Path,
    staged: impl Iterator<Item = (&'a Package, &'a Path)>,
) -> anyhow::Result<Option<PathBuf>> {
    use anyhow::Context as _;
    use toml_edit::{value, DocumentMut, InlineTable, Table};

    let mut patches = Table::new();
    for (index, (package, manifest)) in staged.enumerate() {
        let directory = manifest.parent().context("Staged manifest has no parent")?;
        let mut entry = InlineTable::new();
        entry.insert("package", package.name.as_str().into());
        entry.insert(
            "path",
            directory.to_str().context("Staged package path must be UTF-8")?.into(),
        );
        patches.insert(&format!("release-{index}"), value(entry));
    }
    if patches.is_empty() {
        return Ok(None);
    }
    let mut config = DocumentMut::new();
    config["patch"]["crates-io"] = toml_edit::Item::Table(patches);
    let path = staging.join("dry-run.toml");
    std::fs::write(&path, config.to_string())?;
    Ok(Some(path))
}

pub fn refresh_lock_file() -> anyhow::Result<()> {
    cargo_metadata::MetadataCommand::new().exec()?;
    Ok(())
}

pub(super) fn default_registry() -> anyhow::Result<Option<String>> {
    use anyhow::Context as _;

    if let Some(registry) = std::env::var_os("CARGO_REGISTRY_DEFAULT") {
        return Ok(Some(
            registry
                .into_string()
                .map_err(|_| anyhow::anyhow!("CARGO_REGISTRY_DEFAULT must be UTF-8"))?,
        ));
    }
    let cwd = std::env::current_dir()?;
    let cargo_home = std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| gix::path::env::home_dir().map(|home| home.join(".cargo")));
    for dir in cwd.ancestors().map(|dir| dir.join(".cargo")).chain(cargo_home) {
        // Cargo prefers the extensionless file when both names exist.
        for name in ["config", "config.toml"] {
            let path = dir.join(name);
            let source = match std::fs::read_to_string(&path) {
                Ok(source) => source,
                Err(err) if err.kind() == ErrorKind::NotFound => continue,
                Err(err) => return Err(err).with_context(|| format!("Could not read {}", path.display())),
            };
            let config = source
                .parse::<toml_edit::DocumentMut>()
                .with_context(|| format!("Invalid Cargo config {}", path.display()))?;
            if let Some(registry) = config.get("registry").and_then(|registry| registry.get("default")) {
                return Ok(Some(
                    registry
                        .as_str()
                        .with_context(|| format!("registry.default in {} must be a string", path.display()))?
                        .to_owned(),
                ));
            }
            break;
        }
    }
    Ok(None)
}

fn docs_rs_metadata_publish_args_from_value(
    crate_name: &str,
    metadata: &serde_json::Value,
) -> anyhow::Result<Vec<String>> {
    let Some(docs_rs) = metadata.get("docs").and_then(|docs| docs.get("rs")) else {
        return Ok(Vec::new());
    };
    let docs_rs = match docs_rs.as_object() {
        Some(docs_rs) => docs_rs,
        None => {
            anyhow::bail!(
                "Crate '{}' has invalid package.metadata.docs.rs: expected a table",
                crate_name
            )
        }
    };

    let mut args = Vec::new();
    if let Some(features) = docs_rs.get("features") {
        let features = match features.as_array() {
            Some(features) => features,
            None => {
                anyhow::bail!(
                    "Crate '{}' has invalid package.metadata.docs.rs.features: expected an array of strings",
                    crate_name
                )
            }
        };
        let features = features
            .iter()
            .map(|value| {
                value.as_str().map(str::to_owned).ok_or_else(|| {
                    anyhow::anyhow!(
                        "Crate '{}' has invalid package.metadata.docs.rs.features: expected an array of strings",
                        crate_name
                    )
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        if !features.is_empty() {
            args.push("--features".into());
            args.push(features.join(","));
        }
    }
    if docs_rs
        .get("all-features")
        .map(|value| {
            value.as_bool().ok_or_else(|| {
                anyhow::anyhow!(
                    "Crate '{}' has invalid package.metadata.docs.rs.all-features: expected a boolean",
                    crate_name
                )
            })
        })
        .transpose()?
        .unwrap_or(false)
    {
        args.push("--all-features".into());
    }
    if docs_rs
        .get("no-default-features")
        .map(|value| {
            value.as_bool().ok_or_else(|| {
                anyhow::anyhow!(
                    "Crate '{}' has invalid package.metadata.docs.rs.no-default-features: expected a boolean",
                    crate_name
                )
            })
        })
        .transpose()?
        .unwrap_or(false)
    {
        args.push("--no-default-features".into());
    }

    Ok(args)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::docs_rs_metadata_publish_args_from_value;

    #[test]
    fn docs_rs_metadata_is_ignored_if_absent() {
        assert_eq!(
            docs_rs_metadata_publish_args_from_value("crate", &json!({})).unwrap(),
            Vec::<String>::new()
        );
    }

    #[test]
    fn docs_rs_metadata_is_translated_to_publish_args() {
        assert_eq!(
            docs_rs_metadata_publish_args_from_value(
                "crate",
                &json!({
                    "docs": {
                        "rs": {
                            "features": ["feat-a", "feat-b"],
                            "all-features": true,
                            "no-default-features": true
                        }
                    }
                })
            )
            .unwrap(),
            vec![
                "--features".to_owned(),
                "feat-a,feat-b".to_owned(),
                "--all-features".to_owned(),
                "--no-default-features".to_owned()
            ]
        );
    }

    #[test]
    fn empty_feature_lists_do_not_emit_features_flag() {
        assert_eq!(
            docs_rs_metadata_publish_args_from_value(
                "crate",
                &json!({
                    "docs": {
                        "rs": {
                            "features": [],
                            "all-features": false
                        }
                    }
                })
            )
            .unwrap(),
            Vec::<String>::new()
        );
    }

    #[test]
    fn invalid_docs_rs_table_type_is_reported() {
        let err = docs_rs_metadata_publish_args_from_value("crate", &json!({ "docs": { "rs": [] } })).unwrap_err();
        assert!(err
            .to_string()
            .contains("Crate 'crate' has invalid package.metadata.docs.rs: expected a table"));
    }

    #[test]
    fn invalid_features_type_is_reported() {
        let err =
            docs_rs_metadata_publish_args_from_value("crate", &json!({ "docs": { "rs": { "features": "feat-a" } } }))
                .unwrap_err();
        assert!(err
            .to_string()
            .contains("Crate 'crate' has invalid package.metadata.docs.rs.features: expected an array of strings"));
    }

    #[test]
    fn invalid_boolean_type_is_reported() {
        let err =
            docs_rs_metadata_publish_args_from_value("crate", &json!({ "docs": { "rs": { "all-features": "yes" } } }))
                .unwrap_err();
        assert!(err
            .to_string()
            .contains("Crate 'crate' has invalid package.metadata.docs.rs.all-features: expected a boolean"));
    }
}
