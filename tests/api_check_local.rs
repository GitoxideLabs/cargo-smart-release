use std::{
    fs,
    path::Path,
    process::{Command, Output},
};

use cargo_smart_release_core::api_check as core;
use gix_testtools::{git, tempfile, Result};

fn api_check(root: &Path, args: &[&str]) -> std::io::Result<Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-api-check"));
    gix_testtools::configure_git_environment(&mut command, root);
    command
        .current_dir(root)
        .env("CARGO_TARGET_DIR", root.join("target"))
        .args(["api-check"])
        .args(args)
        .output()
}

fn assert_exit(output: &Output, expected: i32) {
    assert_eq!(
        output.status.code(),
        Some(expected),
        "stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn manifest(supported: bool) -> String {
    let mut manifest = r#"[package]
name = "local-api"
version = "1.0.0"
edition = "2021"
"#
    .to_owned();
    if supported {
        manifest.push_str(
            r#"
[package.metadata.api-check]
profiles = [{ default-features = false, features = [] }]
"#,
        );
    }
    manifest
}

#[test]
fn local_checks_require_a_workspace_crate_and_explicit_profiles() -> Result {
    let dir = tempfile::tempdir()?;
    fs::create_dir(dir.path().join("src"))?;
    fs::write(dir.path().join("src/lib.rs"), "pub fn value() {}\n")?;
    fs::write(dir.path().join("Cargo.toml"), manifest(false))?;
    for (args, message) in [
        (vec![], "required"),
        (vec!["unknown"], "Workspace member 'unknown'"),
        (vec!["local-api"], "package.metadata.api-check.profiles"),
        (vec!["local-api", "--bump", "major"], "--release-plan"),
        (
            vec!["local-api", "--baseline", "HEAD", "--release-plan"],
            "cannot be used",
        ),
    ] {
        let output = api_check(dir.path(), &args)?;
        assert_exit(&output, 2);
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(message),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

#[test]
fn a_first_supported_release_still_audits_the_boundary() -> Result {
    let dir = tempfile::tempdir()?;
    let root = dir.path();
    fs::create_dir(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub fn value() {}\n")?;
    fs::write(root.join("Cargo.toml"), manifest(false))?;
    git(root, "init -q")?;
    git(root, "add .")?;
    git(root, "commit -qm initial")?;
    git(root, "tag v1.0.0")?;
    fs::write(root.join("Cargo.toml"), manifest(true))?;
    let output = api_check(root, &["local-api"])?;
    assert_exit(&output, 0);
    let stderr = String::from_utf8(output.stderr)?;
    assert!(stderr.contains("No supported release baseline"), "{stderr}");
    assert!(stderr.contains("API checks passed"), "{stderr}");
    let mut quiet = Command::new(env!("CARGO_BIN_EXE_cargo-api-check"));
    gix_testtools::configure_git_environment(&mut quiet, root);
    let output = quiet
        .current_dir(root)
        .env("CARGO_TARGET_DIR", root.join("target"))
        .env("RUST_LOG", "off")
        .args(["api-check", "local-api"])
        .output()?;
    assert_exit(&output, 0);
    let stderr = String::from_utf8(output.stderr)?;
    assert!(!stderr.contains("No supported release baseline"), "{stderr}");
    assert!(!stderr.contains("API checks passed"), "{stderr}");
    assert!(core::check("local-api", Some(&root.join("Cargo.toml")), None)?.is_none());
    fs::write(
        root.join("src/lib.rs"),
        r#"
#[doc = "**Unstable.**"] pub struct First;
#[doc = "**Unstable.**"] pub struct Second;
pub fn value() -> (First, Second) { (First, Second) }
"#,
    )?;
    let incompatible = core::check("local-api", Some(&root.join("Cargo.toml")), None)?.expect("boundary violations");
    assert_eq!(incompatible.package, "local-api");
    assert!(incompatible.baseline.is_none());
    assert_eq!(
        incompatible.issues.len(),
        2,
        "every exposed experimental type is retained"
    );
    for issue in &incompatible.issues {
        assert_eq!(
            issue.profile,
            core::FeatureProfile {
                default_features: false,
                features: Vec::new()
            }
        );
    }
    let paths: std::collections::BTreeSet<_> = incompatible
        .issues
        .iter()
        .map(|issue| match &issue.reason {
            core::Reason::ExperimentalType { path } => path.rsplit("::").next().unwrap(),
            reason => panic!("unexpected boundary violation: {reason:?}"),
        })
        .collect();
    assert_eq!(paths, std::collections::BTreeSet::from(["First", "Second"]));
    fs::write(
        root.join("Cargo.toml"),
        manifest(true).replace("features = []", "features = [\"missing\"]"),
    )?;
    let output = api_check(root, &["local-api"])?;
    assert_exit(&output, 2);
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("missing"),
        "invalid profiles cannot pass without a baseline"
    );
    Ok(())
}

#[test]
fn a_moved_manifest_cannot_silently_discard_a_release_baseline() -> Result {
    let dir = tempfile::tempdir()?;
    let root = dir.path();
    fs::create_dir(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "pub fn value() {}\n")?;
    fs::write(root.join("Cargo.toml"), manifest(true))?;
    git(root, "init -q")?;
    git(root, "add .")?;
    git(root, "commit -qm initial")?;
    git(root, "tag local-api-v1.0.0")?;
    fs::create_dir(root.join("member"))?;
    fs::rename(root.join("Cargo.toml"), root.join("member/Cargo.toml"))?;
    fs::rename(root.join("src"), root.join("member/src"))?;
    fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"member\"]\nresolver = \"2\"\n",
    )?;
    let output = api_check(root, &["local-api"])?;
    assert_exit(&output, 2);
    let stderr = String::from_utf8(output.stderr)?;
    assert!(
        stderr.contains("Could not read member/Cargo.toml in release tag"),
        "{stderr}"
    );
    Ok(())
}

#[test]
#[ignore = "requires Rust 1.99 and cargo-semver-checks 0.51+"]
fn local_baselines_support_standalone_workspace_and_nested_workspace_crates() -> Result {
    for layout in ["standalone", "workspace", "nested"] {
        let dir = tempfile::tempdir()?;
        let root = dir.path();
        let workspace = if layout == "nested" {
            root.join("nested")
        } else {
            root.to_owned()
        };
        let package = if layout == "standalone" {
            workspace.clone()
        } else {
            workspace.join("member")
        };
        fs::create_dir_all(package.join("src"))?;
        if layout != "standalone" {
            fs::write(
                workspace.join("Cargo.toml"),
                "[workspace]\nmembers = [\"member\"]\nresolver = \"2\"\n",
            )?;
        }
        fs::write(root.join(".gitignore"), "/target/\nCargo.lock\n")?;
        fs::write(package.join("Cargo.toml"), manifest(false))?;
        fs::write(package.join("src/lib.rs"), "pub fn value() {}\n")?;
        git(root, "init -q")?;
        git(root, "add .")?;
        git(root, "commit -qm ungated")?;
        let prefix = if layout == "standalone" { "" } else { "local-api-" };
        git(root, &format!("tag {prefix}v0.1.0"))?;
        fs::write(package.join("Cargo.toml"), manifest(true))?;
        git(root, "commit -qam supported")?;
        git(root, &format!("tag {prefix}v1.0.0"))?;
        let supported = git(root, "rev-parse HEAD")?;
        fs::write(package.join("Cargo.toml"), manifest(false))?;
        git(root, "commit -qam newer-ungated")?;
        git(root, &format!("tag {prefix}v1.1.0"))?;
        fs::write(package.join("Cargo.toml"), manifest(true))?;
        let manifest = workspace.join("Cargo.toml");
        let args = [
            "local-api",
            "--manifest-path",
            manifest.to_str().expect("temporary manifest is UTF-8"),
        ];
        let before = git(root, "diff HEAD --")?;
        let output = api_check(root, &args)?;
        assert_exit(&output, 0);
        let stderr = String::from_utf8(output.stderr)?;
        assert!(
            stderr.contains(&format!("Using supported release tag {prefix}v1.0.0")),
            "{layout}: {stderr}"
        );
        assert!(
            stderr.contains(supported.trim()),
            "the reported baseline is immutable: {stderr}"
        );
        assert_eq!(
            git(root, "diff HEAD --")?,
            before,
            "checks preserve tracked working-tree changes"
        );
        if layout == "standalone" {
            assert!(core::check("local-api", Some(&manifest), None)?.is_none());
        }
        fs::write(package.join("src/lib.rs"), "pub fn replacement() {}\n")?;
        let output = api_check(root, &args)?;
        assert_exit(&output, 10);
        if layout == "standalone" {
            let incompatible = core::check("local-api", Some(&manifest), None)?.expect("removed API");
            assert!(
                matches!(incompatible.baseline, Some(core::Baseline::Revision(ref revision)) if revision == supported.trim())
            );
            assert!(incompatible.issues.iter().any(|issue| matches!(&issue.reason,
                core::Reason::SignatureChanged { path, before, after: None } if path.ends_with("::value") && before.is_object())));
            let output = incompatible
                .issues
                .iter()
                .find_map(|issue| match &issue.reason {
                    core::Reason::SemverCheck { crate_name, output } if crate_name == "local_api" => Some(output),
                    _ => None,
                })
                .expect("full semver-checks diagnostic output");
            assert_eq!(output.status.code(), Some(100));
            assert!(!output.stdout.is_empty() || !output.stderr.is_empty());
        }
        let mut explicit = args.to_vec();
        explicit.extend(["--baseline", "HEAD"]);
        assert_exit(&api_check(root, &explicit)?, 10);
        *explicit.last_mut().expect("baseline argument") = "does-not-exist";
        assert_exit(&api_check(root, &explicit)?, 2);
    }
    Ok(())
}
