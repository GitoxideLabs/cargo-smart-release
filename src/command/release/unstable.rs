use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context as _};
use cargo_metadata::{DependencyKind, Package};
use semver::{Version, VersionReq};
use toml_edit::{value, Array, DocumentMut, InlineTable, Item, Table};

use super::{cargo, git, manifest::find_dependency_tables, Options};
use crate::{
    api::has_parallel_release,
    utils::{package_by_id, package_by_name, release_tag_name, will},
    version::BumpSpec,
    Context,
};

const GATE: &str = "unstable";
const MARKER_PREFIX: &str = "_cargo-smart-release-unstable-for-";
const STABLE_ALIAS: &str = "smart-release-stable";
pub(super) struct Companion<'a> {
    pub package: &'a Package,
    pub stable: &'a Version,
    pub companion: &'a Version,
    pub manifest: &'a Path,
}

pub(super) fn source_manifest(ctx: &Context, package: &Package) -> anyhow::Result<PathBuf> {
    let worktree = ctx
        .repo
        .workdir()
        .context("A release requires a Git worktree")?
        .canonicalize()?;
    let path = package.manifest_path.as_std_path();
    let root = path
        .parent()
        .context("Package manifest has no parent")?
        .canonicalize()?;
    Ok(root
        .strip_prefix(&worktree)
        .with_context(|| format!("'{}' is outside the Git worktree", path.display()))?
        .join("Cargo.toml"))
}

pub(super) fn prepare_source<'repo>(
    companions: &[Companion<'_>],
    stable_commit: Option<gix::Id<'repo>>,
    ctx: &Context,
    options: &Options,
) -> anyhow::Result<Option<gix::Id<'repo>>> {
    if companions.is_empty() || options.skip_tag {
        return Ok(None);
    }
    let packages = companions
        .iter()
        .map(|release| format!("{}-{}", release.package.name, release.companion))
        .collect::<Vec<_>>()
        .join(", ");
    if options.dry_run {
        log::trace!("WOULD create one companion source commit for {packages} with the stable release as its parent");
        return Ok(None);
    }
    let stable = stable_commit.context("Expected a stable release commit for the companion source")?;
    let packages_and_paths = companions
        .iter()
        .map(|release| Ok((source_manifest(ctx, release.package)?, release.manifest)))
        .collect::<anyhow::Result<Vec<_>>>()?;
    let source = source_commit(
        stable,
        &packages_and_paths,
        &format!("Prepare experimental releases of {packages}\n"),
        options.signoff,
    )?;
    let name = source_ref(stable);
    if let Some(mut previous) = stable.repo().try_find_reference(name.as_str())? {
        if previous.peel_to_id()? != source {
            bail!("The stable release {stable} already has a different prepared companion source commit");
        }
    } else {
        stable.repo().reference(
            name,
            source,
            gix::refs::transaction::PreviousValue::MustNotExist,
            "prepare companion releases",
        )?;
    }
    log::info!("Created companion source commit {source} for {packages}");
    Ok(Some(source))
}

fn source_ref(stable: gix::Id<'_>) -> String {
    format!("refs/cargo-smart-release/companions/{stable}")
}

fn source_commit<'repo>(
    parent: gix::Id<'repo>,
    manifests: &[(PathBuf, &Path)],
    message: &str,
    signoff: bool,
) -> anyhow::Result<gix::Id<'repo>> {
    use gix::objs::tree::EntryKind;

    let parent_commit = parent.object()?.try_into_commit()?;
    let mut tree = parent_commit.tree()?.edit()?;
    for (repository_manifest, staged_manifest) in manifests {
        let repository_dir = repository_manifest
            .parent()
            .context("Repository manifest has no parent")?;
        let staged_dir = staged_manifest.parent().context("Staged manifest has no parent")?;
        for relative in source_files(staged_dir)? {
            let stage_path = staged_dir.join(&relative);
            let mode = fs::symlink_metadata(&stage_path)?;
            let (kind, id) = if mode.is_symlink() {
                let target = fs::read_link(&stage_path)?;
                let target = gix::path::into_bstr(target)?;
                (EntryKind::Link, parent.repo().write_blob(target.as_ref())?)
            } else {
                #[cfg(unix)]
                let executable = {
                    use std::os::unix::fs::PermissionsExt;
                    mode.permissions().mode() & 0o111 != 0
                };
                #[cfg(not(unix))]
                let executable = false;
                let kind = if executable {
                    EntryKind::BlobExecutable
                } else {
                    EntryKind::Blob
                };
                (kind, parent.repo().write_blob_stream(fs::File::open(&stage_path)?)?)
            };
            let path = gix::path::to_unix_separators(gix::path::into_bstr(repository_dir.join(relative))?);
            tree.upsert(path.as_ref(), kind, id)?;
        }
    }
    let author = parent_commit.author()?.to_owned()?;
    let committer = parent_commit.committer()?.to_owned()?;
    let mut message = message.to_owned();
    if signoff {
        message.push_str(&format!("\nSigned-off-by: {} <{}>\n", committer.name, committer.email));
    }
    Ok(parent.repo().write_object(gix::objs::Commit {
        tree: tree.write()?.detach(),
        parents: std::iter::once(parent.detach()).collect(),
        author,
        committer,
        encoding: None,
        message: message.into(),
        extra_headers: Vec::new(),
    })?)
}

fn source_files(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut pending = vec![PathBuf::new()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(root.join(&directory))? {
            let entry = entry?;
            let relative = directory.join(entry.file_name());
            let kind = entry.file_type()?;
            if kind.is_dir() {
                pending.push(relative);
            } else if kind.is_file() || kind.is_symlink() {
                files.push(relative);
            } else {
                bail!("Unsupported file '{}' in the staged package", entry.path().display());
            }
        }
    }
    files.sort_unstable();
    Ok(files)
}

pub(super) fn checkout_source(commit: gix::Id<'_>, destination: &Path) -> anyhow::Result<()> {
    fs::create_dir(destination)?;
    let index = tempfile::tempdir()?;
    let mut prefix = destination.as_os_str().to_os_string();
    prefix.push("/");
    let git = || {
        let mut git = Command::new(gix::path::env::exe_invocation());
        git.current_dir(commit.repo().workdir().expect("release has a worktree"))
            .env("GIT_INDEX_FILE", index.path().join("index"));
        git
    };
    let read = git().arg("read-tree").arg(commit.to_string()).output()?;
    if !read.status.success() {
        bail!(
            "Could not read companion source commit {commit}: {}",
            String::from_utf8_lossy(&read.stderr)
        );
    }
    let output = git()
        .args(["checkout-index", "--all", "--ignore-skip-worktree-bits", "--prefix"])
        .arg(prefix)
        .output()?;
    if !output.status.success() {
        bail!(
            "Could not check out companion source commit {commit}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

pub(super) fn create_tag(
    release: &Companion<'_>,
    commit: Option<gix::Id<'_>>,
    ctx: &Context,
    options: Options,
) -> anyhow::Result<Option<gix::refs::FullName>> {
    let Companion {
        package,
        stable,
        companion,
        ..
    } = release;
    let name = &package.name;
    let message = format!(
        "Release {name} v{companion} with experimental APIs\n\n\
         Prepared alongside {name} v{stable}, with `unstable` enabled by default.\n\n\
         https://static.crates.io/crates/{name}/{name}-{companion}.crate\n"
    );
    git::create_version_tag(package, companion, commit, Some(message), ctx, options)
}

pub(super) fn resume(ctx: &Context, options: &Options) -> anyhow::Result<bool> {
    let [name] = ctx.crate_names.as_slice() else {
        return Ok(false);
    };
    if !matches!(ctx.bump, BumpSpec::Keep) || options.skip_publish || options.skip_tag || uses_custom_registry(options)
    {
        return Ok(false);
    }
    let package = package_by_name(&ctx.meta, name)?;
    let version = &package.version;
    if !has_parallel_release(package, version) {
        return Ok(false);
    }
    let stable_tag_name = release_tag_name(package, version, &ctx.repo)?;
    let Some(mut stable_tag) = ctx
        .repo
        .try_find_reference(format!("refs/tags/{stable_tag_name}").as_str())?
    else {
        return Ok(false);
    };
    let commit = stable_tag.peel_to_id()?;
    if ctx.repo.head()?.try_into_peeled_id()? != Some(commit) {
        return Ok(false);
    }
    let Some(entry) = ctx.crates_index.crate_(name)? else {
        return Ok(false);
    };
    if !entry.versions().iter().any(|entry| {
        entry.version() == version.to_string() && !entry.is_yanked() && !entry.features().contains_key(GATE)
    }) {
        return Ok(false);
    }
    super::assure_working_tree_is_unchanged(Options {
        allow_dirty: false,
        ..options.clone()
    })?;
    let marker = format!("{MARKER_PREFIX}{version}");
    let already_published = entry
        .versions()
        .iter()
        .find(|entry| !entry.is_yanked() && entry.features().contains_key(&marker))
        .map(|entry| Version::parse(entry.version()))
        .transpose()?;
    let relative_manifest = source_manifest(ctx, package)?;
    let prepared = ctx
        .repo
        .try_find_reference(source_ref(commit).as_str())?
        .map(|mut source| source.peel_to_id())
        .transpose()?;
    if let Some(version) = &already_published {
        let companion_tag_name = release_tag_name(package, version, &ctx.repo)?;
        if let Some(mut tag) = ctx
            .repo
            .try_find_reference(format!("refs/tags/{companion_tag_name}").as_str())?
        {
            let tagged = tag.peel_to_id()?.object()?.try_into_commit()?;
            if !tagged.parent_ids().eq(std::iter::once(commit)) {
                bail!("Release tag '{companion_tag_name}' is not a child of the stable release commit");
            }
            log::info!("'{name} v{version}' is already published and tagged");
            git::push_tags_and_head(
                &ctx.repo,
                &[stable_tag.detach().name, tag.detach().name],
                options.clone(),
            )?;
            return Ok(true);
        }
    }
    let staging = tempfile::tempdir()?;
    let (companion, manifest, source) = if let Some(source) = prepared {
        let source_commit = source.object()?.try_into_commit()?;
        if !source_commit.parent_ids().eq(std::iter::once(commit)) {
            bail!("The prepared companion source {source} is not a child of the stable release commit");
        }
        let tree = source_commit.tree()?;
        let blob = tree
            .lookup_entry_by_path(&relative_manifest)?
            .context("The prepared source has no companion manifest")?
            .object()?
            .try_into_blob()?;
        let doc = std::str::from_utf8(&blob.data)?.parse::<DocumentMut>()?;
        if doc.get("features").and_then(|features| features.get(&marker)).is_none() {
            bail!("'{name}' was not recorded as a companion in the prepared source commit {source}");
        }
        let companion = Version::parse(
            doc["package"]["version"]
                .as_str()
                .context("Prepared companion has no version")?,
        )?;
        if already_published
            .as_ref()
            .is_some_and(|published| published != &companion)
        {
            bail!("The published companion for '{name}' differs from the prepared version {companion}");
        }
        let checkout = staging.path().join("companion-source");
        if already_published.is_none() {
            checkout_source(source, &checkout)?;
        }
        (companion, checkout.join(&relative_manifest), Some(source))
    } else {
        if let Some(published) = already_published {
            bail!("'{name} v{published}' is already published, but its prepared source commit is unavailable");
        }
        let plan = Plan::new([(package, version)], ctx, Some(options))?;
        let (_, manifest) = plan
            .stage(package, version, ctx, options, staging.path())?
            .context("Expected staged release manifests")?;
        let manifest = manifest.context("Expected a staged experimental release manifest")?;
        let companion = plan.companion(package).context("Expected a companion version")?.clone();
        let source = prepare_source(
            &[Companion {
                package,
                stable: version,
                companion: &companion,
                manifest: &manifest,
            }],
            Some(commit),
            ctx,
            options,
        )?;
        let manifest = if let Some(source) = source {
            let checkout = staging.path().join("companion-source");
            checkout_source(source, &checkout)?;
            checkout.join(&relative_manifest)
        } else {
            manifest
        };
        (companion, manifest, source)
    };
    if already_published.is_none() {
        cargo::publish_crate(package, &manifest, true, true, false, None, options.clone())?;
    }
    let mut tags = vec![stable_tag.detach().name];
    tags.extend(create_tag(
        &Companion {
            package,
            stable: version,
            companion: &companion,
            manifest: &manifest,
        },
        source,
        ctx,
        options.clone(),
    )?);
    git::push_tags_and_head(&ctx.repo, &tags, options.clone())?;
    Ok(true)
}

fn uses_custom_registry(options: &Options) -> bool {
    options
        .registry
        .as_deref()
        .is_some_and(|registry| registry != "crates-io")
}

#[derive(Default)]
pub(crate) struct Plan<'a> {
    packages: BTreeMap<&'a str, &'a Package>,
    published: BTreeMap<&'a str, &'a Version>,
    companions: BTreeMap<&'a str, Version>,
}

impl<'a> Plan<'a> {
    pub fn new(
        publishees: impl IntoIterator<Item = (&'a Package, &'a Version)>,
        ctx: &'a Context,
        options: Option<&Options>,
    ) -> anyhow::Result<Self> {
        let mut plan = Self {
            published: publishees
                .into_iter()
                .map(|(package, version)| (package.name.as_str(), version))
                .collect(),
            ..Self::default()
        };
        for id in &ctx.meta.workspace_members {
            let package = package_by_id(&ctx.meta, id);
            let name = package.name.as_str();
            let version = plan.published.get(name).copied().unwrap_or(&package.version);
            if has_parallel_release(package, version) {
                plan.packages.insert(name, package);
            }
        }
        for (&name, version) in &plan.published {
            let Some(package) = plan.packages.get(name) else {
                continue;
            };
            if options.is_some_and(uses_custom_registry) {
                bail!("Parallel unstable releases require the crates.io index to allocate a version for '{name}'");
            }
            if package.links.is_some() {
                bail!("Cannot publish parallel unstable releases for '{name}' because Cargo can only resolve one version of a crate with a links key");
            }
            let entry = ctx.crates_index.crate_(name)?;
            let mut versions = entry
                .iter()
                .flat_map(|entry| entry.versions())
                .map(|entry| Version::parse(entry.version()));
            let mut companion = next_version(&mut versions)?;
            while ctx
                .repo
                .try_find_reference(
                    format!("refs/tags/{}", release_tag_name(package, &companion, &ctx.repo)?).as_str(),
                )?
                .is_some()
            {
                companion = next_version(&mut std::iter::once(Ok(companion)))?;
            }
            if let Some(options) = options {
                log::info!(
                    "{} publish '{name} v{companion}' with experimental APIs after v{version}",
                    will(options.dry_run)
                );
            }
            plan.companions.insert(name, companion);
        }
        Ok(plan)
    }

    pub fn companion(&self, package: &Package) -> Option<&Version> {
        self.companions.get(package.name.as_str())
    }

    pub(crate) fn api_routes<'p>(
        &self,
        package: &'p Package,
        ctx: &Context,
        experimental: bool,
    ) -> anyhow::Result<Vec<(&'p cargo_metadata::Dependency, Option<Version>)>> {
        let excluded = if !experimental && self.companion(package).is_some() {
            excluded_features(package)
        } else {
            BTreeSet::new()
        };
        let forwarding = package
            .features
            .iter()
            .filter(|(feature, _)| feature.as_str() == "default" || !excluded.contains(feature.as_str()))
            .flat_map(|(_, features)| features)
            .map(String::as_str);
        let mut routes = Vec::new();
        for dependency in package
            .dependencies
            .iter()
            .filter(|dep| dep.kind == DependencyKind::Normal)
        {
            let Some(target) = self.target(dependency) else {
                continue;
            };
            if requests_experimental(dependency, target, forwarding.clone()) {
                let version = match self.companion(target) {
                    Some(version) => Some(version.clone()),
                    None => published_companion(ctx, target)?,
                };
                if let Some(version) = version {
                    routes.push((dependency, Some(version)));
                }
            } else {
                routes.push((dependency, None));
            }
        }
        Ok(routes)
    }

    pub fn publish_order(
        &self,
        mut pending: Vec<(&'a Package, &'a Version)>,
        ctx: &Context,
        options: &Options,
    ) -> anyhow::Result<Vec<(&'a Package, &'a Version)>> {
        if options.skip_publish {
            return Ok(pending);
        }
        let mut prerequisites = BTreeMap::new();
        for (package, _) in &pending {
            let mut required = BTreeSet::new();
            for dependency in &package.dependencies {
                let Some(target) = crate::utils::workspace_package_by_dependency(&ctx.meta, dependency) else {
                    continue;
                };
                let normal_or_build = matches!(dependency.kind, DependencyKind::Normal | DependencyKind::Build);
                let routed = self.target(dependency).is_some_and(|target| {
                    requests_experimental(
                        dependency,
                        target,
                        package.features.values().flatten().map(String::as_str),
                    )
                });
                if !normal_or_build && (!routed || !versioned_dev_dependency(ctx, package, dependency)?) {
                    continue;
                }
                if routed {
                    self.required_companion(package, target, ctx, options)?;
                }
                if self.published.contains_key(target.name.as_str()) && (normal_or_build || routed) {
                    required.insert(target.name.as_str());
                }
            }
            prerequisites.insert(package.name.as_str(), required);
        }
        let mut ordered = Vec::with_capacity(pending.len());
        let mut completed = BTreeSet::new();
        while !pending.is_empty() {
            let Some(index) = pending
                .iter()
                .position(|(package, _)| prerequisites[package.name.as_str()].is_subset(&completed))
            else {
                let blocked = pending
                    .iter()
                    .map(|(package, _)| package.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                bail!(
                    "Cannot order the uploads. The selected crates need unpublished versions of each other: {blocked}"
                );
            };
            let release = pending.remove(index);
            completed.insert(release.0.name.as_str());
            ordered.push(release);
        }
        Ok(ordered)
    }

    fn required_companion(
        &self,
        package: &Package,
        target: &Package,
        ctx: &Context,
        options: &Options,
    ) -> anyhow::Result<Version> {
        let name = &target.name;
        if uses_custom_registry(options) {
            bail!("Parallel unstable dependencies require the crates.io index for '{name}'");
        }
        match self.companion(target) {
            Some(version) => Ok(version.clone()),
            None => published_companion(ctx, target)?.with_context(|| format!(
                "'{}' uses the experimental API of '{name} v{}'; include '{name}' in the release to publish its companion",
                package.name, target.version
            )),
        }
    }

    fn needs_staging(&self, package: &Package) -> bool {
        self.companion(package).is_some()
            || package.dependencies.iter().any(|dependency| {
                self.target(dependency).is_some_and(|target| {
                    requests_experimental(
                        dependency,
                        target,
                        package.features.values().flatten().map(String::as_str),
                    )
                })
            })
    }

    fn target(&self, dependency: &cargo_metadata::Dependency) -> Option<&Package> {
        let path = dependency.path.as_deref()?;
        self.packages
            .get(dependency.name.as_str())
            .copied()
            .filter(|package| package.manifest_path.parent() == Some(path))
    }

    pub fn stage(
        &self,
        package: &Package,
        version: &Version,
        ctx: &Context,
        options: &Options,
        staging: &Path,
    ) -> anyhow::Result<Option<(PathBuf, Option<PathBuf>)>> {
        if !self.needs_staging(package) {
            return Ok(None);
        }
        let target = staging.join("cargo");
        let current_version = if options.dry_run { &package.version } else { version };
        let status = Command::new("cargo")
            .current_dir(&ctx.root)
            .args([
                "package",
                "--no-verify",
                "--exclude-lockfile",
                "--allow-dirty",
                "--manifest-path",
            ])
            .arg(&package.manifest_path)
            .arg("--package")
            .arg(package.name.as_str())
            .arg("--target-dir")
            .arg(&target)
            .status()
            .context("Could not start Cargo to stage the release")?;
        if !status.success() {
            bail!("Could not stage '{}' with cargo package", package.name);
        }
        let stem = format!("{}-{current_version}", package.name);
        let archive = target.join("package").join(format!("{stem}.crate"));
        let make_variant = |companion: Option<&Version>| -> anyhow::Result<PathBuf> {
            let destination = staging.join(format!("{}-{}", package.name, companion.unwrap_or(version)));
            let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(fs::File::open(&archive)?));
            tar.unpack(&destination)?;
            let root = destination.join(&stem);
            for generated in ["Cargo.toml.orig", ".cargo_vcs_info.json"] {
                let path = root.join(generated);
                if path.is_file() {
                    fs::remove_file(path)?;
                }
            }
            let path = root.join("Cargo.toml");
            let mut doc = fs::read_to_string(&path)?.parse::<DocumentMut>()?;
            self.prepare_manifest(package, version, companion, &mut doc, ctx, options)?;
            if companion.is_some() {
                // Publish this crate independently, even when the source checkout contains a larger workspace.
                doc["workspace"] = Item::Table(Table::new());
                let mut include = Array::new();
                for file in source_files(&root)? {
                    if file == Path::new("Cargo.toml") || file == Path::new("Cargo.lock") {
                        continue;
                    }
                    use gix::bstr::ByteSlice as _;
                    let path = gix::path::to_unix_separators(gix::path::into_bstr(file)?);
                    let mut pattern = String::from("/");
                    for ch in path.to_str()?.chars() {
                        if matches!(ch, '\\' | '*' | '?' | '[' | ']' | ' ') {
                            pattern.push('\\');
                        }
                        pattern.push(ch);
                    }
                    include.push(pattern);
                }
                doc["package"]["include"] = value(include);
                doc["package"]
                    .as_table_like_mut()
                    .expect("Cargo normalized the package table")
                    .remove("exclude");
            }
            fs::write(&path, doc.to_string())?;
            if let Err(err) = fs::copy(ctx.root.join("Cargo.lock"), root.join("Cargo.lock")) {
                if err.kind() != std::io::ErrorKind::NotFound {
                    return Err(err).context("Could not seed the staged package lockfile");
                }
            }
            Ok(path)
        };
        let stable = make_variant(None)?;
        Ok(Some((
            stable,
            self.companion(package)
                .map(|version| make_variant(Some(version)))
                .transpose()?,
        )))
    }

    fn prepare_manifest(
        &self,
        package: &Package,
        version: &Version,
        companion: Option<&Version>,
        doc: &mut DocumentMut,
        ctx: &Context,
        options: &Options,
    ) -> anyhow::Result<()> {
        doc["package"]["version"] = value(companion.unwrap_or(version).to_string());
        let mut removed_features = BTreeSet::new();
        let mut removed_dependencies = BTreeSet::new();
        if let Some(companion) = companion {
            let features = doc["features"]
                .as_table_like_mut()
                .context("Expected a feature table")?;
            if features.iter().any(|(name, _)| name.starts_with(MARKER_PREFIX)) {
                bail!("The feature prefix '{MARKER_PREFIX}' is reserved for parallel unstable releases");
            }
            features.insert(&format!("{MARKER_PREFIX}{version}"), value(Array::new()));
            add_feature(doc, "default", GATE)?;
            if package.metadata["smart-release"]["unstable-stable-dependency"].as_bool() == Some(true) {
                let dependencies = doc.entry("dependencies").or_insert(Item::Table(Table::new()));
                let dependencies = dependencies
                    .as_table_like_mut()
                    .context("Expected a dependency table")?;
                if dependencies.contains_key(STABLE_ALIAS) {
                    bail!(
                        "'{STABLE_ALIAS}' is reserved for the stable release of '{}'",
                        package.name
                    );
                }
                let mut dependency = InlineTable::new();
                dependency.insert("package", package.name.as_str().into());
                dependency.insert(
                    "version",
                    format!("^{}.{}.{}", version.major, version.minor, version.patch).into(),
                );
                dependencies.insert(STABLE_ALIAS, value(dependency));
            }
            debug_assert_eq!(companion.major, 0);
        } else if self.companion(package).is_some() {
            (removed_features, removed_dependencies) = strip_experimental_features(doc, package)?;
        }

        let forwarding: Vec<_> = doc
            .get("features")
            .and_then(Item::as_table_like)
            .into_iter()
            .flat_map(|features| features.iter())
            .flat_map(|(_, items)| items.as_array().into_iter().flatten())
            .filter_map(|item| item.as_str().filter(|item| item.contains('/')).map(str::to_owned))
            .collect();
        for (table, kind) in find_dependency_tables(doc) {
            for alias in &removed_dependencies {
                if table
                    .get(alias)
                    .and_then(|entry| entry.get("optional"))
                    .and_then(Item::as_bool)
                    == Some(true)
                {
                    table.remove(alias);
                }
            }
            for (alias, entry) in table.iter_mut() {
                let alias = alias.get();
                let Some(dependency) = package.dependencies.iter().find(|dependency| {
                    dependency.path.is_some()
                        && dependency.rename.as_deref().unwrap_or(&dependency.name) == alias
                        && dependency_table_matches(dependency, &kind)
                }) else {
                    continue;
                };
                let name = dependency.name.as_str();
                let Some(entry) = entry.as_table_like_mut() else {
                    continue;
                };
                if let Some(target) = self
                    .target(dependency)
                    .filter(|target| requests_experimental(dependency, target, forwarding.iter().map(String::as_str)))
                {
                    let candidate = self.required_companion(package, target, ctx, options)?;
                    entry.insert("version", value(format!("^{candidate}")));
                    if entry.get("default-features").and_then(Item::as_bool) == Some(false) {
                        let features = entry.entry("features").or_insert(value(Array::new()));
                        let features = features.as_array_mut().context("Expected a dependency feature array")?;
                        if !features.iter().any(|feature| feature.as_str() == Some(GATE)) {
                            features.push(GATE);
                        }
                    } else if let Some(features) = entry.get_mut("features").and_then(Item::as_array_mut) {
                        features.retain(|feature| feature.as_str() != Some(GATE));
                    }
                    continue;
                }
                // Dry runs stage the original source, before stable path requirements are adjusted.
                if options.dry_run {
                    if let (Some(next), Some(requirement)) =
                        (self.published.get(name), entry.get("version").and_then(Item::as_str))
                    {
                        if !VersionReq::parse(requirement)?.matches(next) {
                            entry.insert("version", value(format!("^{next}")));
                        }
                    }
                }
            }
        }
        if !removed_features.is_empty() {
            let check_cfg = &mut doc["lints"]["rust"]["unexpected_cfgs"];
            if let Some(level) = check_cfg.as_str() {
                let mut lint = InlineTable::new();
                lint.insert("level", level.into());
                *check_cfg = value(lint);
            } else if check_cfg.is_none() {
                let mut lint = InlineTable::new();
                lint.insert("level", "warn".into());
                *check_cfg = value(lint);
            }
            let check_cfg = check_cfg
                .as_table_like_mut()
                .context("Expected an unexpected_cfgs lint table")?;
            let declarations = check_cfg.entry("check-cfg").or_insert(value(Array::new()));
            declarations
                .as_array_mut()
                .context("Expected a check-cfg array")?
                .push(format!(
                    "cfg(feature, values({}))",
                    removed_features
                        .iter()
                        .map(|feature| format!("{feature:?}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
        }
        Ok(())
    }
}

fn next_version(versions: &mut impl Iterator<Item = Result<Version, semver::Error>>) -> anyhow::Result<Version> {
    let mut minor = 0u64;
    for version in versions {
        let version = version?;
        if version.major == 0 {
            minor = minor.max(version.minor);
        }
    }
    Ok(Version::new(
        0,
        minor.checked_add(1).context("No remaining unstable minor version")?,
        0,
    ))
}

fn published_companion(ctx: &Context, package: &Package) -> anyhow::Result<Option<Version>> {
    let marker = format!("{MARKER_PREFIX}{}", package.version);
    ctx.crates_index
        .crate_(&package.name)?
        .iter()
        .flat_map(|entry| entry.versions())
        .filter(|version| !version.is_yanked() && version.features().contains_key(&marker))
        .try_fold(None, |latest, entry| {
            let version = Version::parse(entry.version())?;
            Ok(latest.max(Some(version)))
        })
}

fn dependency_table_matches(dependency: &cargo_metadata::Dependency, table: &str) -> bool {
    let kind = match dependency.kind {
        DependencyKind::Normal => "dependencies",
        DependencyKind::Development => "dev-dependencies",
        DependencyKind::Build => "build-dependencies",
        _ => return false,
    };
    match &dependency.target {
        Some(target) => {
            table
                .strip_prefix(kind)
                .and_then(|suffix| suffix.strip_prefix(" (")?.strip_suffix(')'))
                .and_then(|platform| platform.parse().ok())
                .as_ref()
                == Some(target)
        }
        None => table == kind,
    }
}

fn versioned_dev_dependency(
    ctx: &Context,
    package: &Package,
    dependency: &cargo_metadata::Dependency,
) -> anyhow::Result<bool> {
    if dependency.req != VersionReq::STAR {
        return Ok(true);
    }
    // Cargo metadata represents both an omitted version and an explicit "*" as STAR.
    let mut manifest = fs::read_to_string(&package.manifest_path)?.parse::<DocumentMut>()?;
    let alias = dependency.rename.as_deref().unwrap_or(&dependency.name);
    for (table, kind) in find_dependency_tables(&mut manifest) {
        if !dependency_table_matches(dependency, &kind) {
            continue;
        }
        let Some(entry) = table.get(alias) else { continue };
        if entry.get("workspace").and_then(Item::as_bool) == Some(true) {
            let workspace = fs::read_to_string(ctx.root.join("Cargo.toml"))?.parse::<DocumentMut>()?;
            return Ok(workspace["workspace"]["dependencies"][alias].get("version").is_some());
        }
        return Ok(entry.get("version").is_some());
    }
    Ok(false)
}

fn requests_experimental<'a>(
    dependency: &cargo_metadata::Dependency,
    target: &Package,
    forwarding: impl Iterator<Item = &'a str>,
) -> bool {
    let excluded = excluded_features(target);
    let alias = dependency.rename.as_deref().unwrap_or(&dependency.name);
    (dependency.uses_default_features && excluded.contains("default"))
        || dependency
            .features
            .iter()
            .any(|feature| excluded.contains(feature.as_str()))
        || forwarding
            .filter_map(|feature| feature.split_once('/'))
            .any(|(name, feature)| name.trim_end_matches('?') == alias && excluded.contains(feature))
}

fn excluded_features(package: &Package) -> BTreeSet<&str> {
    let mut excluded = BTreeSet::from([GATE]);
    if let Some(features) = package.metadata["smart-release"]["unstable-features"].as_array() {
        excluded.extend(features.iter().filter_map(serde_json::Value::as_str));
    }
    loop {
        let count = excluded.len();
        for (name, items) in &package.features {
            if items.iter().any(|item| excluded.contains(item.as_str())) {
                excluded.insert(name);
            }
        }
        if count == excluded.len() {
            return excluded;
        }
    }
}

fn strip_experimental_features(
    doc: &mut DocumentMut,
    package: &Package,
) -> anyhow::Result<(BTreeSet<String>, BTreeSet<String>)> {
    let mut excluded = excluded_features(package);
    excluded.remove("default");
    let features = doc["features"]
        .as_table_like_mut()
        .context("Expected a feature table")?;
    let mut removed_deps = BTreeSet::new();
    for name in &excluded {
        if let Some(items) = features.remove(name).as_ref().and_then(Item::as_array) {
            removed_deps.extend(
                items
                    .iter()
                    .filter_map(|item| optional_dependency(item.as_str()?))
                    .map(str::to_owned),
            );
        }
    }
    if let Some(default) = features.get_mut("default").and_then(Item::as_array_mut) {
        default.retain(|item| item.as_str().is_none_or(|item| !excluded.contains(item)));
    }
    for (name, items) in features.iter() {
        removed_deps.remove(name);
        for dependency in items
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|item| optional_dependency(item.as_str()?))
        {
            removed_deps.remove(dependency);
        }
    }
    if let Some(docs_features) = doc
        .get_mut("package")
        .and_then(|package| package.get_mut("metadata"))
        .and_then(|metadata| metadata.get_mut("docs"))
        .and_then(|docs| docs.get_mut("rs"))
        .and_then(|rs| rs.get_mut("features"))
        .and_then(Item::as_array_mut)
    {
        docs_features.retain(|item| item.as_str().is_none_or(|item| !excluded.contains(item)));
    }
    Ok((excluded.into_iter().map(str::to_owned).collect(), removed_deps))
}

fn optional_dependency(feature: &str) -> Option<&str> {
    feature
        .strip_prefix("dep:")
        .or_else(|| feature.split_once('/').map(|(name, _)| name.trim_end_matches('?')))
}

fn add_feature(doc: &mut DocumentMut, feature: &str, enables: &str) -> anyhow::Result<()> {
    let features = doc["features"]
        .as_table_like_mut()
        .context("Expected a feature table")?;
    let list = features.entry(feature).or_insert(value(Array::new()));
    let list = list.as_array_mut().context("Expected a feature array")?;
    if !list.iter().any(|item| item.as_str() == Some(enables)) {
        list.push(enables);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn companion_tags_share_the_checkout_used_to_package_the_companions() -> gix_testtools::Result {
        let fixture = tempfile::tempdir()?;
        let root = fixture.path();
        let workspace = "[workspace]\nmembers = [\"paired\", \"member\"]\nresolver = \"2\"\n";
        fs::write(root.join(".gitattributes"), "* -text\n")?;
        fs::write(root.join("Cargo.toml"), workspace)?;
        fs::write(root.join("README.md"), "Original source workspace\n")?;
        for name in ["paired", "member"] {
            fs::create_dir_all(root.join(name).join("src"))?;
            fs::write(
                root.join(name).join("Cargo.toml"),
                format!("[package]\nname = \"{name}\"\nversion = \"1.0.0\"\nedition = \"2021\"\n"),
            )?;
            fs::write(root.join(name).join("src/lib.rs"), "pub fn shared() {}\n")?;
            fs::write(root.join(name).join("notes.txt"), "Retain this source file in Git\n")?;
        }
        gix_testtools::git(root, "init")?;
        gix_testtools::git(root, "add .")?;
        gix_testtools::git(root, "commit -m 'stable release'")?;
        let repo = gix::open(root)?;
        let stable = repo.head_id()?;

        let staged = tempfile::tempdir()?;
        let releases = [("paired", "0.4.0"), ("member", "0.8.0")];
        let mut manifests = Vec::new();
        for (name, version) in releases {
            let stage = staged.path().join(name);
            fs::create_dir_all(stage.join("src"))?;
            let manifest = format!(
                "[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2021\"\ninclude = [\"/src/lib.rs\"]\n\n[features]\ndefault = [\"unstable\"]\nunstable = []\n\n[workspace]\n"
            );
            fs::write(stage.join("Cargo.toml"), &manifest)?;
            fs::write(stage.join("src/lib.rs"), "pub fn shared() {}\n")?;
            manifests.push((PathBuf::from(name).join("Cargo.toml"), stage.join("Cargo.toml")));
        }
        let paths: Vec<_> = manifests
            .iter()
            .map(|(path, staged)| (path.clone(), staged.as_path()))
            .collect();
        let child = source_commit(stable, &paths, "Prepare experimental packages", false)?;
        let checkout = staged.path().join("checkout");
        checkout_source(child, &checkout)?;
        assert_eq!(
            fs::read_to_string(checkout.join("README.md"))?,
            "Original source workspace\n"
        );
        assert_eq!(fs::read_to_string(checkout.join("Cargo.toml"))?, workspace);
        assert!(
            !checkout.join(".cargo-smart-release").exists(),
            "package archives have no place in the source tree"
        );

        for (name, version) in releases {
            let tag = format!("{name}/v{version}");
            repo.tag_reference(&tag, child, gix::refs::transaction::PreviousValue::MustNotExist)?;
            assert_eq!(
                repo.find_reference(format!("refs/tags/{tag}").as_str())?.peel_to_id()?,
                child
            );
            assert_eq!(
                gix_testtools::git(root, &format!("rev-parse {tag}^"))?.trim_end(),
                stable.to_string()
            );
            let manifest = checkout.join(name).join("Cargo.toml");
            assert_eq!(
                fs::read(&manifest)?,
                fs::read(staged.path().join(name).join("Cargo.toml"))?
            );
            assert!(checkout.join(name).join("notes.txt").is_file());
            let output = Command::new(env!("CARGO"))
                .args(["package", "--offline", "--list", "--manifest-path"])
                .arg(&manifest)
                .output()?;
            assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
            let files = std::str::from_utf8(&output.stdout)?;
            assert!(
                files.lines().any(|path| Path::new(path) == Path::new("src/lib.rs")),
                "{files}"
            );
            assert!(!files.lines().any(|path| path == "notes.txt"), "{files}");
        }
        assert_eq!(repo.head_id()?, stable);
        Ok(())
    }

    #[test]
    fn companion_uses_the_next_minor_across_the_entire_major_zero_history() -> anyhow::Result<()> {
        assert_eq!(
            next_version(
                &mut ["2.0.0", "0.4.12", "0.11.0-alpha.1", "0.7.5"]
                    .into_iter()
                    .map(Version::parse)
            )?,
            Version::new(0, 12, 0)
        );
        assert_eq!(next_version(&mut std::iter::empty())?, Version::new(0, 1, 0));
        Ok(())
    }
}
