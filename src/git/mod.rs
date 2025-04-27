use std::process::Command;

use anyhow::{anyhow, bail};
use cargo_metadata::{camino::Utf8Path, Package};
use gix::{
    bstr::{BStr, ByteSlice},
    object,
    refs::FullNameRef,
};

use crate::utils::{component_to_bytes, tag_name};

pub mod history;

#[derive(Clone, Debug)]
pub enum PackageChangeKind {
    Untagged {
        #[allow(dead_code)] // only used for debug printing maybe?
        wanted_tag_name: String,
    },
    ChangedOrNew,
}

pub fn change_since_last_release(package: &Package, ctx: &crate::Context) -> anyhow::Result<Option<PackageChangeKind>> {
    let version_tag_name = tag_name(package, &package.version, &ctx.repo);
    let mut tag_ref = match ctx.repo.try_find_reference(&version_tag_name)? {
        None => {
            return Ok(Some(PackageChangeKind::Untagged {
                wanted_tag_name: version_tag_name,
            }));
        }
        Some(r) => r,
    };
    let repo_relative_crate_dir = ctx.repo_relative_path(package);
    Ok(match ctx.repo.head()?.try_into_peeled_id()? {
        Some(current_commit) => {
            let released_target = tag_ref.peel_to_id_in_place()?;

            match repo_relative_crate_dir
                // If it's a top-level crate, use the src-directory for now
                // KEEP THIS IN SYNC with gix::create_ref_history()!
                .or_else(|| (ctx.meta.workspace_members.len() != 1).then(|| Utf8Path::new("src")))
            {
                None => (current_commit != released_target).then_some(PackageChangeKind::ChangedOrNew),
                Some(dir) => {
                    let components = dir.components().map(component_to_bytes);
                    let current_dir_id = current_commit
                        .object()?
                        .peel_to_kind(object::Kind::Tree)?
                        .into_tree()
                        .peel_to_entry(components.clone())?
                        .unwrap_or_else(|| panic!("path '{}' must exist in current_commit `{}`", dir, current_commit))
                        .object_id();
                    if let Some(released_dir_entry) = released_target
                        .object()?
                        .peel_to_kind(object::Kind::Tree)?
                        .into_tree()
                        .peel_to_entry(components)?
                    {
                        let released_dir_id = released_dir_entry.object_id();
                        (released_dir_id != current_dir_id).then_some(PackageChangeKind::ChangedOrNew)
                    } else {
                        log::warn!("Expected path '{dir}' to exist in the last released commit `{released_target}` but it doesn't. Was it moved from a different directory? The generated changelog may miss some changes.");
                        Some(PackageChangeKind::ChangedOrNew)
                    }
                }
            }
        }
        None => Some(PackageChangeKind::ChangedOrNew),
    })
}

pub fn assure_clean_working_tree() -> anyhow::Result<()> {
    let tracked_changed = !Command::new("git")
        .arg("diff")
        .arg("HEAD")
        .arg("--exit-code")
        .arg("--name-only")
        .status()?
        .success();
    if tracked_changed {
        bail!("Detected working tree changes. Please commit beforehand as otherwise these would be committed as part of manifest changes, or use --allow-dirty to force it.")
    }

    let untracked = Command::new("git")
        .arg("ls-files")
        .arg("--exclude-standard")
        .arg("--others")
        .output()?
        .stdout;
    if !untracked.trim().is_empty() {
        let err = anyhow!(gix::bstr::BString::from(untracked));
        return Err(err.context("Found untracked files which would possibly be packaged when publishing."));
    }
    Ok(())
}

pub fn remote_url(repo: &gix::Repository) -> anyhow::Result<Option<gix::Url>> {
    Ok(repo
        .head()?
        .into_remote(gix::remote::Direction::Push)
        .transpose()?
        .and_then(|r| r.url(gix::remote::Direction::Push).map(ToOwned::to_owned)))
}

pub fn author() -> anyhow::Result<gix::actor::Signature> {
    let stdout = Command::new("git").arg("var").arg("GIT_AUTHOR_IDENT").output()?.stdout;
    Ok(gix::actor::SignatureRef::from_bytes::<()>(&stdout)
        .ok()
        .ok_or_else(|| anyhow!("Could not parse author from GIT_AUTHOR_IDENT='{}'", stdout.as_bstr()))?
        .to_owned()?)
}

pub fn strip_tag_path(name: &FullNameRef) -> &BStr {
    try_strip_tag_path(name).expect("prefix iteration works")
}

pub fn try_strip_tag_path(name: &FullNameRef) -> Option<&BStr> {
    name.as_bstr().strip_prefix(b"refs/tags/").map(ByteSlice::as_bstr)
}
