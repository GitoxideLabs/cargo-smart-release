use std::{fs, path::Path, process::Command};

use gix_testtools::{git, tempfile};

const FUZZ_MANIFEST: &str = r#"[package]
name = "release-test-fuzz"
version = "0.0.0"
publish = false

[package.metadata]
cargo-fuzz = true

[workspace]
members = ["."]

[dependencies]
renamed = { package = "release-test", path = "../../release", version = "0.8.0" } # keep this comment

[build-dependencies.release-test]
path = "../../release"
version = "0.8.0" # and this one

[dev-dependencies]
versionless = { package = "release-test", path = "../../release" }

[target.'cfg(unix)'.dependencies.release-test]
path = "../../release"
version = "0.8.0"
"#;

const TOOLS_MANIFEST: &str = r#"[workspace]

[workspace.dependencies]
renamed = { package = "release-test", path = "../release", version = "0.8.0" }
"#;

const UNRELATED_MANIFEST: &str = r#"[package]
name = "unrelated"
version = "0.8.0"

[dependencies]
registry = { package = "release-test", version = "0.8.0" }
other = { package = "release-test", path = "../other", version = "0.8.0" }
versionless = { package = "release-test", path = "../release" }
wrong-name = { path = "../release", version = "0.8.0" }
"#;

#[test]
fn stable_major_bump_requests_are_reported_before_dry_run_and_execution() -> gix_testtools::Result {
    for args in [vec![], vec!["--execute"]] {
        let dir = fixture()?;
        let root = dir.path();
        let manifest = root.join("release/Cargo.toml");
        fs::write(&manifest, fs::read_to_string(&manifest)?.replace("0.8.0", "1.2.3"))?;
        git(root, "commit -am 'stabilize the release crate'")?;
        let head = git(root, "rev-parse HEAD")?;

        let output = release(root, "major", &args)?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stderr}");
        assert!(stderr.contains("Stable-crate breaking-change report:"), "{stderr}");
        assert!(
            stderr.contains("'release-test' 1.2.3 -> 2.0.0 (publish): requested --bump major"),
            "{stderr}"
        );
        if args.is_empty() {
            assert_eq!(git(root, "rev-parse HEAD")?, head);
            assert!(git(root, "diff HEAD --")?.is_empty());
        } else {
            assert!(fs::read_to_string(&manifest)?.contains("version = \"2.0.0\""));
        }
    }
    Ok(())
}

#[test]
fn stable_breaking_releases_are_reported_when_the_manifest_is_already_bumped() -> gix_testtools::Result {
    let dir = fixture()?;
    let root = dir.path();
    let manifest = root.join("release/Cargo.toml");
    fs::write(&manifest, fs::read_to_string(&manifest)?.replace("0.8.0", "2.0.0"))?;
    git(root, "commit -am 'pre-bump the release crate'")?;
    let cargo_home = root.join("cargo-home");
    let (index_path, _) = crates_index::local_path_and_canonical_url_with_hash_kind(
        crates_index::sparse::URL,
        Some(&cargo_home),
        &crates_index::HashKind::Stable,
    )?;
    let cache_path = index_path.join(".cache/re/le/release-test");
    fs::create_dir_all(cache_path.parent().unwrap())?;
    let entry = r#"{"name":"release-test","vers":"1.2.3","deps":[],"cksum":"0000000000000000000000000000000000000000000000000000000000000000","features":{},"yanked":false}"#;
    fs::write(
        cache_path,
        [
            b"\x03\x02\0\0\0etag: \"test\"\0".as_slice(),
            b"1.2.3\0",
            entry.as_bytes(),
            b"\0",
        ]
        .concat(),
    )?;

    let output = release(root, "keep", &[])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains("'release-test' 1.2.3 -> 2.0.0 (publish): manifest version 2.0.0 already differs from latest published version 1.2.3"),
        "{stderr}"
    );
    assert!(git(root, "diff HEAD --")?.is_empty());
    Ok(())
}

#[test]
fn detached_head_supports_dry_runs_but_cannot_push_a_release() -> gix_testtools::Result {
    let dir = fixture()?;
    let root = dir.path();
    git(root, "tag release-test-v0.8.0")?;
    write(root, "release/src/lib.rs", "pub fn fixed() {}\n")?;
    git(root, "commit -am 'fix: repair the library'")?;
    let head = git(root, "rev-parse HEAD")?;
    write(root, "release/src/lib.rs", "pub fn breaking() {}\n")?;
    git(root, "commit -am 'feat!: break the library'")?;
    git(root, "checkout --detach HEAD^")?;
    let original_status = git(root, "status --porcelain")?;

    let run = |args: &[&str]| {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_cargo-smart-release"));
        gix_testtools::configure_git_environment(&mut cmd, root)
            .current_dir(root.join("release"))
            .env("CARGO_HOME", root.join("cargo-home"))
            .env("CARGO_NET_OFFLINE", "true")
            .env("RUST_LOG", "info,cargo_smart_release=trace")
            .args(args)
            .output()
    };
    let release_args = [
        "smart-release",
        "release-test",
        "--no-publish",
        "--no-changelog-github-release",
        "--no-changelog-preview",
        "--no-bump-on-demand",
        "--verbose",
    ];
    let output = run(&release_args)?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("'release-test' from 0.8.0 to 0.8.1"), "{stderr}");
    assert!(
        stderr.contains("detached HEAD"),
        "the push preview explains its limitation: {stderr}"
    );

    let output = run(&["changelog", "release-test", "--no-preview"])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("WOULD write 3 sections"), "{stderr}");

    let output = run(&[release_args.as_slice(), &["--execute"]].concat())?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "{stderr}");
    assert!(stderr.contains("detached HEAD"), "{stderr}");
    assert_eq!(git(root, "rev-parse HEAD")?, head);
    assert_eq!(git(root, "status --porcelain")?, original_status);
    assert!(!root.join("release/CHANGELOG.md").exists());
    assert!(!root.join("release/CHANGELOG.md.lock").exists());
    assert!(!root.join("release/Cargo.toml.lock").exists());
    assert!(gix::open(root)?.head()?.is_detached());

    let output = run(&[release_args.as_slice(), &["--execute", "--no-tag", "--allow-dirty"]].concat())?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(fs::read_to_string(root.join("release/Cargo.toml"))?.contains("version = \"0.8.1\""));
    Ok(())
}

#[test]
fn updates_tracked_dependents_outside_the_release_workspace() -> gix_testtools::Result {
    let dir = fixture()?;
    let root = dir.path();
    let original_package = fs::read_to_string(root.join("release/Cargo.toml"))?;
    let original_head = git(root, "rev-parse HEAD")?;

    let output = release(root, "minor", &[])?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(fs::read_to_string(root.join("tools/fuzz/Cargo.toml"))?, FUZZ_MANIFEST);
    assert_eq!(fs::read_to_string(root.join("tools/Cargo.toml"))?, TOOLS_MANIFEST);
    assert_eq!(fs::read_to_string(root.join("release/Cargo.toml"))?, original_package);
    assert_eq!(git(root, "rev-parse HEAD")?, original_head, "dry runs do not commit");

    let output = release(root, "minor", &["--execute"])?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(
        git(root, "show -s --format='%an <%ae> %aI%n%cn <%ce> %cI' HEAD")?,
        git(root, "show -s --format='%an <%ae> %aI%n%cn <%ce> %cI' HEAD^")?,
        "release commits use the same deterministic signatures as fixture commits"
    );
    assert_eq!(
        fs::read_to_string(root.join("tools/fuzz/Cargo.toml"))?,
        FUZZ_MANIFEST.replace("\"0.8.0\"", "\"^0.9.0\""),
        "the standalone fuzz workspace gets dependency edits while preserving comments and its own version"
    );
    assert_eq!(
        fs::read_to_string(root.join("tools/Cargo.toml"))?,
        TOOLS_MANIFEST.replace("\"0.8.0\"", "\"^0.9.0\"")
    );
    assert_eq!(
        fs::read_to_string(root.join("unrelated/Cargo.toml"))?,
        UNRELATED_MANIFEST
    );
    assert_eq!(
        fs::read_to_string(root.join("unrelated-crlf/Cargo.toml"))?,
        UNRELATED_MANIFEST.replace('\n', "\r\n"),
        "unrelated manifests retain their line endings"
    );
    #[cfg(unix)]
    assert_eq!(
        fs::read_to_string(root.join("link-target.toml"))?,
        TOOLS_MANIFEST.replace("../release", "release"),
        "tracked manifest symlinks are not followed"
    );
    for path in ["untracked/Cargo.toml", "ignored/Cargo.toml"] {
        assert_eq!(fs::read_to_string(root.join(path))?, TOOLS_MANIFEST, "{path}");
    }
    assert_eq!(
        fs::read_to_string(root.join("tools/fuzz/Cargo.lock"))?,
        "leave this separate lockfile alone\n"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .replace('\\', "/")
            .contains("broken/Cargo.toml"),
        "malformed extra manifests produce a warning"
    );
    assert_eq!(
        git(root, "diff-tree --no-commit-id --name-only -r HEAD")?.replace('\r', ""),
        "release/Cargo.toml\ntools/Cargo.toml\ntools/fuzz/Cargo.toml\n",
        "all manifest edits belong to the release commit"
    );
    assert!(git(root, "diff HEAD --")?.is_empty());
    Ok(())
}

#[test]
fn invalid_dependency_requirements_prevent_partial_releases() -> gix_testtools::Result {
    for (requirement, message) in [
        ("\"=0.8.0\"", "comparator"),
        ("\"invalid\"", "unexpected character"),
        ("3", "must be a string"),
    ] {
        let dir = fixture()?;
        let root = dir.path();
        write(
            root,
            "tools/fuzz/Cargo.toml",
            &FUZZ_MANIFEST.replace("version = \"0.8.0\"", &format!("version = {requirement}")),
        )?;
        git(root, "commit -am 'set dependency requirement'")?;
        let original_head = git(root, "rev-parse HEAD")?;
        let output = release(root, "minor", &["--execute"])?;
        let stderr = String::from_utf8_lossy(&output.stderr).replace('\\', "/");
        assert!(!output.status.success(), "{requirement}: {stderr}");
        assert!(stderr.contains(message), "{requirement}: {stderr}");
        assert!(stderr.contains("tools/fuzz/Cargo.toml"), "{stderr}");
        assert_eq!(git(root, "rev-parse HEAD")?, original_head);
        assert!(git(root, "diff HEAD --")?.is_empty(), "no partial release edits");
        for path in [
            "release/Cargo.toml.lock",
            "tools/Cargo.toml.lock",
            "tools/fuzz/Cargo.toml.lock",
        ] {
            assert!(!root.join(path).exists(), "release locks are cleaned up: {path}");
        }
    }
    Ok(())
}

#[test]
fn discovered_dependents_respect_conservative_version_handling() -> gix_testtools::Result {
    for conservative in [true, false] {
        let dir = fixture()?;
        let root = dir.path();
        let args = if conservative {
            vec!["--execute"]
        } else {
            vec!["--execute", "--no-conservative-pre-release-version-handling"]
        };
        let output = release(root, "patch", &args)?;
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        for (path, original) in [
            ("tools/fuzz/Cargo.toml", FUZZ_MANIFEST),
            ("tools/Cargo.toml", TOOLS_MANIFEST),
        ] {
            let expected = if conservative {
                original.replace("\"0.8.0\"", "\"^0.8.1\"")
            } else {
                original.to_owned()
            };
            assert_eq!(fs::read_to_string(root.join(path))?, expected, "{path}");
        }
    }
    Ok(())
}

#[cfg(unix)]
#[test]
fn workspace_manifest_symlinks_keep_their_dependency_base_directory() -> gix_testtools::Result {
    let dir = fixture()?;
    let root = dir.path();
    let workspace = fs::read_to_string(root.join("release/Cargo.toml"))?;
    write(
        root,
        "release/Cargo.toml",
        &workspace.replace("[workspace]", "[workspace]\nmembers = [\"dependent\"]"),
    )?;
    fs::rename(root.join("release/Cargo.toml"), root.join("release-manifest.toml"))?;
    std::os::unix::fs::symlink("../release-manifest.toml", root.join("release/Cargo.toml"))?;
    let dependent = r#"[package]
name = "workspace-dependent"
version = "0.0.0"
publish = false

[dependencies]
release-test = { path = "..", version = "0.8.0" }
"#;
    write(root, "dependent-manifest.toml", dependent)?;
    write(root, "release/dependent/src/lib.rs", "")?;
    std::os::unix::fs::symlink(
        "../../dependent-manifest.toml",
        root.join("release/dependent/Cargo.toml"),
    )?;
    git(root, "add release release-manifest.toml dependent-manifest.toml")?;
    git(root, "commit -m 'add a workspace member with a symlinked manifest'")?;

    let output = release(root, "minor", &["--execute"])?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    for path in ["release/Cargo.toml", "release/dependent/Cargo.toml"] {
        assert!(
            root.join(path).is_symlink(),
            "workspace manifest symlinks survive the release: {path}"
        );
    }
    assert_eq!(
        fs::read_to_string(root.join("release/dependent/Cargo.toml"))?,
        dependent.replace("\"0.8.0\"", "\"^0.9.0\""),
        "Cargo resolves dependencies relative to the manifest location, not its symlink target"
    );
    assert_eq!(
        fs::read_to_string(root.join("dependent-manifest.toml"))?,
        dependent.replace("\"0.8.0\"", "\"^0.9.0\""),
        "the dependency edit is written to the symlink target"
    );
    assert!(git(root, "diff HEAD --")?.is_empty());
    Ok(())
}

fn write(root: &Path, path: &str, content: &str) -> std::io::Result<()> {
    let path = root.join(path);
    fs::create_dir_all(path.parent().expect("fixture files have a parent"))?;
    fs::write(path, content)
}

fn fixture() -> gix_testtools::Result<tempfile::TempDir> {
    let dir = tempfile::tempdir()?;
    let root = dir.path();
    // The release workspace is below the repository root; its dependents are siblings.
    write(
        root,
        "release/Cargo.toml",
        "[package]\nname = \"release-test\"\nversion = \"0.8.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )?;
    write(root, "release/src/lib.rs", "")?;
    write(root, "tools/fuzz/Cargo.toml", FUZZ_MANIFEST)?;
    write(root, "tools/Cargo.toml", TOOLS_MANIFEST)?;
    write(root, "unrelated/Cargo.toml", UNRELATED_MANIFEST)?;
    write(
        root,
        "unrelated-crlf/Cargo.toml",
        &UNRELATED_MANIFEST.replace('\n', "\r\n"),
    )?;
    write(
        root,
        "other/Cargo.toml",
        "[package]\nname = \"release-test\"\nversion = \"0.8.0\"\n",
    )?;
    write(root, "broken/Cargo.toml", "not valid TOML [")?;
    #[cfg(unix)]
    {
        write(
            root,
            "link-target.toml",
            &TOOLS_MANIFEST.replace("../release", "release"),
        )?;
        fs::create_dir_all(root.join("linked"))?;
        std::os::unix::fs::symlink("../link-target.toml", root.join("linked/Cargo.toml"))?;
    }
    write(root, ".gitignore", "/cargo-home/\n**/Cargo.lock\n/ignored/\n")?;
    git(root, "init")?;
    git(root, "add .")?;
    git(root, "commit -m initial")?;
    write(root, "untracked/Cargo.toml", TOOLS_MANIFEST)?;
    write(root, "ignored/Cargo.toml", TOOLS_MANIFEST)?;
    write(root, "tools/fuzz/Cargo.lock", "leave this separate lockfile alone\n")?;
    Ok(dir)
}

fn release(root: &Path, bump: &str, args: &[&str]) -> std::io::Result<std::process::Output> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_cargo-smart-release"));
    gix_testtools::configure_git_environment(&mut cmd, root)
        .current_dir(root.join("release"))
        .env("CARGO_HOME", root.join("cargo-home"))
        .env("CARGO_NET_OFFLINE", "true")
        .env("RUST_LOG", "info,cargo_smart_release=trace")
        .args([
            "smart-release",
            "release-test",
            "--no-publish",
            "--no-push",
            "--no-tag",
            "--no-changelog-github-release",
            "--no-bump-on-demand",
            "--bump",
            bump,
            "--bump-dependencies",
            "keep",
        ])
        .args(args)
        .output()
}
