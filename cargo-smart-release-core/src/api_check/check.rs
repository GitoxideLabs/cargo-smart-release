//! Audit and compare supported APIs, including their public dependency identities.
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

use anyhow::{anyhow, Context, Result};
use serde_json::Value;

use super::{Audit, Baseline, FeatureProfile, Incompatible, Reason, ReleaseRequest};

/// Check the manifest's supported feature profiles and public dependency policy.
/// Returns all detected violations, or `None` when every supported profile passes.
///
/// An item is experimental if its rustdoc text contains a line exactly equal to `**Unstable.**`
/// or starting with `**Unstable:**`, for example `**Unstable:** Since 2026-10-13.`.
/// Matching is case-sensitive and does not trim whitespace. These are checker conventions,
/// not Rust feature gates or compiler stability attributes.
///
/// Experimental items are excluded from the supported API, but supported APIs that expose
/// experimental types are reported as boundary violations. With recursive checking enabled,
/// the same rules apply to reachable workspace dependency members.
pub fn boundary(manifest: &Path) -> Result<Option<Incompatible>> {
    let manifest = manifest.canonicalize()?;
    let metadata = cargo_metadata(&manifest, &[])?;
    let package = metadata["packages"]
        .as_array()
        .context("metadata has no packages")?
        .iter()
        .find(|package| package["manifest_path"] == manifest.to_string_lossy().as_ref())
        .context("manifest package missing from metadata")?;
    let name = required_str(package, "name")?;
    let unstable = package["metadata"]["smart-release"]
        .get("unstable-features")
        .map_or_else(|| Ok(Vec::new()), string_array)?;
    let output = tempfile::tempdir()?;
    let mut incompatible = Incompatible {
        package: name.to_owned(),
        manifest_path: manifest.clone(),
        baseline: None,
        issues: Vec::new(),
    };
    for (index, profile) in profiles(package)?.iter().enumerate() {
        let features = feature_args(profile, &package["features"], &unstable)?;
        let resolved = cargo_metadata(&manifest, &features)?;
        let document = rustdoc_document(
            &manifest,
            &features,
            &resolved,
            name,
            &output.path().join(format!("{index}.json")),
        )?;
        incompatible.record(
            profile,
            boundary_graph(&document, package, &resolved, &features, output.path())?.reasons,
        );
    }
    Ok(incompatible.into_option())
}

/// Read public dependencies from rustdoc JSON and check an optional crate allowlist.
/// Returns every dependency path and all disallowed dependency names without printing them.
///
/// Unlike [`boundary`], this audits the full public API, including items whose docs contain
/// the `**Unstable.**` or `**Unstable:**` experimental markers.
pub fn audit(path: &Path, allowed: Option<&BTreeSet<String>>) -> Result<Audit> {
    let document = serde_json::from_reader(fs::File::open(path)?)?;
    let dependencies = public_dependencies(&document)?;
    let disallowed = dependencies
        .keys()
        .filter(|name| allowed.is_some_and(|allowed| !allowed.contains(*name)))
        .cloned()
        .collect();
    Ok(Audit {
        dependencies,
        disallowed,
    })
}

/// Compare a smart-release JSON request against its baseline and planned dependency updates.
/// Returns all detected incompatibilities, or `None` when the check passes.
///
/// Uses the experimental documentation markers and matching rules described in [`boundary`].
/// Marked items are excluded from supported API comparisons, including reachable workspace
/// dependency members when recursive checking is enabled. Supported APIs that expose
/// experimental types are still reported as boundary violations.
pub fn release(request: &ReleaseRequest, baseline_override: Option<&Path>) -> Result<Option<Incompatible>> {
    let baseline_override = baseline_override.map(Path::canonicalize).transpose()?;
    let package = request.package.as_str();
    let manifest = request.manifest_path.canonicalize()?;
    let metadata = cargo_metadata(&manifest, &[])?;
    let package_metadata = find_package(&metadata, package, &manifest)?;
    let root = Path::new(required_str(&metadata, "workspace_root")?);
    let mut supported_profiles = profiles(package_metadata)?;
    let unstable = &request.unstable_features;
    let dependencies = &request.dependencies;
    let baseline_dir = tempfile::tempdir()?;
    let baseline_root = match baseline_override.as_deref() {
        Some(path) => path.to_path_buf(),
        None => archive_baseline(root, &request.baseline_revision, baseline_dir.path())?,
    };
    let baseline_manifest = baseline_root.join(manifest.strip_prefix(root)?);
    let baseline_metadata = cargo_metadata(&baseline_manifest, &[])?;
    let baseline_package = find_package(&baseline_metadata, package, &baseline_manifest)?;
    let mut incompatible = Incompatible {
        package: package.to_owned(),
        manifest_path: manifest.clone(),
        baseline: Some(match baseline_override {
            Some(path) => Baseline::Checkout(path),
            None => Baseline::Revision(request.baseline_revision.clone()),
        }),
        issues: Vec::new(),
    };
    if !baseline_package["metadata"]["api-check"].is_null() {
        for profile in profiles(baseline_package)? {
            if !supported_profiles.iter().any(|current| same_profile(&profile, current)) {
                tracing::warn!("Supported feature profile removed: {profile:?}");
                incompatible.record(&profile, [Reason::ProfileRemoved]);
                supported_profiles.push(profile.clone());
            }
        }
    }
    for (profile_index, profile) in supported_profiles.iter().enumerate() {
        let feature_args = feature_args(profile, &package_metadata["features"], unstable)?;
        tracing::info!("Checking {package}, features: {feature_args:?}");
        let current_metadata = cargo_metadata(&manifest, &feature_args)?;
        let current_json = baseline_dir.path().join(format!("current-{profile_index}.json"));
        let baseline_json = baseline_dir.path().join(format!("baseline-{profile_index}.json"));
        let document = rustdoc_document(&manifest, &feature_args, &current_metadata, package, &current_json)?;
        let current_graph = boundary_graph(
            &document,
            package_metadata,
            &current_metadata,
            &feature_args,
            baseline_dir.path(),
        )?;
        incompatible.record(profile, current_graph.reasons);
        if !baseline_package["metadata"]["api-check"].is_null()
            && !profiles(baseline_package)?
                .iter()
                .any(|baseline| same_profile(baseline, profile))
        {
            // A newly supported configuration has no prior compatibility promise, but must pass the boundary audit.
            continue;
        }
        let baseline_metadata = cargo_metadata(&baseline_manifest, &feature_args)?;
        let before = rustdoc_document(
            &baseline_manifest,
            &feature_args,
            &baseline_metadata,
            package,
            &baseline_json,
        )?;
        let baseline_graph = boundary_graph(
            &before,
            baseline_package,
            &baseline_metadata,
            &feature_args,
            baseline_dir.path(),
        )?;
        let mut exposed = baseline_graph.dependencies.clone();
        for (name, paths) in &current_graph.dependencies {
            exposed.entry(name.clone()).or_default().extend(paths.clone());
        }
        tracing::info!(
            "Public dependency crates: {}",
            exposed.keys().cloned().collect::<Vec<_>>().join(", ")
        );
        for dependency in dependencies {
            let name = dependency.name.as_str();
            let Some(crate_name) = planned_dependency_crate(&current_metadata, name)? else {
                continue;
            };
            if dependency.breaking {
                let Some(paths) = exposed.get(crate_name) else { continue };
                tracing::warn!(
                    "Breaking public dependency {name}: {}",
                    paths.iter().cloned().collect::<Vec<_>>().join(", ")
                );
                incompatible.record(
                    profile,
                    [Reason::BreakingDependency {
                        dependency: dependency.clone(),
                        crate_name: crate_name.to_owned(),
                        paths: paths.clone(),
                    }],
                );
            }
        }
        incompatible.record(
            profile,
            identity_changes(&exposed, &baseline_metadata, &current_metadata)?,
        );
        incompatible.record(
            profile,
            signature_changes(&baseline_graph.signatures, &current_graph.signatures),
        );
        for (name, (before, supported)) in &baseline_graph.documents {
            let Some((after, current_supported)) = current_graph.documents.get(name) else {
                tracing::warn!("Supported dependency API disappeared: {name}");
                incompatible.record(
                    profile,
                    [Reason::SupportedDependencyRemoved {
                        crate_name: name.clone(),
                    }],
                );
                continue;
            };
            fs::write(
                &baseline_json,
                serde_json::to_vec(&supported_document(before, supported)?)?,
            )?;
            fs::write(
                &current_json,
                serde_json::to_vec(&supported_document(after, current_supported)?)?,
            )?;
            tracing::info!("Comparing supported API of {name}");
            if let Some(output) = semver_check(&baseline_json, &current_json)? {
                incompatible.record(
                    profile,
                    [Reason::SemverCheck {
                        crate_name: name.clone(),
                        output,
                    }],
                );
            }
        }
    }
    Ok(incompatible.into_option())
}

fn profiles(package: &Value) -> Result<Vec<FeatureProfile>> {
    let profiles = package["metadata"]["api-check"]["profiles"]
        .as_array()
        .filter(|profiles| !profiles.is_empty())
        .context("package.metadata.api-check.profiles must describe the supported stable feature profiles")?;
    serde_json::from_value(Value::Array(profiles.clone())).context("Invalid package.metadata.api-check.profiles")
}

fn same_profile(before: &FeatureProfile, after: &FeatureProfile) -> bool {
    before.default_features == after.default_features
        && before.features.iter().collect::<BTreeSet<_>>() == after.features.iter().collect::<BTreeSet<_>>()
}

struct SupportedGraph {
    reasons: Vec<Reason>,
    dependencies: BTreeMap<String, BTreeSet<String>>,
    signatures: BTreeMap<String, Value>,
    documents: BTreeMap<String, (Value, BTreeSet<String>)>,
}

fn semver_check(before: &Path, after: &Path) -> Result<Option<Output>> {
    let output = Command::new("cargo")
        .args(["semver-checks", "check-release", "--current-rustdoc"])
        .arg(after)
        .arg("--baseline-rustdoc")
        .arg(before)
        .args(["--release-type", "minor", "--color", "never"])
        .output()?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    match output.status.code() {
        Some(0) => Ok(None),
        // cargo-semver-checks 0.51+ distinguishes detected breaks from tool failures.
        Some(100) => Ok(Some(output)),
        _ => Err(anyhow!(
            "cargo-semver-checks failed without a completed compatibility verdict: {}",
            output.status
        )),
    }
}

fn supported_document(document: &Value, supported: &BTreeSet<String>) -> Result<Value> {
    let mut kept = supported.clone();
    // Preserve export paths through modules and re-exports, but only to collected definitions.
    kept.insert(item_key(&document["root"])?);
    loop {
        let mut changed = false;
        for (id, item) in document["index"].as_object().context("rustdoc has no index")? {
            let target = &item["inner"]["use"]["id"];
            let contains_supported = item["inner"]["module"]["items"].as_array().is_some_and(|children| {
                children
                    .iter()
                    .any(|child| item_key(child).is_ok_and(|id| kept.contains(&id)))
            });
            if !unstable(item) && (contains_supported || (!target.is_null() && kept.contains(&item_key(target)?))) {
                changed |= kept.insert(id.clone());
            }
        }
        if !changed {
            break;
        }
    }
    let mut filtered = document.clone();
    let index = filtered["index"].as_object_mut().context("rustdoc has no index")?;
    // Removed children must also leave the index: semver analysis requires variants and methods to have owners.
    index.retain(|id, item| item["crate_id"] != 0 || kept.contains(id));
    for item in index.values_mut().filter(|item| item["crate_id"] == 0) {
        filter_children(&mut item["inner"], &kept)?;
    }
    Ok(filtered)
}

fn filter_children(value: &mut Value, kept: &BTreeSet<String>) -> Result<()> {
    match value {
        Value::Object(fields) => {
            let mut stripped = false;
            for (key, value) in fields.iter_mut() {
                if let Some(children) = value.as_array_mut().filter(|_| {
                    matches!(
                        key.as_str(),
                        "items" | "impls" | "implementations" | "variants" | "fields" | "tuple"
                    )
                }) {
                    let mut selected = Vec::new();
                    for child in children.iter() {
                        if child.is_object() {
                            let mut child = child.clone();
                            filter_children(&mut child, kept)?;
                            selected.push(child);
                        } else if child.is_null() || kept.contains(&item_key(child)?) {
                            selected.push(child.clone());
                        } else if key == "tuple" {
                            selected.push(Value::Null);
                        } else {
                            stripped = true;
                        }
                    }
                    *children = selected;
                } else {
                    filter_children(value, kept)?;
                }
            }
            if stripped && fields.contains_key("has_stripped_fields") {
                fields.insert("has_stripped_fields".into(), Value::Bool(true));
            }
        }
        Value::Array(values) => {
            for value in values {
                filter_children(value, kept)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn boundary_graph(
    document: &Value,
    package: &Value,
    metadata: &Value,
    features: &[String],
    output: &Path,
) -> Result<SupportedGraph> {
    // An isolated consumer excludes workspace and dev-only feature unification from the library contract.
    let consumer = tempfile::tempdir()?;
    fs::copy(
        Path::new(required_str(metadata, "workspace_root")?).join("Cargo.lock"),
        consumer.path().join("Cargo.lock"),
    )?;
    fs::create_dir(consumer.path().join("src"))?;
    fs::write(consumer.path().join("src/lib.rs"), "")?;
    let selected = features
        .windows(2)
        .find(|args| args[0] == "--features")
        .map(|args| args[1].split(',').collect::<Vec<_>>())
        .unwrap_or_default();
    let manifest = consumer.path().join("Cargo.toml");
    let dependency_dir = Path::new(required_str(package, "manifest_path")?)
        .parent()
        .context("package manifest has no parent")?;
    fs::write(
        &manifest,
        format!(
            "[package]\nname = \"api-check-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n[dependencies]\napi = {{ package = {}, path = {}, default-features = {}, features = {} }}\n",
            serde_json::to_string(required_str(package, "name")?)?,
            serde_json::to_string(&dependency_dir.to_string_lossy())?,
            !features.iter().any(|arg| arg == "--no-default-features"),
            serde_json::to_string(&selected)?,
        ),
    )?;
    // Only this temporary copied lock may change to incorporate the isolated consumer.
    let mut resolved = serde_json::to_value(cargo_metadata::MetadataCommand::new().manifest_path(&manifest).exec()?)?;
    resolved["workspace_members"] = metadata["workspace_members"].clone();
    resolved["target_directory"] = metadata["target_directory"].clone();
    supported_graph(document, &package["metadata"]["api-check"], &resolved, |dependency| {
        let node = resolved["resolve"]["nodes"]
            .as_array()
            .context("metadata has no resolved nodes")?
            .iter()
            .find(|node| node["id"] == dependency["id"])
            .context("public dependency has no resolved features")?;
        let features = string_array(&node["features"])?;
        let mut args = vec!["--no-default-features".into()];
        if !features.is_empty() {
            args.extend(["--features".into(), features.join(",")]);
        }
        let name = required_str(dependency, "name")?;
        rustdoc_document(
            Path::new(required_str(dependency, "manifest_path")?),
            &args,
            &resolved,
            name,
            &output.join(format!("dependency-{}.json", library_name(dependency)?)),
        )
    })
}

fn supported_graph(
    document: &Value,
    policy: &Value,
    metadata: &Value,
    mut load: impl FnMut(&Value) -> Result<Value>,
) -> Result<SupportedGraph> {
    let read_list = |key: &str| -> Result<Vec<String>> { policy.get(key).map_or_else(|| Ok(Vec::new()), string_array) };
    let allowed_types = read_list("allowed-unstable-types")?;
    let allowed_crates = read_list("allowed-unstable-crates")?;
    let recursive = policy.get("recursive").map_or(Ok(false), |value| {
        value.as_bool().context("api-check.recursive must be a boolean")
    })?;
    let root = walk_supported(document, &[document["root"].clone()])?;
    let mut graph = SupportedGraph {
        reasons: root
            .unsupported
            .into_iter()
            .map(|path| Reason::ExperimentalType { path })
            .collect(),
        dependencies: root.dependencies.clone(),
        signatures: item_signatures(document, &root.seen)?,
        documents: BTreeMap::from([(
            required_str(&document["index"][item_key(&document["root"])?], "name")?.to_owned(),
            (document.clone(), root.seen.clone()),
        )]),
    };
    let mut pending: Vec<_> = root.dependencies.into_iter().collect();
    let mut documents = BTreeMap::new();
    let mut seen = BTreeSet::new();
    while let Some((name, paths)) = pending.pop() {
        let Some(dependency) = workspace_library(metadata, &name)? else {
            // shortcut: third-party members are not expanded; extend loading if workspace-only plumbing becomes insufficient.
            continue;
        };
        if !documents.contains_key(&name) {
            documents.insert(name.clone(), load(dependency)?);
        }
        let document = &documents[&name];
        let mut names = BTreeMap::new();
        public_names(document, &document["root"], String::new(), None, &mut names)?;
        let mut allowed_ids = BTreeSet::new();
        for path in allowed_types
            .iter()
            .filter(|path| path.starts_with(&format!("{name}::")))
        {
            allowed_ids.insert(resolve_item(document, &names, path)?);
        }
        let provisional = required_str(dependency, "version")?.starts_with("0.");
        let blanket = allowed_crates.contains(&name);
        let supported = if provisional {
            None
        } else {
            Some(walk_supported(document, &[document["root"].clone()])?.seen)
        };
        for path in paths {
            let id = resolve_item(document, &names, &path)?;
            if !seen.insert((name.clone(), id.clone())) {
                continue;
            }
            let item = &document["index"][&id];
            if unstable(item)
                || (provisional && !blanket && !allowed_ids.contains(&id))
                || supported.as_ref().is_some_and(|items| !items.contains(&id))
            {
                tracing::warn!("Stable API exposes experimental plumbing item {path}");
                let crate_name = name.clone();
                graph.reasons.push(if unstable(item) {
                    Reason::ExperimentalDependencyItem { crate_name, path }
                } else if provisional && !blanket && !allowed_ids.contains(&id) {
                    Reason::UnlistedDependencyItem { crate_name, path }
                } else {
                    Reason::UnsupportedDependencyItem { crate_name, path }
                });
                continue;
            }
            if recursive {
                let walk = walk_supported(document, &[Value::String(id)])?;
                graph.reasons.extend(
                    walk.unsupported
                        .into_iter()
                        .map(|path| Reason::ExperimentalType { path }),
                );
                if provisional && !blanket {
                    for id in &walk.seen {
                        let item = &document["index"][id];
                        if item["crate_id"] == 0 && type_definition(item) && !allowed_ids.contains(id) {
                            let path = names.get(id).cloned().unwrap_or_else(|| item_path(document, id));
                            tracing::warn!("Supported member of {name} exposes experimental plumbing type {path}");
                            graph.reasons.push(Reason::UnlistedDependencyType {
                                crate_name: name.clone(),
                                path,
                            });
                        }
                    }
                }
                graph.signatures.extend(item_signatures(document, &walk.seen)?);
                graph
                    .documents
                    .entry(name.clone())
                    .or_insert_with(|| (document.clone(), BTreeSet::new()))
                    .1
                    .extend(walk.seen);
                for (crate_name, paths) in &walk.dependencies {
                    graph
                        .dependencies
                        .entry(crate_name.clone())
                        .or_default()
                        .extend(paths.clone());
                }
                pending.extend(walk.dependencies);
            }
        }
    }
    Ok(graph)
}

fn workspace_library<'a>(metadata: &'a Value, name: &str) -> Result<Option<&'a Value>> {
    let members = metadata["workspace_members"]
        .as_array()
        .context("metadata has no workspace members")?;
    let mut matches = metadata["packages"]
        .as_array()
        .context("metadata has no packages")?
        .iter()
        .filter(|package| {
            members.contains(&package["id"]) && library_name(package).is_ok_and(|library| library == name)
        });
    let result = matches.next();
    if matches.next().is_some() {
        return Err(anyhow!("ambiguous workspace library {name}"));
    }
    Ok(result)
}

fn resolve_item(document: &Value, names: &BTreeMap<String, String>, path: &str) -> Result<String> {
    if let Some((id, _)) = names.iter().find(|(_, name)| name.as_str() == path) {
        return Ok(id.clone());
    }
    document["paths"]
        .as_object()
        .context("rustdoc has no paths")?
        .iter()
        .find(|(id, summary)| {
            summary["crate_id"] == 0 && !document["index"][*id].is_null() && item_path(document, id) == path
        })
        .map(|(id, _)| id.clone())
        .with_context(|| format!("cannot resolve plumbing item {path} in dependency rustdoc"))
}

fn item_path(document: &Value, id: &str) -> String {
    document["paths"][id]["path"].as_array().map_or_else(
        || id.to_owned(),
        |parts| parts.iter().filter_map(Value::as_str).collect::<Vec<_>>().join("::"),
    )
}

/// Return whether an item's rustdoc text marks it as experimental.
///
/// Recognizes a line exactly equal to `**Unstable.**` or starting with `**Unstable:**`.
/// Matching is case-sensitive and does not trim whitespace: `**Unstable.** More text`
/// does not match, while `**Unstable:** More text` does. Missing or non-string docs
/// return `false`. This is a checker convention, not a Rust stability attribute.
fn unstable(item: &Value) -> bool {
    item["docs"].as_str().is_some_and(|docs| {
        docs.lines()
            .any(|line| line.starts_with("**Unstable:**") || line == "**Unstable.**")
    })
}

fn type_definition(item: &Value) -> bool {
    item["inner"].as_object().is_some_and(|inner| {
        inner.keys().any(|kind| {
            matches!(
                kind.as_str(),
                "struct" | "enum" | "union" | "trait" | "trait_alias" | "type_alias" | "extern_type"
            )
        })
    })
}

fn rustdoc_document(
    manifest: &Path,
    features: &[String],
    metadata: &Value,
    package: &str,
    output: &Path,
) -> Result<Value> {
    let rustc = Command::new("rustc").arg("-vV").output()?;
    if !rustc.status.success() {
        return Err(anyhow!("could not determine the rustc host target"));
    }
    let version = String::from_utf8(rustc.stdout)?;
    let host = version
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .context("rustc -vV has no host target")?;
    let path = Path::new(required_str(metadata, "target_directory")?)
        .join(host)
        .join("doc")
        .join(format!(
            "{}.json",
            library_name(find_package(metadata, package, manifest)?)?
        ));
    // Discard only this generated JSON, so a failed or redirected rustdoc invocation cannot reuse a stale API.
    match fs::remove_file(&path) {
        Ok(()) => {}
        Err(err) if err.kind() == io::ErrorKind::NotFound => {}
        Err(err) => return Err(err.into()),
    }
    let mut rustdoc = Command::new("cargo");
    rustdoc
        .env("RUSTC_BOOTSTRAP", "1")
        .args(["rustdoc", "--locked", "--lib", "--manifest-path"])
        .arg(manifest)
        .args(["--target", host])
        .args(features)
        .args([
            "--",
            "-D",
            "warnings",
            "-Z",
            "unstable-options",
            "--output-format",
            "json",
        ]);
    successful(rustdoc)?;
    fs::copy(path, output)?;
    Ok(serde_json::from_reader(fs::File::open(output)?)?)
}

fn required_str<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value[key]
        .as_str()
        .with_context(|| format!("missing string field {key}"))
}

fn string_array(value: &Value) -> Result<Vec<String>> {
    value
        .as_array()
        .context("expected an array of strings")?
        .iter()
        .map(|value| value.as_str().map(str::to_owned).context("expected a string"))
        .collect()
}

fn feature_args(profile: &FeatureProfile, features: &Value, unstable: &[String]) -> Result<Vec<String>> {
    let defaults = profile.default_features;
    let selected = &profile.features;
    let mut pending = selected.clone();
    if defaults {
        pending.push("default".into());
    }
    let mut seen = BTreeSet::new();
    while let Some(feature) = pending.pop() {
        if !seen.insert(feature.clone()) {
            continue;
        }
        if unstable.contains(&feature) {
            return Err(anyhow!("stable API profile enables excluded feature {feature}"));
        }
        if let Some(values) = features[&feature].as_array() {
            pending.extend(
                values
                    .iter()
                    .filter_map(Value::as_str)
                    .filter(|value| features.get(value).is_some())
                    .map(str::to_owned),
            );
        } else if feature != "default" {
            return Err(anyhow!("unknown stable profile feature {feature}"));
        }
    }
    let mut args = Vec::new();
    if !defaults {
        args.push("--no-default-features".into());
    }
    if !selected.is_empty() {
        args.extend(["--features".into(), selected.join(",")]);
    }
    Ok(args)
}

fn cargo_metadata(manifest: &Path, features: &[String]) -> Result<Value> {
    Ok(serde_json::to_value(super::metadata(Some(manifest), features)?)?)
}

fn find_package<'a>(metadata: &'a Value, name: &str, manifest: &Path) -> Result<&'a Value> {
    let manifest = manifest.canonicalize()?;
    metadata["packages"]
        .as_array()
        .context("metadata has no packages")?
        .iter()
        .find(|package| {
            package["name"] == name
                && package["manifest_path"]
                    .as_str()
                    .is_some_and(|path| Path::new(path).canonicalize().is_ok_and(|path| path == manifest))
        })
        .with_context(|| format!("package {name} missing from metadata"))
}

fn library_name(package: &Value) -> Result<&str> {
    package["targets"]
        .as_array()
        .context("package has no targets")?
        .iter()
        .find(|target| is_library(target))
        .and_then(|target| target["name"].as_str())
        .context("package has no library target")
}

fn is_library(target: &Value) -> bool {
    target["kind"].as_array().is_some_and(|kinds| {
        kinds.iter().any(|kind| {
            matches!(
                kind.as_str(),
                Some("lib" | "rlib" | "dylib" | "staticlib" | "cdylib" | "proc-macro")
            )
        })
    })
}

fn planned_dependency_crate<'a>(metadata: &'a Value, name: &str) -> Result<Option<&'a str>> {
    let members = metadata["workspace_members"]
        .as_array()
        .context("metadata has no workspace members")?;
    let mut matches = metadata["packages"]
        .as_array()
        .context("metadata has no packages")?
        .iter()
        .filter(|package| package["name"] == name && members.contains(&package["id"]));
    let package = matches
        .next()
        .with_context(|| format!("planned dependency {name} is not a workspace member"))?;
    if matches.next().is_some() {
        return Err(anyhow!("planned dependency {name} is ambiguous"));
    }
    let targets = package["targets"].as_array().context("package has no targets")?;
    let libraries: Vec<_> = targets.iter().filter(|target| is_library(target)).collect();
    match libraries.as_slice() {
        [] => Ok(None),
        [library] => Ok(Some(required_str(library, "name")?)),
        _ => Err(anyhow!("planned dependency {name} has multiple library targets")),
    }
}

fn archive_baseline(root: &Path, revision: &str, destination: &Path) -> Result<PathBuf> {
    let repository_root = super::git_root(root)?;
    let mut archive = Command::new("git")
        .current_dir(&repository_root)
        .args(["archive", "--format=tar", "--", revision])
        .stdout(Stdio::piped())
        .spawn()?;
    let extracted = {
        let mut tar = tar::Archive::new(archive.stdout.take().context("git archive has no stdout")?);
        tar.unpack(destination).and_then(|()| {
            // Git may still be writing padding after the archive's end marker.
            io::copy(&mut tar.into_inner(), &mut io::sink()).map(|_| ())
        })
    };
    let archived = archive.wait()?;
    extracted.with_context(|| format!("could not extract baseline {revision}"))?;
    if !archived.success() {
        return Err(anyhow!(
            "could not extract baseline {revision}; the release tag must exist locally"
        ));
    }
    Ok(destination.join(root.strip_prefix(&repository_root)?))
}

fn successful(mut command: Command) -> Result<()> {
    let status = command.status()?;
    if !status.success() {
        return Err(anyhow!("{command:?} failed: {status}"));
    }
    Ok(())
}

fn compatibility_family(version: &str) -> Result<String> {
    let parts: Vec<_> = version.split(['.', '-', '+']).take(3).collect();
    if parts.len() != 3 || parts.iter().any(|part| part.parse::<u64>().is_err()) {
        return Err(anyhow!("invalid dependency version {version}"));
    }
    Ok(if parts[0] != "0" {
        parts[0].to_owned()
    } else if parts[1] != "0" {
        parts[..2].join(".")
    } else {
        parts.join(".")
    })
}

fn dependency_identities(metadata: &Value, crate_name: &str) -> Result<BTreeSet<String>> {
    let candidates: Vec<_> = metadata["packages"]
        .as_array()
        .context("metadata has no packages")?
        .iter()
        .filter(|package| library_name(package).is_ok_and(|name| name == crate_name))
        .collect();
    if candidates.len() > 1 {
        return Err(anyhow!(
            "public crate {crate_name} maps to multiple Cargo packages; resolve its precise origin before releasing"
        ));
    }
    candidates
        .into_iter()
        .map(|package| {
            let family = compatibility_family(required_str(package, "version")?)?;
            let source = match package["source"].as_str() {
                Some(source) => source.to_owned(),
                None => {
                    let manifest = Path::new(required_str(package, "manifest_path")?);
                    let root = Path::new(required_str(metadata, "workspace_root")?);
                    match manifest.strip_prefix(root) {
                        Ok(relative) => format!("workspace:{}", relative.display()),
                        Err(_) => format!("path:{}", manifest.display()),
                    }
                }
            };
            Ok(format!("{source}::{family}"))
        })
        .collect()
}

fn identity_changes(
    exposed: &BTreeMap<String, BTreeSet<String>>,
    baseline: &Value,
    current: &Value,
) -> Result<Vec<Reason>> {
    let mut changes = Vec::new();
    for name in exposed
        .keys()
        .filter(|name| !matches!(name.as_str(), "std" | "core" | "alloc"))
    {
        let before = dependency_identities(baseline, name)?;
        let after = dependency_identities(current, name)?;
        if after.is_empty() {
            return Err(anyhow!(
                "cannot identify public dependency crate {name} in Cargo metadata"
            ));
        }
        if !before.is_empty() && before != after {
            tracing::warn!("Public dependency identity changed for {name}: {before:?} -> {after:?}");
            changes.push(Reason::DependencyIdentityChanged {
                crate_name: name.clone(),
                paths: exposed[name].clone(),
                before,
                after,
            });
        }
    }
    Ok(changes)
}

#[cfg(test)]
fn unchanged_types(before: &Value, after: &Value) -> Result<bool> {
    Ok(signature_changes(&type_signatures(before)?, &type_signatures(after)?).is_empty())
}

fn signature_changes(before: &BTreeMap<String, Value>, after: &BTreeMap<String, Value>) -> Vec<Reason> {
    let mut changes = Vec::new();
    for (path, signature) in before {
        if after.get(path) != Some(signature) {
            tracing::warn!("Public type signature changed or disappeared: {path}");
            changes.push(Reason::SignatureChanged {
                path: path.clone(),
                before: signature.clone(),
                after: after.get(path).cloned(),
            });
        }
    }
    changes
}

#[cfg(test)]
fn type_signatures(document: &Value) -> Result<BTreeMap<String, Value>> {
    item_signatures(document, &walk_api(document)?.seen)
}

fn item_signatures(document: &Value, seen: &BTreeSet<String>) -> Result<BTreeMap<String, Value>> {
    let mut names = BTreeMap::new();
    public_names(document, &document["root"], String::new(), None, &mut names)?;
    for id in seen {
        let item = &document["index"][id];
        if item["crate_id"] == 0
            && !names.contains_key(id)
            && item["inner"].as_object().is_some_and(|inner| {
                inner
                    .keys()
                    .any(|kind| matches!(kind.as_str(), "struct" | "enum" | "union" | "trait" | "type_alias"))
            })
        {
            // A returned type can expose fields and methods even when its defining module is private.
            let mut path = string_array(&document["paths"][id]["path"])?;
            let name = path.pop().context("reachable type has no canonical path")?;
            public_names(
                document,
                &Value::String(id.clone()),
                path.join("::"),
                Some(&name),
                &mut names,
            )?;
        }
    }
    let mut signatures = BTreeMap::new();
    for id in seen {
        let item = &document["index"][id];
        if item["crate_id"].as_u64() != Some(0) {
            continue;
        }
        let inner = item["inner"].as_object().context("item has no definition")?;
        let (kind, definition) = inner.iter().next().context("item has no definition")?;
        if kind == "impl" {
            let target = normalized_type(&definition["for"], document, &names)?;
            let trait_type = normalized_type(&definition["trait"], document, &names)?;
            let path = format!("impl {trait_type} for {target}");
            let impl_signature = signature(kind, definition, document, &names)?;
            if !definition["trait"].is_null() {
                signatures.insert(path.clone(), impl_signature.clone());
            }
            for method in definition["items"].as_array().context("impl has no items")? {
                let method_id = item_key(method)?;
                if !seen.contains(&method_id) {
                    continue;
                }
                let method = &document["index"][method_id];
                let definition = method["inner"].as_object().context("impl item has no definition")?;
                let (kind, definition) = definition.iter().next().context("empty impl item")?;
                signatures.insert(
                    format!("{path}::{}", required_str(method, "name")?),
                    serde_json::json!({"impl": impl_signature, "item": signature(kind, definition, document, &names)?}),
                );
            }
        } else if let Some(path) = names
            .get(id)
            .filter(|_| !matches!(kind.as_str(), "module" | "use" | "macro" | "proc_macro"))
        {
            signatures.insert(path.clone(), signature(kind, definition, document, &names)?);
        }
    }
    Ok(signatures)
}

fn signature(kind: &str, definition: &Value, document: &Value, names: &BTreeMap<String, String>) -> Result<Value> {
    let mut value = definition.clone();
    let shape = definition["kind"].as_str().or_else(|| {
        definition["kind"]
            .as_object()
            .and_then(|value| value.keys().next().map(String::as_str))
    });
    if let Some(fields) = value.as_object_mut() {
        for member in [
            "items",
            "impls",
            "implementations",
            "variants",
            "fields",
            "kind",
            "has_body",
            "provided_trait_methods",
            "blanket_impl",
        ] {
            fields.remove(member);
        }
    }
    if let Some(inputs) = value.pointer_mut("/sig/inputs").filter(|_| kind == "function") {
        let types = inputs
            .as_array()
            .context("function inputs must be an array")?
            .iter()
            .map(|input| {
                input
                    .as_array()
                    .and_then(|input| input.get(1))
                    .cloned()
                    .context("unknown function input shape")
            })
            .collect::<Result<Vec<Value>>>()?;
        *inputs = Value::Array(types);
    }
    Ok(serde_json::json!({"kind": kind, "shape": shape, "type": normalized_type(&value, document, names)?}))
}

fn normalized_type(value: &Value, document: &Value, names: &BTreeMap<String, String>) -> Result<Value> {
    match value {
        Value::Object(fields) => {
            let mut normalized = serde_json::Map::new();
            for (key, value) in fields {
                if key == "id" {
                    let id = item_key(value)?;
                    let path = match names.get(&id) {
                        Some(path) if document["index"][&id]["crate_id"] == 0 => path.clone(),
                        _ => string_array(&document["paths"][&id]["path"])
                            .with_context(|| format!("cannot normalize type {id}"))?
                            .join("::"),
                    };
                    normalized.insert("resolved_identity".into(), Value::String(path));
                } else if key != "path" || !fields.contains_key("id") {
                    normalized.insert(key.clone(), normalized_type(value, document, names)?);
                }
            }
            Ok(Value::Object(normalized))
        }
        Value::Array(values) => values
            .iter()
            .map(|value| normalized_type(value, document, names))
            .collect::<Result<Vec<_>>>()
            .map(Value::Array),
        _ => Ok(value.clone()),
    }
}

fn item_key(id: &Value) -> Result<String> {
    match id {
        Value::Number(number) => Ok(number.to_string()),
        Value::String(value) => Ok(value.clone()),
        _ => Err(anyhow!("invalid rustdoc item ID")),
    }
}

fn public_names(
    document: &Value,
    id: &Value,
    parent: String,
    exported_name: Option<&str>,
    names: &mut BTreeMap<String, String>,
) -> Result<()> {
    let key = item_key(id)?;
    let item = &document["index"][&key];
    if item.is_null() || item["crate_id"] != 0 {
        return Ok(());
    }
    let inner = item["inner"].as_object().context("item has no definition")?;
    let (kind, definition) = inner.iter().next().context("empty definition")?;
    let name = match (exported_name, kind.as_str()) {
        (Some(name), _) => name,
        (None, "use") => required_str(definition, "name")?,
        _ => required_str(item, "name")?,
    };
    let path = if parent.is_empty() {
        name.to_owned()
    } else {
        format!("{parent}::{name}")
    };
    if kind == "use" {
        let target = &definition["id"];
        if !target.is_null() {
            let target_id = item_key(target)?;
            if document["index"][&target_id]["crate_id"] == 0 {
                // Traverse the definition using its exported name, even when its source module is private.
                public_names(document, target, parent, Some(name), names)?;
            }
        }
        return Ok(());
    }
    if let Some(previous) = names.get_mut(&key) {
        if path.len() < previous.len() {
            *previous = path;
        }
        return Ok(());
    }
    names.insert(key, path.clone());
    let children = match kind.as_str() {
        "module" | "trait" => definition.get("items"),
        "enum" => definition.get("variants"),
        "struct" => definition
            .pointer("/kind/plain/fields")
            .or_else(|| definition.pointer("/kind/tuple")),
        "union" => definition.get("fields"),
        "variant" => definition
            .pointer("/kind/struct/fields")
            .or_else(|| definition.pointer("/kind/tuple")),
        _ => None,
    };
    if let Some(children) = children {
        for child in children.as_array().context("named children must be an array")? {
            if child.is_null() {
                continue;
            }
            let child_id = item_key(child)?;
            let inherit = matches!(kind.as_str(), "enum" | "variant" | "trait");
            if inherit || document["index"][&child_id]["visibility"] == "public" {
                public_names(document, child, path.clone(), None, names)?;
            }
        }
    }
    Ok(())
}

fn public_dependencies(document: &Value) -> Result<BTreeMap<String, BTreeSet<String>>> {
    Ok(walk_api(document)?.dependencies)
}

fn walk_api(document: &Value) -> Result<ApiWalk<'_>> {
    walk_items(document, &[document["root"].clone()], false)
}

fn walk_supported<'a>(document: &'a Value, roots: &[Value]) -> Result<ApiWalk<'a>> {
    walk_items(document, roots, true)
}

fn walk_items<'a>(document: &'a Value, roots: &[Value], supported_only: bool) -> Result<ApiWalk<'a>> {
    if document["format_version"].as_u64() != Some(61) {
        return Err(anyhow!(
            "the API walker requires rustdoc JSON format 61; review schema changes before updating it"
        ));
    }
    if document["includes_private"].as_bool() != Some(false) {
        return Err(anyhow!("rustdoc JSON must omit private items"));
    }
    let mut walk = ApiWalk {
        document,
        seen: BTreeSet::new(),
        dependencies: BTreeMap::new(),
        supported_only,
        unsupported: BTreeSet::new(),
    };
    for root in roots {
        walk.visit(root, false)?;
    }
    Ok(walk)
}

struct ApiWalk<'a> {
    document: &'a Value,
    seen: BTreeSet<String>,
    dependencies: BTreeMap<String, BTreeSet<String>>,
    supported_only: bool,
    unsupported: BTreeSet<String>,
}

impl ApiWalk<'_> {
    fn visit(&mut self, id: &Value, public_only: bool) -> Result<()> {
        if id.is_null() {
            return Ok(());
        }
        let key = item_key(id)?;
        let item = &self.document["index"][&key];
        if public_only && !item.is_null() && item["visibility"] != "public" {
            return Ok(());
        }
        if self.supported_only && unstable(item) {
            if !public_only && type_definition(item) {
                let path = item_path(self.document, &key);
                tracing::warn!("Stable API refers to experimental type {path}");
                self.unsupported.insert(path);
            }
            return Ok(());
        }
        // Rustdoc gives instantiated blanket impls the local crate ID, so use their trait's origin.
        if !item["inner"]["impl"]["blanket_impl"].is_null() {
            let trait_id = item_key(&item["inner"]["impl"]["trait"]["id"])?;
            let origin = self.document["index"][&trait_id]["crate_id"]
                .as_u64()
                .or_else(|| self.document["paths"][&trait_id]["crate_id"].as_u64())
                .context("blanket trait has no crate identity")?;
            if origin != 0 {
                return Ok(());
            }
        }
        if !self.seen.insert(key.clone()) {
            return Ok(());
        }
        let summary = &self.document["paths"][&key];
        let crate_id = item["crate_id"]
            .as_u64()
            .or_else(|| summary["crate_id"].as_u64())
            .with_context(|| format!("rustdoc item {key} has no crate identity"))?;
        if crate_id != 0 {
            let name = required_str(&self.document["external_crates"][crate_id.to_string()], "name")?;
            let path = summary["path"].as_array().map_or_else(
                || format!("{name}::{}", item["name"].as_str().unwrap_or("impl")),
                |parts| parts.iter().filter_map(Value::as_str).collect::<Vec<_>>().join("::"),
            );
            self.dependencies.entry(name.to_owned()).or_default().insert(path);
            if item["inner"]["impl"].is_null() {
                return Ok(());
            }
        }
        let inner = item["inner"]
            .as_object()
            .with_context(|| format!("rustdoc item {key} has no definition"))?;
        if inner.len() != 1 {
            return Err(anyhow!("rustdoc item {key} has an unknown definition shape"));
        }
        let (kind, value) = inner.iter().next().context("empty item definition")?;
        match kind.as_str() {
            "module" => self.children(&value["items"], true),
            "impl" => {
                let mut signature = value.clone();
                signature.as_object_mut().context("invalid impl")?.remove("items");
                self.scan(&signature)?;
                self.children(&value["items"], value["trait"].is_null())
            }
            "struct" | "union" => {
                let mut definition = value.clone();
                // Ordinary rustdoc output strips private fields; verify visibility on any fields that remain.
                if let Some(fields) = definition.pointer_mut("/kind/plain/fields") {
                    self.children(fields, true)?;
                    *fields = Value::Array(Vec::new());
                }
                if let Some(fields) = definition.get_mut("fields") {
                    self.children(fields, true)?;
                    *fields = Value::Array(Vec::new());
                }
                self.scan(&definition)
            }
            "enum" | "variant" | "function" | "type_alias" | "trait" | "trait_alias" | "struct_field"
            | "assoc_type" | "assoc_const" | "constant" | "static" | "extern_type" | "use" | "macro" | "proc_macro"
            | "primitive" | "keyword" => self.scan(value),
            _ => Err(anyhow!(
                "unsupported rustdoc item kind {kind}; update the API walker before releasing"
            )),
        }
    }

    fn children(&mut self, ids: &Value, public_only: bool) -> Result<()> {
        for id in ids.as_array().context("rustdoc child IDs must be an array")? {
            self.visit(id, public_only)?;
        }
        Ok(())
    }

    fn scan(&mut self, value: &Value) -> Result<()> {
        match value {
            Value::Object(fields) => {
                for (key, value) in fields {
                    match key.as_str() {
                        "id" => self.visit(value, false)?,
                        "items" | "impls" | "implementations" | "variants" | "fields" | "tuple" if value.is_array() => {
                            for child in value.as_array().context("expected child IDs")? {
                                if child.is_number() || child.is_string() || child.is_null() {
                                    self.visit(child, false)?;
                                } else {
                                    self.scan(child)?;
                                }
                            }
                        }
                        _ => self.scan(value)?,
                    }
                }
            }
            Value::Array(values) => {
                for value in values {
                    self.scan(value)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn check_boundary(
        document: &Value,
        policy: &Value,
        metadata: &Value,
        load: impl FnMut(&Value) -> Result<Value>,
    ) -> Result<bool> {
        Ok(supported_graph(document, policy, metadata, load)?.reasons.is_empty())
    }

    #[test]
    fn baseline_archives_preserve_nested_paths_and_file_metadata() -> Result<()> {
        let repository = tempfile::tempdir()?;
        let root = repository.path().canonicalize()?;
        let workspace = root.join("nested");
        fs::create_dir_all(workspace.join("src"))?;
        let source = PathBuf::from("src").join(format!("{}.rs", "a".repeat(120)));
        let contents = vec![b'x'; 1024 * 1024];
        fs::write(workspace.join(&source), &contents)?;
        fs::write(root.join("root.txt"), "repository root")?;
        fs::write(workspace.join("tool"), "#!/bin/sh\nexit 0\n")?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::{symlink, PermissionsExt};
            fs::set_permissions(workspace.join("tool"), fs::Permissions::from_mode(0o755))?;
            symlink("../root.txt", workspace.join("root-link"))?;
        }
        for args in [
            vec!["init", "-q"],
            vec!["add", "."],
            vec![
                "-c",
                "commit.gpgSign=false",
                "-c",
                "user.name=API",
                "-c",
                "user.email=api@example.com",
                "commit",
                "-qm",
                "baseline",
            ],
        ] {
            let mut git = Command::new("git");
            git.current_dir(&root).args(args);
            successful(git)?;
        }
        fs::write(workspace.join(&source), "working tree change")?;
        let destination = tempfile::tempdir()?;
        let baseline = archive_baseline(&workspace, "HEAD", destination.path())?;
        assert_eq!(baseline, destination.path().join("nested"));
        assert_eq!(fs::read(baseline.join(source))?, contents);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_ne!(fs::metadata(baseline.join("tool"))?.permissions().mode() & 0o111, 0);
            assert_eq!(fs::read_link(baseline.join("root-link"))?, Path::new("../root.txt"));
            assert_eq!(fs::read_to_string(baseline.join("root-link"))?, "repository root");
        }
        assert!(archive_baseline(&workspace, "does-not-exist", destination.path()).is_err());
        let invalid_destination = destination.path().join("file");
        fs::write(&invalid_destination, "cannot extract into a file")?;
        assert!(archive_baseline(&workspace, "HEAD", &invalid_destination).is_err());
        Ok(())
    }

    #[test]
    fn named_exceptions_check_supported_dependency_members_recursively() -> Result<()> {
        let metadata = json!({
            "workspace_members": ["dep"],
            "packages": [{"id": "dep", "name": "public_dep", "version": "0.1.0",
                "targets": [{"name": "public_dep", "kind": ["lib"]}]}]
        });
        let root = json!({
            "root": 0, "includes_private": false, "format_version": 61,
            "external_crates": {"1": {"name": "public_dep"}},
            "paths": {"10": {"crate_id": 1, "path": ["public_dep", "Item"]}},
            "index": {
                "0": {"name": "api", "crate_id": 0, "visibility": "public", "inner": {"module": {"items": [1]}}},
                "1": {"name": "read", "crate_id": 0, "visibility": "public", "inner": {"function": {"sig": {"output": {"resolved_path": {"id": 10}}}}}}
            }
        });
        let dependency = json!({
            "root": 0, "includes_private": false, "format_version": 61, "external_crates": {},
            "paths": {"1": {"crate_id": 0, "path": ["public_dep", "Item"]}, "2": {"crate_id": 0, "path": ["public_dep", "Other"]}},
            "index": {
                "0": {"name": "public_dep", "crate_id": 0, "visibility": "public", "inner": {"module": {"items": [1, 2]}}},
                "1": {"name": "Item", "crate_id": 0, "visibility": "public", "inner": {"struct": {"kind": {"plain": {"fields": [3]}}, "impls": [4]}}},
                "2": {"name": "Other", "crate_id": 0, "visibility": "public", "inner": {"struct": {"kind": "unit", "impls": []}}},
                "3": {"name": "value", "crate_id": 0, "visibility": "public", "inner": {"struct_field": {"primitive": "bool"}}},
                "4": {"crate_id": 0, "inner": {"impl": {"items": [5], "trait": null}}},
                "5": {"name": "experimental", "crate_id": 0, "visibility": "public", "docs": "\n\n**Unstable:** Since 2026-10-13.", "inner": {"function": {"sig": {"output": {"resolved_path": {"id": 2}}}}}}
            }
        });
        let policy = json!({"recursive": true, "allowed-unstable-types": ["public_dep::Item"]});
        assert!(
            check_boundary(&root, &policy, &metadata, |_| Ok(dependency.clone()))?,
            "feature-unified experimental methods do not become supported members"
        );
        let mut leaked = dependency.clone();
        leaked["index"]["3"]["inner"]["struct_field"] = json!({"resolved_path": {"id": 2}});
        assert!(
            !check_boundary(&root, &policy, &metadata, |_| Ok(leaked.clone()))?,
            "a named type exception does not admit an unlisted type through a public field"
        );
        let mut direct = policy.clone();
        direct["recursive"] = json!(false);
        assert!(
            check_boundary(&root, &direct, &metadata, |_| Ok(leaked.clone()))?,
            "dependency member traversal is explicitly opt-in"
        );
        let mut leaked = dependency.clone();
        leaked["index"]["5"]["docs"] = Value::Null;
        assert!(
            !check_boundary(&root, &policy, &metadata, |_| Ok(leaked.clone()))?,
            "an ungated method returning an unlisted type is a supported API leak"
        );
        let mut blanket = policy.clone();
        blanket["allowed-unstable-crates"] = json!(["public_dep"]);
        assert!(
            check_boundary(&root, &blanket, &metadata, |_| Ok(leaked.clone()))?,
            "a whole-crate exception intentionally admits all of its own types"
        );
        let mut root = root;
        root["paths"]["10"]["path"] = json!(["public_dep", "Other"]);
        assert!(
            !check_boundary(&root, &policy, &metadata, |_| Ok(dependency.clone()))?,
            "a new direct type from an already allowed dependency is still rejected"
        );
        let typo = json!({"allowed-unstable-types": ["public_dep::Missing"]});
        assert!(
            check_boundary(&root, &typo, &metadata, |_| Ok(dependency.clone())).is_err(),
            "misspelled exceptions cannot silently grant permission"
        );
        let mut transitive_root = root;
        transitive_root["paths"]["10"]["path"] = json!(["public_dep", "Item"]);
        let mut transitive = dependency.clone();
        transitive["external_crates"] = json!({"1": {"name": "other_dep"}});
        transitive["paths"]["20"] = json!({"crate_id": 1, "path": ["other_dep", "Payload"]});
        transitive["index"]["3"]["inner"]["struct_field"] = json!({"resolved_path": {"id": 20}});
        let other = json!({
            "root": 0, "includes_private": false, "format_version": 61, "external_crates": {},
            "paths": {"1": {"crate_id": 0, "path": ["other_dep", "Payload"]}},
            "index": {
                "0": {"name": "other_dep", "crate_id": 0, "visibility": "public", "inner": {"module": {"items": [1]}}},
                "1": {"name": "Payload", "crate_id": 0, "visibility": "public", "inner": {"struct": {"kind": "unit", "impls": []}}}
            }
        });
        let mut metadata = metadata;
        metadata["workspace_members"] = json!(["dep", "other"]);
        metadata["packages"].as_array_mut().expect("test metadata has packages").push(json!({
            "id": "other", "name": "other_dep", "version": "0.1.0", "targets": [{"name": "other_dep", "kind": ["lib"]}]
        }));
        assert!(
            !check_boundary(&transitive_root, &policy, &metadata, |package| {
                Ok(if package["name"] == "public_dep" {
                    transitive.clone()
                } else {
                    other.clone()
                })
            })?,
            "recursive checks cross dependency boundaries rather than stopping at the first allowed type"
        );
        let mut policy = policy;
        policy["allowed-unstable-types"] = json!(["public_dep::Item", "other_dep::Payload"]);
        assert!(
            check_boundary(&transitive_root, &policy, &metadata, |package| {
                Ok(if package["name"] == "public_dep" {
                    transitive.clone()
                } else {
                    other.clone()
                })
            })?,
            "each exposed provisional type needs its own explicit exception"
        );
        metadata["packages"][0]["version"] = json!("1.0.0");
        let mut hidden = dependency;
        hidden["index"]["0"]["inner"]["module"]["items"] = json!([6]);
        hidden["index"]["6"] = json!({
            "crate_id": 0, "visibility": "public", "docs": "\n\n**Unstable.**",
            "inner": {"use": {"name": "Item", "id": 1}}
        });
        assert!(
            !check_boundary(&transitive_root, &policy, &metadata, |_| Ok(hidden.clone()))?,
            "an experimental re-export in a stable dependency does not make its type supported through feature unification"
        );
        Ok(())
    }

    #[test]
    fn public_graph_follows_associated_types_and_ignores_private_fields() -> Result<()> {
        let document = json!({
            "root": 0, "includes_private": false, "format_version": 61,
            "external_crates": {"1": {"name": "public_dep"}, "2": {"name": "private_dep"}},
            "paths": {"10": {"crate_id": 1, "path": ["public_dep", "Item"]}, "11": {"crate_id": 2, "path": ["private_dep", "Storage"]}},
            "index": {
                "0": {"crate_id": 0, "visibility": "public", "inner": {"module": {"items": [1]}}},
                "1": {"crate_id": 0, "visibility": "public", "inner": {"struct": {"kind": {"plain": {"fields": [2]}}, "impls": [3]}}},
                "2": {"crate_id": 0, "visibility": "default", "inner": {"struct_field": {"resolved_path": {"id": 11}}}},
                "3": {"crate_id": 0, "inner": {"impl": {"trait": {"id": 10}, "items": [4]}}},
                "4": {"crate_id": 0, "visibility": "default", "inner": {"assoc_type": {"type": {"resolved_path": {"id": 10}}}}}
            }
        });
        let exposed = public_dependencies(&document)?;
        assert!(
            exposed.contains_key("public_dep"),
            "associated types are public dependency commitments"
        );
        assert!(
            !exposed.contains_key("private_dep"),
            "private storage does not force dependency propagation"
        );
        let mut blanket = document;
        blanket["index"]["3"]["inner"]["impl"]["blanket_impl"] = json!({"generic": "T"});
        assert!(
            public_dependencies(&blanket)?.is_empty(),
            "external blanket impls do not bind the implementing crate to that trait's version"
        );
        Ok(())
    }

    fn function_api() -> Value {
        json!({
            "root": 0, "includes_private": false, "format_version": 61,
            "external_crates": {"1": {"name": "alloc"}},
            "paths": {"10": {"crate_id": 1, "path": ["alloc", "string", "String"]}, "11": {"crate_id": 1, "path": ["alloc", "vec", "Vec"]}},
            "index": {
                "0": {"name": "api", "crate_id": 0, "visibility": "public", "inner": {"module": {"items": [1]}}},
                "1": {"name": "read", "crate_id": 0, "visibility": "public", "inner": {"function": {"sig": {"inputs": [], "output": {"resolved_path": {"id": 10, "path": "String"}}}, "generics": {"params": [], "where_predicates": []}}}},
                "2": {"name": "experimental", "crate_id": 0, "visibility": "default", "inner": {"function": {"sig": {"inputs": [], "output": {"resolved_path": {"id": 11}}}}}}
            }
        })
    }

    #[test]
    fn type_changes_are_detected_even_within_one_dependency() -> Result<()> {
        let before = function_api();
        let mut after = before.clone();
        after["index"]["2"]["inner"]["function"]["sig"]["output"] = json!({"primitive": "bool"});
        assert!(
            unchanged_types(&before, &after)?,
            "unreachable experimental functions are outside the stable contract"
        );
        after["index"]["1"]["inner"]["function"]["sig"]["output"] = json!({"resolved_path": {"id": 11, "path": "Vec"}});
        assert!(
            !unchanged_types(&before, &after)?,
            "String to Vec is a breaking return type even though both come from alloc"
        );
        Ok(())
    }

    #[test]
    fn rustdoc_ids_and_source_aliases_do_not_define_type_identity() -> Result<()> {
        let before = function_api();
        let mut after = before.clone();
        after["paths"]["99"] = after["paths"]["10"].clone();
        after["index"]["1"]["inner"]["function"]["sig"]["output"] =
            json!({"resolved_path": {"id": 99, "path": "std::string::String"}});
        assert!(
            unchanged_types(&before, &after)?,
            "type identity uses resolved paths instead of session IDs or import spelling"
        );
        Ok(())
    }

    #[test]
    fn public_tuple_fields_and_bounds_are_checked() -> Result<()> {
        let mut before = function_api();
        before["index"]["0"]["inner"]["module"]["items"] = json!([3]);
        before["index"]["3"] = json!({"name": "Wrapper", "crate_id": 0, "visibility": "public", "inner": {"struct": {"kind": {"tuple": [4]}, "generics": {"params": [], "where_predicates": []}, "impls": []}}});
        before["index"]["4"] = json!({"name": "0", "crate_id": 0, "visibility": "public", "inner": {"struct_field": {"resolved_path": {"id": 10}}}});
        assert!(
            public_dependencies(&before)?.contains_key("alloc"),
            "tuple field types are public dependencies"
        );
        let mut after = before.clone();
        after["index"]["4"]["inner"]["struct_field"] = json!({"resolved_path": {"id": 11}});
        assert!(
            !unchanged_types(&before, &after)?,
            "changing a tuple field's type breaks its contract"
        );
        let mut after = before.clone();
        after["index"]["3"]["inner"]["struct"]["generics"]["where_predicates"] =
            json!([{"lifetime_predicate": {"lifetime": "'a", "outlives": ["'static"]}}]);
        assert!(
            !unchanged_types(&before, &after)?,
            "tightening generic bounds breaks the contract"
        );
        Ok(())
    }

    #[test]
    fn fields_of_types_returned_from_private_modules_are_checked() -> Result<()> {
        let mut before = function_api();
        before["paths"]["3"] = json!({"crate_id": 0, "path": ["api", "hidden", "S"]});
        before["index"]["1"]["inner"]["function"]["sig"]["output"] = json!({"resolved_path": {"id": 3}});
        before["index"]["3"] = json!({"name": "S", "crate_id": 0, "visibility": "public", "inner": {"struct": {"kind": {"plain": {"fields": [4]}}, "generics": {"params": [], "where_predicates": []}, "impls": []}}});
        before["index"]["4"] = json!({"name": "x", "crate_id": 0, "visibility": "public", "inner": {"struct_field": {"resolved_path": {"id": 10}}}});
        let mut after = before.clone();
        after["index"]["4"]["inner"]["struct_field"] = json!({"resolved_path": {"id": 11}});
        assert!(
            !unchanged_types(&before, &after)?,
            "callers can access read().x even though hidden::S has no public name"
        );
        Ok(())
    }

    #[test]
    fn profiles_cannot_enable_unstable_indirectly() {
        let features = json!({"default": ["bundle"], "bundle": ["unstable"], "unstable": []});
        let profile = FeatureProfile {
            default_features: true,
            features: Vec::new(),
        };
        assert!(
            feature_args(&profile, &features, &["unstable".into()]).is_err(),
            "stable profiles exclude the entire unstable feature closure"
        );
    }

    #[test]
    fn signature_change_reports_keep_changed_and_removed_items() {
        let old = json!({"inputs": ["bool"], "output": "bool", "generics": {"params": []}});
        let new = json!({"inputs": ["u8"], "output": "bool", "generics": {"params": []}});
        let before = BTreeMap::from([
            ("api::changed".into(), old.clone()),
            ("api::removed".into(), old.clone()),
        ]);
        let after = BTreeMap::from([("api::changed".into(), new.clone())]);
        let changes = signature_changes(&before, &after);
        assert_eq!(changes.len(), 2);
        assert!(
            matches!(&changes[0], Reason::SignatureChanged { path, before, after: Some(after) }
            if path == "api::changed" && before == &old && after == &new)
        );
        assert!(
            matches!(&changes[1], Reason::SignatureChanged { path, before, after: None }
            if path == "api::removed" && before == &old)
        );
    }

    #[test]
    fn ambiguous_public_dependency_origins_and_unknown_schemas_fail_closed() {
        let metadata = json!({"packages": [
            {"name": "foo", "version": "1.0.0", "targets": [{"kind": ["lib"], "name": "foo"}]},
            {"name": "foo", "version": "2.0.0", "targets": [{"kind": ["lib"], "name": "foo"}]}
        ]});
        assert!(
            dependency_identities(&metadata, "foo").is_err(),
            "identical rustdoc crate names cannot identify which major version a public type uses"
        );
        let mut document = function_api();
        document["format_version"] = json!(62);
        assert!(
            public_dependencies(&document).is_err(),
            "new schemas need review so new type-reference shapes cannot be silently skipped"
        );
    }

    #[test]
    fn requested_manifest_selects_the_package_when_names_overlap() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let old = directory.path().join("registry.toml");
        let current = directory.path().join("workspace.toml");
        fs::write(&old, "")?;
        fs::write(&current, "")?;
        let metadata = json!({"packages": [
            {"name": "foo", "version": "1.0.0", "manifest_path": old},
            {"name": "foo", "version": "2.0.0", "manifest_path": current}
        ]});
        assert_eq!(
            find_package(&metadata, "foo", &current)?["version"],
            "2.0.0",
            "a registry package cannot supply the workspace package's stable feature profiles"
        );
        Ok(())
    }

    #[test]
    fn library_targets_include_explicit_crate_types_and_proc_macros() -> Result<()> {
        for kind in ["lib", "rlib", "dylib", "staticlib", "cdylib", "proc-macro"] {
            let package = json!({"targets": [{"kind": [kind], "name": "public_api"}]});
            assert_eq!(library_name(&package)?, "public_api", "{kind} is a library target");
        }
        assert!(
            library_name(&json!({"targets": [{"kind": ["bin"], "name": "application"}]})).is_err(),
            "binary-only packages do not have a library API"
        );
        Ok(())
    }

    #[test]
    fn planned_dependencies_use_the_actual_library_target_name() -> Result<()> {
        let metadata = json!({"workspace_members": ["local"], "packages": [
            {"id": "registry", "name": "package-name", "targets": [{"kind": ["lib"], "name": "old_name"}]},
            {"id": "local", "name": "package-name", "targets": [{"kind": ["lib"], "name": "public_types"}]}
        ]});
        assert_eq!(
            planned_dependency_crate(&metadata, "package-name")?,
            Some("public_types"),
            "custom lib names cannot hide a planned public dependency break"
        );
        Ok(())
    }

    #[test]
    fn local_dependency_identity_preserves_origin_but_normalizes_archives() -> Result<()> {
        let mut before = json!({"workspace_root": "/baseline", "packages": [
            {"name": "foo", "version": "1.0.0", "source": null, "manifest_path": "/baseline/foo/Cargo.toml", "targets": [{"kind": ["lib"], "name": "foo"}]}
        ]});
        let mut after = before.clone();
        after["workspace_root"] = json!("/current");
        after["packages"][0]["manifest_path"] = json!("/current/foo/Cargo.toml");
        assert_eq!(
            dependency_identities(&before, "foo")?,
            dependency_identities(&after, "foo")?,
            "temporary baseline archive directories do not change nominal identity"
        );
        before["packages"][0]["manifest_path"] = json!("/external/old/Cargo.toml");
        after["packages"][0]["manifest_path"] = json!("/external/new/Cargo.toml");
        assert_ne!(
            dependency_identities(&before, "foo")?,
            dependency_identities(&after, "foo")?,
            "distinct external path packages must not collapse to one workspace identity"
        );
        let paths = BTreeSet::from(["foo::Item".to_owned()]);
        let changes = identity_changes(&BTreeMap::from([("foo".into(), paths.clone())]), &before, &after)?;
        assert_eq!(changes.len(), 1);
        let Reason::DependencyIdentityChanged {
            crate_name,
            paths: reported_paths,
            before: old,
            after: new,
        } = &changes[0]
        else {
            panic!("expected an identity change");
        };
        assert_eq!(crate_name, "foo");
        assert_eq!(reported_paths, &paths);
        assert_eq!(old, &dependency_identities(&before, "foo")?);
        assert_eq!(new, &dependency_identities(&after, "foo")?);
        Ok(())
    }

    #[test]
    fn dependency_identity_uses_cargos_compatibility_ranges() -> Result<()> {
        assert_eq!(
            compatibility_family("1.2.3")?,
            compatibility_family("1.4.0")?,
            "stable minor versions can unify"
        );
        assert_ne!(
            compatibility_family("0.2.3")?,
            compatibility_family("0.3.0")?,
            "pre-1.0 minor bumps change nominal identity"
        );
        assert_ne!(
            compatibility_family("0.0.3")?,
            compatibility_family("0.0.4")?,
            "0.0 patch versions are incompatible"
        );
        Ok(())
    }
}
