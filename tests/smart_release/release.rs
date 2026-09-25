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
    // Isolate subprocess Git operations without changing the test process's environment.
    for (key, _) in std::env::vars_os().filter(|(key, _)| key.to_string_lossy().starts_with("GIT_")) {
        cmd.env_remove(key);
    }
    gix_testtools::apply_git_config_by_environment(
        &mut cmd,
        &[
            ("user.name", "author"),
            ("user.email", "author@example.com"),
            ("commit.gpgsign", "false"),
            ("tag.gpgsign", "false"),
        ],
    );
    cmd.current_dir(root.join("release"))
        .env("GIT_CONFIG_GLOBAL", if cfg!(windows) { "nul" } else { "/dev/null" })
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("CARGO_HOME", root.join("cargo-home"))
        .env("CARGO_NET_OFFLINE", "true")
        .env("RUST_LOG", "info,cargo_smart_release=trace")
        .args([
            "smart-release",
            "release-test",
            "--no-publish",
            "--no-push",
            "--no-tag",
            "--no-changelog",
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