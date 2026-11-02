use std::{fs, os::unix::fs::PermissionsExt, path::Path, process::Command};

use gix_testtools::{git, tempfile};

#[test]
fn release_tags_are_pushed_in_batches_and_github_releases_are_opt_in_for_members() -> gix_testtools::Result {
    for (packages, options, expected_tags, expected_releases) in [
        (
            vec!["tag-member-a", "tag-root", "tag-member-b"],
            vec![],
            vec![vec!["v0.1.0"], vec!["tag-member-a/v0.1.0", "tag-member-b/v0.1.0"]],
            vec!["v0.1.0"],
        ),
        (
            vec!["tag-root", "tag-member-a", "tag-member-b"],
            vec!["--changelog-github-release-for-namespaced-tags"],
            vec![vec!["v0.1.0"], vec!["tag-member-a/v0.1.0", "tag-member-b/v0.1.0"]],
            vec!["tag-member-a/v0.1.0", "tag-member-b/v0.1.0", "v0.1.0"],
        ),
        (
            vec!["tag-member-a", "tag-member-b"],
            vec![],
            vec![vec!["tag-member-a/v0.1.0", "tag-member-b/v0.1.0"]],
            vec![],
        ),
        (vec!["tag-root"], vec![], vec![vec!["v0.1.0"]], vec!["v0.1.0"]),
        (
            vec!["tag-root", "tag-member-a", "tag-member-b"],
            vec!["--no-changelog-github-release"],
            vec![vec!["v0.1.0"], vec!["tag-member-a/v0.1.0", "tag-member-b/v0.1.0"]],
            vec![],
        ),
    ] {
        let directory = fixture()?;
        let root = directory.path().join("worktree");
        let bin = directory.path().join("bin");
        let release_log = directory.path().join("github-releases");
        let command_path = std::env::join_paths(
            std::iter::once(bin).chain(std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())),
        )?;
        let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-smart-release"));
        gix_testtools::configure_git_environment(&mut command, &root);
        let output = command
            .current_dir(&root)
            .env("PATH", command_path)
            .env("GH_RELEASE_LOG", &release_log)
            .env("CARGO_HOME", directory.path().join("cargo-home"))
            .env("CARGO_NET_OFFLINE", "true")
            .args([
                "smart-release",
                "--execute",
                "--no-publish",
                "--bump",
                "keep",
                "--bump-dependencies",
                "keep",
                "--no-changelog-preview",
                "--no-changelog-links",
                "--allow-fully-generated-changelogs",
                "--allow-empty-release-message",
            ])
            .args(&packages)
            .args(&options)
            .output()?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{packages:?} {options:?}: {stderr}");

        let pushes = fs::read_to_string(directory.path().join("remote.git/pushes"))?;
        let batches: Vec<Vec<_>> = pushes
            .trim_end()
            .split("\n\n")
            .map(|batch| {
                let mut tags: Vec<_> = batch
                    .lines()
                    .filter_map(|line| line.strip_prefix("refs/tags/"))
                    .collect();
                tags.sort_unstable();
                tags
            })
            .collect();
        assert_eq!(batches, expected_tags, "{packages:?} {options:?}: {pushes}\n{stderr}");
        assert!(
            pushes.lines().any(|line| line == "refs/heads/main"),
            "the release commit is pushed"
        );
        let remote_tags = git(directory.path().join("remote.git"), "tag --list")?;
        let expected_remote_tags = expected_tags
            .into_iter()
            .flatten()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(
            remote_tags.lines().collect::<std::collections::BTreeSet<_>>(),
            expected_remote_tags,
            "push.followTags must not push unrelated or premature tags"
        );
        let releases = fs::read_to_string(&release_log)?;
        let mut release_tags: Vec<_> = releases.lines().collect();
        release_tags.sort_unstable();
        assert_eq!(release_tags, expected_releases, "{packages:?} {options:?}: {stderr}");

        let released_head = git(&root, "rev-parse HEAD")?;
        let unchanged = command.output()?;
        let stderr = String::from_utf8_lossy(&unchanged.stderr);
        assert!(!unchanged.status.success(), "{stderr}");
        assert!(stderr.contains("There is no crate eligible for publishing"), "{stderr}");
        assert_eq!(
            git(&root, "rev-parse HEAD")?,
            released_head,
            "new tags identify unchanged releases"
        );
        assert_eq!(fs::read_to_string(directory.path().join("remote.git/pushes"))?, pushes);
        assert_eq!(fs::read_to_string(release_log)?, releases);
    }
    Ok(())
}

fn fixture() -> gix_testtools::Result<tempfile::TempDir> {
    let directory = tempfile::tempdir()?;
    let root = directory.path().join("worktree");
    fs::create_dir(&root)?;
    for (path, name) in [
        ("", "tag-root"),
        ("member-a", "tag-member-a"),
        ("member-b", "tag-member-b"),
    ] {
        let package = root.join(path);
        fs::create_dir_all(package.join("src"))?;
        fs::write(package.join("src/lib.rs"), "pub fn initial() {}\n")?;
        let workspace = if path.is_empty() {
            "\n[workspace]\nmembers = [\"member-a\", \"member-b\"]\n"
        } else {
            ""
        };
        fs::write(
            package.join("Cargo.toml"),
            format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n{workspace}"),
        )?;
    }
    fs::write(root.join(".gitignore"), "/target/\n/Cargo.lock\n")?;
    git(&root, "init -q -b main")?;
    git(&root, "add .")?;
    git(&root, "commit -qm 'feat: initial release'")?;
    git(&root, "tag -am unrelated unrelated")?;
    git(directory.path(), "init -q --bare remote.git")?;
    git(&root, "remote add origin ../remote.git")?;
    git(&root, "config branch.main.remote origin")?;
    git(&root, "config branch.main.merge refs/heads/main")?;
    git(&root, "config push.followTags true")?;
    executable(
        &directory.path().join("remote.git/hooks/pre-receive"),
        "#!/bin/sh\nwhile read old new ref; do printf '%s\\n' \"$ref\"; done >> \"$GIT_DIR/pushes\"\nprintf '\\n' >> \"$GIT_DIR/pushes\"\n",
    )?;
    fs::create_dir(directory.path().join("bin"))?;
    executable(
        &directory.path().join("bin/gh"),
        "#!/bin/sh\nif test \"$1\" = release; then printf '%s\\n' \"$3\" >> \"$GH_RELEASE_LOG\"; fi\n",
    )?;
    fs::write(directory.path().join("github-releases"), "")?;
    Ok(directory)
}

fn executable(path: &Path, content: &str) -> std::io::Result<()> {
    fs::write(path, content)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o755))
}
