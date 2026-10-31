use std::{collections::BTreeSet, path::PathBuf};

use anyhow::Context as _;
use cargo_smart_release::command;
use cargo_smart_release_core::api_check;

#[derive(clap::Args)]
#[command(args_conflicts_with_subcommands = true, subcommand_negates_reqs = true)]
pub struct Args {
    /// Workspace crate to audit and compare with its latest supported release tag.
    #[arg(required = true, value_name = "CRATE")]
    krate: Option<String>,
    /// Workspace or package manifest (defaults to Cargo's current-directory discovery).
    #[arg(long, conflicts_with = "release_plan")]
    manifest_path: Option<PathBuf>,
    /// Compare with this Git revision, including an ungated predecessor.
    #[arg(long, conflicts_with = "release_plan")]
    baseline: Option<String>,
    /// Use smart-release's planner and configured checkers, including proposed dependency bumps.
    #[arg(long)]
    release_plan: bool,
    /// Release bump: auto (default), keep, patch, minor, or major.
    #[arg(long, requires = "release_plan")]
    bump: Option<String>,
    /// Dependency bump: auto (default), keep, patch, minor, or major.
    #[arg(long, requires = "release_plan")]
    bump_dependencies: Option<String>,
    /// Apply the specified bumps even when the manifest version is unpublished.
    #[arg(long, requires = "release_plan")]
    no_bump_on_demand: bool,
    /// Include stable dependencies and dependents in the release plan.
    #[arg(long, requires = "release_plan")]
    auto_publish_of_stable_crates: bool,
    /// Refresh the crates.io index before release planning.
    #[arg(long, requires = "release_plan")]
    update_crates_index: bool,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Read a smart-release JSON request from stdin and check compatibility.
    Release {
        /// Use this baseline checkout instead of archiving the requested revision.
        #[arg(long)]
        baseline_root: Option<PathBuf>,
    },
    /// Audit supported profiles against the manifest's unstable dependency exceptions.
    Boundary { manifest: PathBuf },
    /// Print public dependencies of an existing rustdoc JSON document.
    Audit {
        json: PathBuf,
        /// Optional comma-separated allowlist of dependency crate names.
        #[arg(value_name = "CRATE,...")]
        allowed: Option<String>,
    },
}

pub fn run(args: Args) -> ! {
    super::init_logging(false);
    let result = execute(args);
    std::process::exit(match result {
        Ok(true) => 0,
        Ok(false) => 10,
        Err(err) => {
            eprintln!("API check could not complete: {err:#}");
            2
        }
    });
}

fn execute(args: Args) -> anyhow::Result<bool> {
    if let Some(command) = args.command {
        return match command {
            Command::Release { baseline_root } => {
                let request = serde_json::from_reader(std::io::stdin().lock())
                    .context("Invalid API check request on stdin")?;
                Ok(api_check::release(&request, baseline_root.as_deref())?.is_none())
            }
            Command::Boundary { manifest } => Ok(api_check::boundary(&manifest)?.is_none()),
            Command::Audit { json, allowed } => {
                let allowed = allowed.map(|allowed| allowed.split(',').map(str::to_owned).collect::<BTreeSet<_>>());
                let audit = api_check::audit(&json, allowed.as_ref())?;
                println!("{}", serde_json::to_string_pretty(&audit.dependencies)?);
                Ok(audit.disallowed.is_empty())
            }
        };
    }
    let krate = args.krate.context("Provide a workspace crate name or a plumbing subcommand")?;
    if args.release_plan {
        return command::api_check::release_plan(
            vec![krate],
            super::to_bump_spec(args.bump.as_deref().unwrap_or(super::DEFAULT_BUMP_SPEC))?,
            super::to_bump_spec(args.bump_dependencies.as_deref().unwrap_or(super::DEFAULT_BUMP_SPEC))?,
            command::api_check::Options {
                allow_auto_publish_of_stable_crates: args.auto_publish_of_stable_crates,
                bump_when_needed: !args.no_bump_on_demand,
                update_crates_index: args.update_crates_index,
            },
        );
    }
    Ok(api_check::check(&krate, args.manifest_path.as_deref(), args.baseline.as_deref())?.is_none())
}
