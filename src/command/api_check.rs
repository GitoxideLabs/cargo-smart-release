use anyhow::bail;

use crate::{api::Checks, traverse, utils::package_by_name, version::BumpSpec, Context};

/// Release-planning settings shared with `cargo smart-release`.
pub struct Options {
    pub allow_auto_publish_of_stable_crates: bool,
    pub bump_when_needed: bool,
    pub update_crates_index: bool,
}

/// Check release candidates without entering the release mutation phase.
pub fn release_plan(
    crates: Vec<String>,
    bump: BumpSpec,
    bump_dependencies: BumpSpec,
    opts: Options,
) -> anyhow::Result<bool> {
    let ctx = Context::with_metadata(
        cargo_smart_release_core::api_check::metadata(None, &[])?,
        crates,
        false,
        bump,
        bump_dependencies,
    )?;
    let mut checks = Checks::new(&ctx, opts.allow_auto_publish_of_stable_crates)?;
    for name in &ctx.crate_names {
        let package = package_by_name(&ctx.meta, name)?;
        if !checks.has_policy(package) {
            bail!("'{name}' has no enabled release API-check policy in package.metadata.smart-release");
        }
    }
    if opts.update_crates_index {
        ctx.crates_index.update();
    } else {
        log::info!("Using cached crates.io versions; --update-crates-index refreshes release baselines");
    }
    let traversal = traverse::Options {
        allow_auto_publish_of_stable_crates: opts.allow_auto_publish_of_stable_crates,
        bump_when_needed: opts.bump_when_needed,
        isolate_dependencies_from_breaking_changes: true,
        traverse_graph: true,
    };
    let crates = traverse::dependencies_with_checks(&ctx, traversal, &mut checks)?;
    let crates = traverse::assure_crates_index_is_uptodate(crates, &ctx, traversal, &mut checks)?;
    for dep in &crates {
        let package = dep.package;
        if let Some(bump) = dep.mode.version_adjustment_bump() {
            log::info!(
                "Release candidate {}: {} -> {}",
                package.name,
                package.version,
                bump.next_release
            );
        }
        if checks.has_policy(package) {
            log::info!(
                "Stable API of '{}': {}",
                package.name,
                if checks.incompatible.contains(&package.id) {
                    "incompatible; requires a breaking release"
                } else {
                    "compatible; inherited breaking bumps stop at this boundary"
                }
            );
        }
    }
    Ok(checks.incompatible.is_empty())
}
