use std::collections::BTreeSet;

use cargo_metadata::{DependencyKind, Package, PackageId};

use crate::{
    git,
    traverse::dependency::{ManifestAdjustment, VersionAdjustment},
    utils::{
        is_pre_release_version, package_by_id, package_by_name, package_eq_dependency_ignore_dev_without_version,
        workspace_package_by_dependency,
    },
    version,
    version::{Bump, BumpSpec},
    Context,
};

pub mod dependency {
    use crate::{git, version};

    /// Skipped crates are always dependent ones
    #[derive(Copy, Clone, Debug, PartialOrd, Ord, Eq, PartialEq)]
    pub enum NoPublishReason {
        Unchanged,
        DeniedAutopublishOfProductionCrate,
        PublishDisabledInManifest,
        BreakingChangeCausesManifestUpdate,
    }

    impl std::fmt::Display for NoPublishReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                NoPublishReason::PublishDisabledInManifest => "disabled",
                NoPublishReason::DeniedAutopublishOfProductionCrate => "denied",
                NoPublishReason::Unchanged => "unchanged",
                NoPublishReason::BreakingChangeCausesManifestUpdate => "dep-breaking",
            })
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Kind {
        /// Initially selected by user
        UserSelection,
        /// A changed dependency of the user selected crate that thus needs publishing
        DependencyOrDependentOfUserSelection,
    }

    #[derive(Clone, Debug)]
    pub enum VersionAdjustment {
        /// The crate changed directly or indirectly and should see a version change.
        Changed {
            /// The direct git change detected for this crate, if any.
            ///
            /// This stays `None` when the version adjustment is only propagated from
            /// elsewhere in the graph.
            change: Option<git::PackageChangeKind>,
            bump: version::Bump,
        },
        /// One of the crates dependencies signalled breaking changes, and is published because of that.
        Breakage {
            bump: version::Bump,
            /// Set if there is a change at all, which might not be the case for previously skipped crates.
            #[allow(dead_code)] // strange that this isn't used, indeed!
            change: Option<git::PackageChangeKind>,
            /// The direct dependency causing the breakage because it's breaking itself
            causing_dependency_names: Vec<String>,
        },
    }

    impl VersionAdjustment {
        pub fn bump(&self) -> &version::Bump {
            match self {
                VersionAdjustment::Breakage { bump, .. } | VersionAdjustment::Changed { bump, .. } => bump,
            }
        }
    }

    #[allow(clippy::large_enum_variant)]
    #[derive(Clone, Debug)]
    pub enum ManifestAdjustment {
        DueToDependencyChange,
        Version(VersionAdjustment),
    }

    #[derive(Clone, Debug)]
    pub enum Mode {
        ToBePublished {
            adjustment: VersionAdjustment,
        },
        /// Won't be published but manifest might have to be fixed if a version bump is present.
        NotForPublishing {
            reason: NoPublishReason,
            adjustment: Option<ManifestAdjustment>,
        },
    }

    impl Mode {
        pub fn manifest_will_change(&self) -> bool {
            matches!(
                self,
                Mode::ToBePublished { .. }
                    | Mode::NotForPublishing {
                        adjustment: Some(_),
                        ..
                    }
            )
        }
        pub fn safety_bump(&self) -> Option<&version::Bump> {
            match self {
                Mode::ToBePublished { adjustment }
                | Mode::NotForPublishing {
                    adjustment: Some(ManifestAdjustment::Version(adjustment)),
                    ..
                } => match adjustment {
                    VersionAdjustment::Breakage { bump, .. } => Some(bump),
                    VersionAdjustment::Changed { .. } => None,
                },
                _ => None,
            }
        }
        pub fn version_adjustment_bump(&self) -> Option<&version::Bump> {
            match self {
                Mode::ToBePublished { adjustment }
                | Mode::NotForPublishing {
                    adjustment: Some(ManifestAdjustment::Version(adjustment)),
                    ..
                } => Some(adjustment.bump()),
                _ => None,
            }
        }
    }
}

#[derive(Clone)]
pub struct Dependency<'meta> {
    pub package: &'meta Package,
    pub kind: dependency::Kind,
    pub mode: dependency::Mode,
}

impl std::fmt::Debug for Dependency<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("Dependency");
        dbg.field("package", &self.package.id.repr);
        dbg.field("kind", &self.kind);
        dbg.field("mode", &self.mode);
        dbg.finish()
    }
}

pub struct Options {
    pub allow_auto_publish_of_stable_crates: bool,
    pub bump_when_needed: bool,
    pub isolate_dependencies_from_breaking_changes: bool,
    pub traverse_graph: bool,
}

pub fn dependencies(
    ctx: &Context,
    Options {
        allow_auto_publish_of_stable_crates,
        bump_when_needed,
        isolate_dependencies_from_breaking_changes,
        traverse_graph,
    }: Options,
) -> anyhow::Result<Vec<Dependency<'_>>> {
    let mut seen = BTreeSet::new();
    let mut crates = Vec::new();
    for crate_name in &ctx.crate_names {
        let mut crates_this_round = Vec::new();
        let package = package_by_name(&ctx.meta, crate_name)?;
        if seen.contains(&&package.id) {
            continue;
        }
        if traverse_graph {
            depth_first_traversal(
                ctx,
                &mut seen,
                &mut crates_this_round,
                package,
                allow_auto_publish_of_stable_crates,
                bump_when_needed,
            )?;
        }

        match git::change_since_last_release(package, ctx)? {
            Some(user_package_change) => {
                crates_this_round.push(Dependency {
                    package,
                    kind: dependency::Kind::UserSelection,
                    mode: if package_may_be_published(package) {
                        dependency::Mode::ToBePublished {
                            adjustment: VersionAdjustment::Changed {
                                change: Some(user_package_change),
                                bump: version::bump_package(package, ctx, bump_when_needed)?,
                            },
                        }
                    } else {
                        dependency::Mode::NotForPublishing {
                            reason: dependency::NoPublishReason::PublishDisabledInManifest,
                            adjustment: None,
                        }
                    },
                });
                seen.insert(&package.id);
            }
            None => {
                crates_this_round.push(Dependency {
                    package,
                    kind: dependency::Kind::UserSelection,
                    mode: dependency::Mode::NotForPublishing {
                        reason: dependency::NoPublishReason::Unchanged,
                        adjustment: None,
                    },
                });
            }
        }
        merge_crates(&mut crates, crates_this_round);
    }

    if isolate_dependencies_from_breaking_changes {
        forward_propagate_breaking_changes_for_publishing(
            ctx,
            &mut crates,
            bump_when_needed,
            allow_auto_publish_of_stable_crates,
        )?;
        forward_propagate_breaking_changes_for_manifest_updates(
            ctx,
            &mut crates,
            bump_when_needed,
            allow_auto_publish_of_stable_crates,
        )?;
    }
    adjust_workspace_crates_depending_on_adjusted_crates(ctx, &mut crates, bump_when_needed)?;
    Ok(crates)
}

fn merge_crates<'meta>(dest: &mut Vec<Dependency<'meta>>, src: Vec<Dependency<'meta>>) {
    if dest.is_empty() {
        *dest = src;
    } else {
        for dep in src {
            if !dest.iter().any(|dest| dest.package.id == dep.package.id) {
                dest.push(dep);
            }
        }
    }
}

fn forward_propagate_breaking_changes_for_manifest_updates<'meta>(
    ctx: &'meta Context,
    crates: &mut Vec<Dependency<'meta>>,
    bump_when_needed: bool,
    allow_auto_publish_of_stable_crates: bool,
) -> anyhow::Result<()> {
    let mut non_publishing_crates_with_safety_bumps = Vec::new();
    let mut backing = crates
        .iter()
        .filter(
            |c| matches!(&c.mode, dependency::Mode::ToBePublished { adjustment } if adjustment.bump().is_breaking()),
        )
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let workspace_packages: Vec<_> = ctx
        .meta
        .workspace_members
        .iter()
        .map(|wmid| package_by_id(&ctx.meta, wmid))
        .filter(|p| package_may_be_published(p)) // will publish, non-publishing ones need no safety bumps
        .collect();
    let mut set_to_expand_from = &backing;
    let mut seen = BTreeSet::default();
    loop {
        let mut new_crates_this_round = Vec::<Dependency<'_>>::new();
        for dependee in set_to_expand_from {
            for dependant in workspace_packages.iter().filter(|p| {
                p.dependencies
                    .iter()
                    .any(|dep| package_eq_dependency_ignore_dev_without_version(dependee.package, dep))
            }) {
                if seen.contains(&&dependant.id) {
                    continue;
                }
                seen.insert(&dependant.id);
                let bump = breaking_version_bump(ctx, dependant, bump_when_needed)?;
                if bump.next_release_changes_manifest() {
                    // Propagate a breaking dependency bump to an already-known dependant,
                    // but only upgrade entries that were explicitly selected by the user
                    // and previously stayed unchanged. Any crate that already carries its
                    // own adjustment is left alone so this pass doesn't overwrite an
                    // earlier decision with a weaker or conflicting one.
                    if let Some(existing_idx) = crates.iter().position(|c| c.package.id == dependant.id) {
                        let existing = &mut crates[existing_idx];
                        match (&existing.kind, &existing.mode) {
                            (
                                dependency::Kind::UserSelection,
                                dependency::Mode::NotForPublishing {
                                    reason: dependency::NoPublishReason::Unchanged,
                                    adjustment: None,
                                },
                            ) if is_pre_release_version(&dependant.version) || allow_auto_publish_of_stable_crates => {
                                existing.mode = dependency::Mode::ToBePublished {
                                    adjustment: VersionAdjustment::Breakage {
                                        bump,
                                        change: None,
                                        causing_dependency_names: vec![dependee.package.name.to_string()],
                                    },
                                };
                            }
                            _ => {
                                let is_breaking =
                                    existing.mode.version_adjustment_bump().is_some_and(Bump::is_breaking);
                                if !is_breaking {
                                    log::debug!(
                                        "Wanted to mark '{}' for breaking manifest change, but its already known without breaking change.",
                                        dependant.name
                                    );
                                }
                            }
                        }
                    } else if is_pre_release_version(&dependant.version) || allow_auto_publish_of_stable_crates {
                        let kind = if ctx.crate_names.contains(&dependant.name) {
                            dependency::Kind::UserSelection
                        } else {
                            dependency::Kind::DependencyOrDependentOfUserSelection
                        };
                        let adjustment = VersionAdjustment::Breakage {
                            bump,
                            change: None,
                            causing_dependency_names: vec![dependee.package.name.to_string()],
                        };
                        new_crates_this_round.push(Dependency {
                            package: dependant,
                            kind,
                            mode: match kind {
                                dependency::Kind::UserSelection => dependency::Mode::ToBePublished { adjustment },
                                dependency::Kind::DependencyOrDependentOfUserSelection => {
                                    dependency::Mode::NotForPublishing {
                                        reason: dependency::NoPublishReason::BreakingChangeCausesManifestUpdate,
                                        adjustment: Some(ManifestAdjustment::Version(adjustment)),
                                    }
                                }
                            },
                        });
                    } else {
                        log::trace!(
                            "Ignored stable crate '{}' despite being eligible for safety bump and manifest change.",
                            dependant.name
                        );
                    }
                }
            }
        }

        if new_crates_this_round.is_empty() {
            break;
        }
        non_publishing_crates_with_safety_bumps.extend(new_crates_this_round.iter().cloned());
        backing = new_crates_this_round;
        set_to_expand_from = &backing;
    }
    crates.extend(non_publishing_crates_with_safety_bumps);
    Ok(())
}

fn package_may_be_published(p: &Package) -> bool {
    p.publish.is_none()
}

fn forward_propagate_breaking_changes_for_publishing(
    ctx: &Context,
    crates: &mut [Dependency<'_>],
    bump_when_needed: bool,
    allow_auto_publish_of_stable_crates: bool,
) -> anyhow::Result<()> {
    let mut previous_edits = Vec::new();
    loop {
        let mut seen_this_round = BTreeSet::default();
        let mut edits = Vec::new();
        // skipped don't have version bumps, we don't have manifest updates yet
        for (idx, starting_crate_for_backward_search) in crates
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, c)| matches!(c.mode, dependency::Mode::ToBePublished { .. }))
        {
            find_safety_bump_edits_backwards_from_crates_for_publish(
                crates,
                (idx, starting_crate_for_backward_search),
                &mut seen_this_round,
                &mut edits,
            );
            seen_this_round.insert(idx);
        }

        if edits == previous_edits {
            break;
        }

        previous_edits.clone_from(&edits);
        for edit_for_publish in edits {
            edit_for_publish.apply(crates, ctx, bump_when_needed, allow_auto_publish_of_stable_crates)?;
        }
    }
    Ok(())
}

#[derive(PartialEq, Eq, Clone)]
struct EditForPublish {
    crates_idx: usize,
    causing_dependency_indices: Vec<usize>,
}

impl EditForPublish {
    fn from(idx: usize, causing_dependency_indices: Vec<usize>) -> Self {
        EditForPublish {
            crates_idx: idx,
            causing_dependency_indices,
        }
    }

    fn apply(
        self,
        crates: &mut [Dependency<'_>],
        ctx: &Context,
        bump_when_needed: bool,
        allow_auto_publish_of_stable_crates: bool,
    ) -> anyhow::Result<()> {
        let causing_dependency_names = self
            .causing_dependency_indices
            .into_iter()
            .map(|idx| crates[idx].package.name.to_string())
            .collect();
        let dep_mut = &mut crates[self.crates_idx];
        if is_pre_release_version(&dep_mut.package.version) || allow_auto_publish_of_stable_crates {
            let breaking_bump = breaking_version_bump(ctx, dep_mut.package, bump_when_needed)?;
            match &mut dep_mut.mode {
                dependency::Mode::NotForPublishing {
                    adjustment: maybe_adjustment,
                    ..
                } => {
                    let adjustment = match maybe_adjustment.take() {
                        Some(ManifestAdjustment::DueToDependencyChange) => {
                            unreachable!("BUG: code generating these runs later")
                        }
                        Some(ManifestAdjustment::Version(mut adjustment)) => {
                            make_breaking(&mut adjustment, breaking_bump, causing_dependency_names);
                            adjustment
                        }
                        None => VersionAdjustment::Breakage {
                            bump: breaking_bump,
                            causing_dependency_names,
                            change: None,
                        },
                    };
                    dep_mut.mode = dependency::Mode::ToBePublished { adjustment };
                }
                dependency::Mode::ToBePublished { adjustment, .. } => {
                    make_breaking(adjustment, breaking_bump, causing_dependency_names);
                }
            }
        } else {
            log::trace!(
                "Ignored stable crate '{}' despite being eligible for safety bump and publishing.",
                dep_mut.package.name
            );
        }
        Ok(())
    }
}

fn breaking_version_bump(ctx: &Context, package: &Package, bump_when_needed: bool) -> anyhow::Result<Bump> {
    let breaking_spec = if is_pre_release_version(&package.version) {
        BumpSpec::Minor
    } else {
        BumpSpec::Major
    };
    version::bump_package_with_spec(package, breaking_spec, ctx, bump_when_needed)
}

fn make_breaking(adjustment: &mut VersionAdjustment, breaking_bump: Bump, breaking_crate_names: Vec<String>) {
    match adjustment {
        VersionAdjustment::Breakage { .. } => {}
        VersionAdjustment::Changed { change, bump } => {
            bump.next_release = breaking_bump.next_release;
            *adjustment = VersionAdjustment::Breakage {
                bump: bump.clone(),
                change: change.clone(),
                causing_dependency_names: breaking_crate_names,
            };
        }
    }
}

fn find_safety_bump_edits_backwards_from_crates_for_publish(
    crates: &[Dependency<'_>],
    start: (usize, &Dependency<'_>),
    seen: &mut BTreeSet<usize>,
    edits: &mut Vec<EditForPublish>,
) -> Vec<usize> {
    let (current_idx, current) = start;
    let mut breaking_indices = Vec::new();
    for (dep_idx, dep) in current.package.dependencies.iter().filter_map(|dep| {
        crates
            .iter()
            .enumerate()
            .find(|(_, c)| package_eq_dependency_ignore_dev_without_version(c.package, dep))
    }) {
        if seen.contains(&dep_idx) {
            continue;
        }
        match dep.mode.version_adjustment_bump() {
            Some(dep_bump) if dep_bump.is_breaking() => {
                if !edits.iter().any(|e| e.crates_idx == current_idx) {
                    edits.push(EditForPublish::from(current_idx, vec![dep_idx]));
                }
                if !breaking_indices.contains(&dep_idx) {
                    breaking_indices.push(dep_idx);
                }
            }
            _ => {
                seen.insert(dep_idx);
                let breaking_package_indices =
                    find_safety_bump_edits_backwards_from_crates_for_publish(crates, (dep_idx, dep), seen, edits);
                if !breaking_package_indices.is_empty() {
                    if !edits.iter().any(|e| e.crates_idx == current_idx) {
                        edits.push(EditForPublish::from(current_idx, breaking_package_indices.clone()));
                    }
                    for idx in breaking_package_indices {
                        if !breaking_indices.contains(&idx) {
                            breaking_indices.push(idx);
                        }
                    }
                }
            }
        }
    }
    breaking_indices
}

fn depth_first_traversal<'meta>(
    ctx: &'meta Context,
    seen: &mut BTreeSet<&'meta PackageId>,
    crates: &mut Vec<Dependency<'meta>>,
    root: &Package,
    allow_auto_publish_of_stable_crates: bool,
    bump_when_needed: bool,
) -> anyhow::Result<()> {
    for workspace_dependency in root
        .dependencies
        .iter()
        .filter(|d| d.kind == DependencyKind::Normal)
        .filter_map(|d| workspace_package_by_dependency(&ctx.meta, d))
    {
        if seen.contains(&&workspace_dependency.id) {
            continue;
        }
        seen.insert(&workspace_dependency.id);
        depth_first_traversal(
            ctx,
            seen,
            crates,
            workspace_dependency,
            allow_auto_publish_of_stable_crates,
            bump_when_needed,
        )?;

        crates.push(match git::change_since_last_release(workspace_dependency, ctx)? {
            Some(change) => {
                if is_pre_release_version(&workspace_dependency.version) || allow_auto_publish_of_stable_crates {
                    Dependency {
                        package: workspace_dependency,
                        kind: dependency::Kind::DependencyOrDependentOfUserSelection,
                        mode: dependency::Mode::ToBePublished {
                            adjustment: VersionAdjustment::Changed {
                                change: Some(change),
                                bump: version::bump_package(workspace_dependency, ctx, bump_when_needed)?,
                            },
                        },
                    }
                } else {
                    Dependency {
                        package: workspace_dependency,
                        kind: dependency::Kind::DependencyOrDependentOfUserSelection,
                        mode: dependency::Mode::NotForPublishing {
                            reason: dependency::NoPublishReason::DeniedAutopublishOfProductionCrate,
                            adjustment: None,
                        },
                    }
                }
            }
            None => Dependency {
                package: workspace_dependency,
                kind: dependency::Kind::DependencyOrDependentOfUserSelection,
                mode: dependency::Mode::NotForPublishing {
                    reason: dependency::NoPublishReason::Unchanged,
                    adjustment: None,
                },
            },
        });
    }
    Ok(())
}

/// Ensure workspace crates that depend on crates whose own release version is
/// changing are included as well, so dependency declarations remain
/// consistent with the version updates that will be applied elsewhere in the
/// workspace.
///
/// To do that, `ctx` provides the workspace membership and dependency graph,
/// and `crates` is updated in place with any newly affected packages.
///
/// `bump_when_needed` matters when an existing entry must be promoted from
/// unchanged to publishable: instead of always computing a fresh bump, it
/// allows the promotion to keep the version already present in the manifest
/// when that version is already ahead of the latest published release and is
/// sufficient for the required change.
fn adjust_workspace_crates_depending_on_adjusted_crates<'meta>(
    ctx: &'meta Context,
    crates: &mut Vec<Dependency<'meta>>,
    bump_when_needed: bool,
) -> anyhow::Result<()> {
    loop {
        let version_adjusted_workspace_crates: Vec<_> = crates
            .iter()
            .filter(|c| c.mode.version_adjustment_bump().is_some())
            .map(|c| c.package)
            .collect();
        let mut changed = false;

        for wsp in ctx.meta.workspace_members.iter().map(|id| package_by_id(&ctx.meta, id)) {
            let depends_on_adjusted_crate = wsp.dependencies.iter().any(|dependency| {
                version_adjusted_workspace_crates
                    .iter()
                    .any(|adjusted| package_eq_dependency_ignore_dev_without_version(adjusted, dependency))
            });
            if !depends_on_adjusted_crate {
                continue;
            }

            match crates.iter_mut().find(|c| c.package.id == wsp.id) {
                Some(existing) => {
                    changed |= maybe_promote_selected_dependency(existing, ctx, bump_when_needed)?;
                }
                None => {
                    crates.push(Dependency {
                        kind: dependency::Kind::DependencyOrDependentOfUserSelection,
                        package: wsp,
                        mode: dependency::Mode::NotForPublishing {
                            adjustment: ManifestAdjustment::DueToDependencyChange.into(),
                            reason: dependency::NoPublishReason::Unchanged,
                        },
                    });
                    changed = true;
                }
            }
        }

        if !changed {
            break;
        }
    }
    Ok(())
}

/// Promote `dependency` from `NotForPublishing` to `ToBePublished` only when
/// it already exists in the traversal result but is still considered
/// effectively untouched: the crate must have been a direct `UserSelection`,
/// still carry the `Unchanged` reason, have no prior manifest or version
/// adjustment recorded, and remain publishable according to its manifest.
///
/// If all of these conditions hold, the function computes a version adjustment
/// with `bump_when_needed` and returns `true`; otherwise it leaves the entry as
/// is and returns `false`.
fn maybe_promote_selected_dependency(
    dependency: &mut Dependency<'_>,
    ctx: &Context,
    bump_when_needed: bool,
) -> anyhow::Result<bool> {
    match &mut dependency.mode {
        dependency::Mode::ToBePublished { .. } => Ok(false),
        dependency::Mode::NotForPublishing { reason, adjustment } => {
            if dependency.kind == dependency::Kind::UserSelection
                && adjustment.is_none()
                && *reason == dependency::NoPublishReason::Unchanged
                && package_may_be_published(dependency.package)
            {
                dependency.mode = dependency::Mode::ToBePublished {
                    adjustment: VersionAdjustment::Changed {
                        change: None,
                        bump: version::bump_package(dependency.package, ctx, bump_when_needed)?,
                    },
                };
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}
