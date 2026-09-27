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
