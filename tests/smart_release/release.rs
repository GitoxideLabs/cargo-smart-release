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
fn stages_only_the_new_changelogs_written_during_release() -> gix_testtools::Result {
    for (options, expected_message, stages_both) in [
        (
            ["--no-publish"].as_slice(),
            Some("Please consider creating changelog entries for crate: stats-only"),
            false,
        ),
        (
            &["--registry", "unused", "--allow-fully-generated-changelogs"],
            Some("Write changelog entries for crate(s) stats-only and try again"),
            false,
        ),
        (&["--no-publish", "--allow-empty-release-message"], None, true),
    ] {
        check_new_changelog_staging(options, expected_message, stages_both)?;
    }
    Ok(())
}

fn check_new_changelog_staging(
    options: &[&str],
    expected_message: Option<&str>,
    stages_both: bool,
) -> gix_testtools::Result {
    let dir = tempfile::tempdir()?;
    let root = dir.path();
    write(
        root,
        "Cargo.toml",
        "[workspace]\nresolver = \"2\"\nmembers = [\"stats-only\", \"with-notes\"]\n",
    )?;
    write(root, ".gitignore", "/cargo-home/\n/Cargo.lock\n")?;
    for name in ["stats-only", "with-notes"] {
        write(
            root,
            &format!("{name}/Cargo.toml"),
            &format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"),
        )?;
        write(root, &format!("{name}/src/lib.rs"), "")?;
    }
    git(root, "init")?;
    git(root, "add .")?;
    git(root, "commit -m 'add workspace crates'")?;
    write(root, "with-notes/src/lib.rs", "pub fn new_feature() {}\n")?;
    git(root, "commit -am 'feat: introduce an API'")?;

    let run = |extra: &[&str]| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-smart-release"));
        gix_testtools::configure_git_environment(&mut command, root)
            .current_dir(root)
            .env("CARGO_HOME", root.join("cargo-home"))
            .env("CARGO_NET_OFFLINE", "true")
            .env("RUST_LOG", "info,cargo_smart_release=trace")
            .args([
                "smart-release",
                "stats-only",
                "with-notes",
                "--verbose",
                "--no-push",
                "--no-changelog-github-release",
                "--no-changelog-preview",
                "--no-bump-on-demand",
                "--bump",
                "keep",
            ])
            .args(options)
            .args(extra)
            .output()
    };
    let skip_publish = options.contains(&"--no-publish");
    let output = run(if skip_publish {
        &[]
    } else {
        &["--dry-run-cargo-publish"]
    })?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    if !skip_publish {
        assert!(stderr.contains("WOULD stop release after commit"), "{stderr}");
        assert!(!stderr.contains("\"publish\""), "{stderr}");
        assert!(!stderr.contains("WOULD create tag"), "{stderr}");
    }
    let add_preview = stderr
        .lines()
        .find(|line| line.contains("\"add\" \"--\""))
        .expect("dry run previews git add");
    assert!(add_preview.contains("stats-only"), "{add_preview}");
    assert_eq!(add_preview.contains("with-notes"), stages_both, "{add_preview}");
    assert!(git(root, "status --porcelain")?.is_empty());

    let output = run(&["--execute"])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.success(), skip_publish, "{stderr}");
    if let Some(message) = expected_message {
        assert!(stderr.contains(message), "{stderr}");
    }
    assert!(!stderr.contains("\"publish\""), "{stderr}");
    let changelog = git(root, "show HEAD:stats-only/CHANGELOG.md")?;
    assert!(changelog.contains("### Commit Statistics"), "{changelog}");
    if stages_both {
        let changelog = git(root, "show HEAD:with-notes/CHANGELOG.md")?;
        assert!(changelog.contains("Introduce an API"), "{changelog}");
    } else {
        assert!(!root.join("with-notes/CHANGELOG.md").exists());
    }
    assert!(git(root, "status --porcelain")?.is_empty());
    Ok(())
}

#[test]
fn handwritten_unreleased_notes_only_replace_empty_release_placeholders() -> gix_testtools::Result {
    for (existing_notes, generated) in [("", true), ("", false), ("Existing release notes.", true)] {
        let dir = fixture()?;
        let root = dir.path();
        let changelog_path = root.join("release/CHANGELOG.md");
        let original = format!(
            "# Changelog\n\n## Unreleased\n\nadd `Error::source()` implementation.\n\n## v0.8.0 (2026-11-16)\n\n{existing_notes}\n\n## 0.7.0 (2026-11-04)\n\nPreviously released notes.\n",
        );
        fs::write(&changelog_path, &original)?;
        git(root, "add release/CHANGELOG.md")?;
        git(root, "commit -m 'prepare handwritten release notes'")?;
        let head = git(root, "rev-parse HEAD")?;
        let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-smart-release"));
        gix_testtools::configure_git_environment(&mut command, root)
            .current_dir(root.join("release"))
            .env("CARGO_HOME", root.join("cargo-home"))
            .env("CARGO_NET_OFFLINE", "true")
            .args([
                "smart-release",
                "release-test",
                "--execute",
                "--no-publish",
                "--no-push",
                "--no-changelog-github-release",
                "--no-changelog-preview",
                "--no-bump-on-demand",
                "--bump",
                "keep",
                "--bump-dependencies",
                "keep",
            ]);
        if !generated {
            for segment in ["clippy", "commit-statistics", "commit-details", "git-conventional"] {
                command.args(["--changelog-without", segment]);
            }
        }

        let output = command.output()?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        let changelog = fs::read_to_string(&changelog_path)?;
        if existing_notes.is_empty() {
            assert!(output.status.success(), "{stderr}");
            assert!(!changelog.contains("Unreleased"), "{changelog}");
            assert_eq!(changelog.matches("## v0.8.0").count(), 1, "{changelog}");
            let (released, previous) = changelog.split_once("## 0.7.0").expect("previous release remains");
            assert!(
                released.contains("add `Error::source()` implementation."),
                "{changelog}"
            );
            assert_eq!(released.contains("### Commit Statistics"), generated, "{changelog}");
            assert!(previous.contains("Previously released notes."), "{changelog}");
        } else {
            assert!(!output.status.success(), "{stderr}");
            assert!(
                stderr.contains("Refusing to merge user-authored changelog content into another release"),
                "{stderr}"
            );
            assert_eq!(changelog, original);
            assert_eq!(git(root, "rev-parse HEAD")?, head);
        }
        assert!(git(root, "diff HEAD --")?.is_empty());
    }
    Ok(())
}

#[test]
#[cfg(unix)]
fn major_zero_companions_publish_the_experimental_api_and_route_workspace_consumers() -> gix_testtools::Result {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir()?;
    let root = dir.path();
    write(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = [\"api\", \"client\", \"extra\", \"dev\"]\nresolver = \"2\"\n",
    )?;
    write(
        root,
        "api/Cargo.toml",
        r#"[package]
name = "csr-parallel-api"
version = "1.2.3"
edition = "2021"
[features]
default = ["supported", "unstable"]
supported = []
unstable = []
experimental = ["unstable"]
extra-experimental = []
[package.metadata.smart-release]
unstable-features = ["extra-experimental"]
unstable-stable-dependency = true
[lints.rust]
unexpected_cfgs = "deny"
[package.metadata.docs.rs]
features = ["supported", "experimental"]
"#,
    )?;
    write(
        root,
        "api/src/lib.rs",
        r#"#[cfg(not(companion))]
pub struct Shared;
#[cfg(companion)]
pub use smart_release_stable::Shared;
pub fn supported() {}
#[cfg(feature = "unstable")]
pub fn experimental() {}
"#,
    )?;
    write(
        root,
        "api/build.rs",
        r#"fn main() {
    println!("cargo:rustc-check-cfg=cfg(companion)");
    if env!("CARGO_PKG_VERSION_MAJOR") == "0" {
        println!("cargo:rustc-cfg=companion");
    }
}
"#,
    )?;
    write(
        root,
        "client/Cargo.toml",
        r#"[package]
name = "csr-parallel-client"
version = "0.4.0"
edition = "2021"
[features]
experiment = ["api/unstable", "api?/experimental"]
[target.'cfg(any(unix,windows))'.dependencies]
api = { package = "csr-parallel-api", path = "../api", version = "^1.2.3", optional = true, default-features = false, features = ["default"] }
"#,
    )?;
    write(root, "client/src/lib.rs", "")?;
    write(
        root,
        "dev/Cargo.toml",
        r#"[package]
name = "csr-parallel-dev"
version = "0.2.0"
edition = "2021"
[dev-dependencies]
csr-parallel-api = { path = "../api", version = "*", features = ["unstable"] }
"#,
    )?;
    write(root, "dev/src/lib.rs", "")?;
    write(
        root,
        "extra/Cargo.toml",
        r#"[package]
name = "csr-parallel-extra"
version = "2.0.0"
edition = "2021"
[features]
unstable = []
"#,
    )?;
    write(root, "extra/src/lib.rs", "")?;
    write(root, ".gitignore", "/Cargo.lock\n/cargo-home/\n/bin/\n/staged/\n")?;
    git(root, "init")?;
    git(root, "add .")?;
    git(root, "commit -m 'define the parallel release workspace'")?;
    let real_cargo = std::env::var_os("CARGO").expect("Cargo supplies its executable to tests");
    write(
        root,
        "bin/cargo",
        r#"#!/bin/sh
set -eu
if [ "$1" != publish ]; then exec "$REAL_CARGO" "$@"; fi
dry_run=no
for arg do
    if [ "$arg" = --dry-run ]; then dry_run=yes; fi
done
test "$dry_run" = yes
while [ "$1" != --manifest-path ]; do
    if [ "$1" = --config ]; then
        cp "$2" "$CAPTURE/patches.toml"
        shift
    fi
    shift
done
root=${2%/*}
version_dir=${root%/*}
destination="$CAPTURE/${version_dir##*/}"
mkdir "$destination"
cp -R "$root/." "$destination"
printf '%s\n' "${version_dir##*/}" >> "$CAPTURE/order"
"#,
    )?;
    fs::set_permissions(root.join("bin/cargo"), fs::Permissions::from_mode(0o755))?;
    fs::create_dir(root.join("staged"))?;
    let path = std::env::var_os("PATH").expect("the test environment has a PATH");
    let path = std::env::join_paths(std::iter::once(root.join("bin")).chain(std::env::split_paths(&path)))?;
    let run = |capture: &Path, packages: &[&str], bump: &str| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-smart-release"));
        gix_testtools::configure_git_environment(&mut command, root)
            .current_dir(root)
            .env("PATH", &path)
            .env("REAL_CARGO", &real_cargo)
            .env("CAPTURE", capture)
            .env("RUST_LOG", "info,cargo_smart_release=trace")
            .env("CARGO_HOME", root.join("cargo-home"))
            .env_remove("CARGO_REGISTRY_DEFAULT")
            .env("CARGO_NET_OFFLINE", "true")
            .args([
                "smart-release",
                "--dry-run-cargo-publish",
                "--no-push",
                "--no-changelog",
                "--no-changelog-github-release",
                "--no-bump-on-demand",
                "--bump",
                bump,
            ])
            .args(packages)
            .output()
    };
    let staged = root.join("staged");
    write(root, ".cargo/config.toml", "[registry]\ndefault = \"private\"\n")?;
    let output = run(&staged, &["csr-parallel-api"], "patch")?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success() && stderr.contains("require the crates.io index"),
        "{stderr}"
    );
    assert!(!staged.join("order").exists());
    fs::remove_file(root.join(".cargo/config.toml"))?;

    let output = run(&staged, &["csr-parallel-dev"], "patch")?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success() && stderr.contains("include 'csr-parallel-api' in the release"),
        "{stderr}"
    );
    assert!(!staged.join("order").exists());

    let output = run(
        &staged,
        &[
            "csr-parallel-dev",
            "csr-parallel-client",
            "csr-parallel-api",
            "csr-parallel-extra",
        ],
        "patch",
    )?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("WOULD create tag csr-parallel-api/v1.2.4"), "{stderr}");
    assert!(
        stderr.contains("WOULD create one companion source commit for csr-parallel-api-0.1.0, csr-parallel-extra-0.1.0 with the stable release as its parent"),
        "{stderr}"
    );
    assert!(
        stderr.contains("WOULD create tag object csr-parallel-api/v0.1.0 with changelog message, first line is: 'Release csr-parallel-api v0.1.0 with experimental APIs'"),
        "{stderr}"
    );
    assert!(
        stderr.contains("WOULD create tag object csr-parallel-extra/v0.1.0"),
        "{stderr}"
    );
    assert_eq!(
        stderr.matches("WOULD create one companion source commit").count(),
        1,
        "{stderr}"
    );

    assert_eq!(
        fs::read_to_string(staged.join("order"))
            .as_deref()
            .unwrap_or("<missing>"),
        "csr-parallel-api-1.2.4\ncsr-parallel-api-0.1.0\ncsr-parallel-dev-0.2.1\ncsr-parallel-client-0.4.1\ncsr-parallel-extra-2.0.1\ncsr-parallel-extra-0.1.0\n",
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stable = staged.join("csr-parallel-api-1.2.4");
    let client = fs::read_to_string(staged.join("csr-parallel-client-0.4.1/Cargo.toml"))?;
    assert!(client.contains("version = \"^0.1.0\""), "{client}");
    assert!(
        client.contains("api?/experimental") && client.contains("api/unstable"),
        "{client}"
    );
    assert_eq!(fs::read(stable.join("Cargo.lock"))?, fs::read(root.join("Cargo.lock"))?);
    let patches = fs::read_to_string(staged.join("patches.toml"))?;
    assert!(
        patches.contains("csr-parallel-api-0.1.0") && patches.contains("csr-parallel-api-1.2.4"),
        "{patches}"
    );
    let stable_manifest = fs::read_to_string(stable.join("Cargo.toml"))?;
    let stable_manifest = stable_manifest.parse::<toml_edit::DocumentMut>()?;
    assert!(stable_manifest["features"].get("unstable").is_none());
    assert!(stable_manifest["features"].get("experimental").is_none());
    assert!(stable_manifest["features"].get("extra-experimental").is_none());
    assert_eq!(
        stable_manifest["package"]["metadata"]["docs"]["rs"]["features"]
            .as_array()
            .unwrap()
            .len(),
        1
    );

    assert!(
        git(root, "diff HEAD --")?.is_empty(),
        "staging kept source manifests intact"
    );

    let manifest = root.join("api/Cargo.toml");
    fs::write(&manifest, fs::read_to_string(&manifest)?.replace("1.2.3", "1.2.5"))?;
    git(root, "commit -am 'release the stable package'")?;
    git(root, "tag csr-parallel-api/v1.2.5")?;
    let (cache, _) = crates_index::local_path_and_canonical_url_with_hash_kind(
        crates_index::sparse::URL,
        Some(&root.join("cargo-home")),
        &crates_index::HashKind::Stable,
    )?;
    let cache = cache.join(".cache/cs/r-/csr-parallel-api");
    fs::create_dir_all(cache.parent().expect("cache has a parent"))?;
    let mut entries = b"\x03\x02\0\0\0etag: \"test\"\0".to_vec();
    for (version, yanked) in [("1.2.5", false), ("0.6.0", true)] {
        let entry = serde_json::json!({
            "name": "csr-parallel-api", "vers": version, "deps": [], "cksum": "0".repeat(64),
            "features": {}, "yanked": yanked
        });
        entries.extend_from_slice(format!("{version}\0{entry}\0").as_bytes());
    }
    fs::write(&cache, entries)?;
    let retry = staged.join("retry");
    fs::create_dir(&retry)?;
    let output = run(&retry, &["csr-parallel-api"], "keep")?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains("WOULD create tag object csr-parallel-api/v0.7.0"),
        "{stderr}"
    );
    assert_eq!(
        fs::read_to_string(retry.join("order"))?,
        "csr-parallel-api-0.7.0\n",
        "retry publishes only a companion and never reuses a yanked version"
    );

    let stable_branch = git(root, "symbolic-ref --short HEAD")?;
    let stable_commit = git(root, "rev-parse HEAD")?;
    git(root, "switch -c prepared-companion-fixture")?;
    let mut prepared_manifest =
        fs::read_to_string(retry.join("csr-parallel-api-0.7.0/Cargo.toml"))?.parse::<toml_edit::DocumentMut>()?;
    prepared_manifest["package"]["description"] = toml_edit::value("Prepared companion source");
    fs::write(&manifest, prepared_manifest.to_string())?;
    git(root, "commit -am 'prepare the companion source'")?;
    let source_commit = git(root, "rev-parse HEAD")?;
    git(
        root,
        &format!(
            "update-ref refs/cargo-smart-release/companions/{} {}",
            stable_commit.trim(),
            source_commit.trim(),
        ),
    )?;
    git(root, &format!("switch {}", stable_branch.trim()))?;
    let resumed = staged.join("resumed");
    fs::create_dir(&resumed)?;
    let output = run(&resumed, &["csr-parallel-api"], "keep")?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert_eq!(fs::read_to_string(resumed.join("order"))?, "companion-source\n");
    assert_eq!(
        fs::read_to_string(resumed.join("companion-source/Cargo.toml"))?,
        prepared_manifest.to_string(),
        "a retry uses the original prepared manifest"
    );

    let stable_later = staged.join("csr-parallel-api-1.2.5");
    let mut manifest = stable_manifest.clone();
    manifest["package"]["version"] = toml_edit::value("1.2.5");
    write(&stable_later, "Cargo.toml", &manifest.to_string())?;
    write(
        &stable_later,
        "src/lib.rs",
        &fs::read_to_string(stable.join("src/lib.rs"))?,
    )?;
    fs::copy(stable.join("build.rs"), stable_later.join("build.rs"))?;

    write(
        &staged,
        "check/Cargo.toml",
        r#"[package]
name = "check-parallel-releases"
version = "0.1.0"
edition = "2021"
[workspace]
[dependencies]
stable = { package = "csr-parallel-api", version = "1.2", features = ["supported"] }
early = { package = "csr-parallel-api", version = "0.1" }
late = { package = "csr-parallel-api", version = "0.7" }

[patch.crates-io]
stable = { package = "csr-parallel-api", path = "../csr-parallel-api-1.2.4" }
stable-later = { package = "csr-parallel-api", path = "../csr-parallel-api-1.2.5" }
early = { package = "csr-parallel-api", path = "../csr-parallel-api-0.1.0" }
late = { package = "csr-parallel-api", path = "../retry/csr-parallel-api-0.7.0" }
"#,
    )?;
    let consumer = staged.join("check");
    write(
        &consumer,
        "src/lib.rs",
        "pub fn check(a: early::Shared, b: late::Shared) -> (stable::Shared, early::Shared) {
    stable::supported();
    early::experimental();
    late::experimental();
    (a, b)
}",
    )?;
    let check = || {
        Command::new(&real_cargo)
            .current_dir(&consumer)
            .args(["check", "--offline"])
            .output()
    };
    let output = check()?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    write(&consumer, "src/lib.rs", "pub fn check() { stable::experimental(); }")?;
    let output = check()?;
    assert!(!output.status.success(), "the 1.x crate must hide its experimental API");
    assert!(String::from_utf8_lossy(&output.stderr).contains("cannot find function `experimental`"));
    Ok(())
}

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
fn manifest_only_changes_in_a_workspace_root_are_included_in_changelogs() -> gix_testtools::Result {
    let dir = tempfile::tempdir()?;
    let root = dir.path();
    write(
        root,
        "Cargo.toml",
        "[package]\nname = \"release-test\"\nversion = \"0.8.0\"\n\n\
         [workspace]\nmembers = [\"leaf\"]\n\n\
         [dependencies]\nleaf = { path = \"leaf\", version = \"0.1.0\" }\n",
    )?;
    write(root, "src/lib.rs", "pub fn initial() {}\n")?;
    write(
        root,
        "leaf/Cargo.toml",
        "[package]\nname = \"leaf\"\nversion = \"0.1.0\"\n",
    )?;
    write(root, "leaf/src/lib.rs", "pub fn initial() {}\n")?;
    write(root, ".gitignore", "/cargo-home/\n/Cargo.lock\n")?;
    git(root, "init")?;
    git(root, "add .")?;
    git(root, "commit -m 'feat: initial workspace'")?;
    git(root, "tag v0.8.0")?;
    git(root, "tag leaf/v0.1.0")?;
    let manifest_path = root.join("Cargo.toml");
    fs::write(
        &manifest_path,
        fs::read_to_string(&manifest_path)? + "\n[features]\nplatform-backend = []\n",
    )?;
    git(root, "commit -am 'fix: configure the platform backend'")?;
    let recorded = "# Changelog\n\n## 0.8.0 (2000-01-02)\n\nPreviously recorded release notes.\n";
    write(root, "CHANGELOG.md", recorded)?;
    git(root, "add CHANGELOG.md")?;
    git(root, "commit -m 'record the previous release'")?;
    let manifest = fs::read(&manifest_path)?;
    let head = git(root, "rev-parse HEAD")?;
    let mut preview = Command::new(env!("CARGO_BIN_EXE_cargo-changelog"));
    let preview = gix_testtools::configure_git_environment(&mut preview, root)
        .current_dir(root)
        .env("CARGO_HOME", root.join("cargo-home"))
        .env("CARGO_NET_OFFLINE", "true")
        .args(["changelog", "release-test", "--only", "--no-preview"])
        .output()?;
    assert!(preview.status.success(), "{}", String::from_utf8_lossy(&preview.stderr));
    assert_eq!(
        fs::read_to_string(root.join("CHANGELOG.md"))?,
        recorded,
        "preview does not write"
    );
    let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-changelog"));
    let output = gix_testtools::configure_git_environment(&mut command, root)
        .current_dir(root)
        .env("CARGO_HOME", root.join("cargo-home"))
        .env("CARGO_NET_OFFLINE", "true")
        .args(["changelog", "release-test", "--write", "--no-preview"])
        .output()?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let changelog = fs::read_to_string(root.join("CHANGELOG.md"))?;
    let (unreleased, released) = changelog.split_once("## 0.8.0").expect("release section exists");
    assert!(unreleased.contains("configure the platform backend"), "{changelog}");
    assert!(released.contains("Previously recorded release notes."), "{changelog}");
    assert!(
        released.contains("initial workspace"),
        "recorded releases gain generated history: {changelog}"
    );
    assert!(
        !root.join("leaf/CHANGELOG.md").exists(),
        "unchanged dependencies stay excluded"
    );
    assert_eq!(fs::read(&manifest_path)?, manifest);
    assert_eq!(git(root, "rev-parse HEAD")?, head);
    Ok(())
}

#[test]
fn changelog_prefers_namespaced_tags_for_duplicate_release_versions() -> gix_testtools::Result {
    for namespaced_first in [false, true] {
        let dir = fixture()?;
        let root = dir.path();
        let mut tags = ["release-test-v0.8.0", "release-test/v0.8.0"];
        if namespaced_first {
            tags.reverse();
        }
        git(root, &format!("tag {}", tags[0]))?;
        write(root, "release/src/lib.rs", "pub fn migrated() {}\n")?;
        git(root, "commit -am 'fix: between release aliases'")?;
        git(root, &format!("tag {}", tags[1]))?;
        write(root, "release/src/lib.rs", "pub fn migrated() {}\npub fn later() {}\n")?;
        git(root, "commit -am 'fix: after both release tags'")?;

        let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-smart-release"));
        let output = gix_testtools::configure_git_environment(&mut command, root)
            .current_dir(root.join("release"))
            .env("CARGO_HOME", root.join("cargo-home"))
            .env("CARGO_NET_OFFLINE", "true")
            .args(["changelog", "release-test", "--write", "--allow-dirty", "--no-preview"])
            .output()?;
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let changelog = fs::read_to_string(root.join("release/CHANGELOG.md"))?;
        assert_eq!(changelog.matches("## v0.8.0").count(), 1, "{changelog}");
        let (unreleased, released) = changelog.split_once("## v0.8.0").expect("release section exists");
        assert!(unreleased.contains("after both release tags"), "{changelog}");
        assert_eq!(
            unreleased.contains("between release aliases"),
            namespaced_first,
            "{changelog}"
        );
        assert_eq!(
            released.contains("between release aliases"),
            !namespaced_first,
            "{changelog}"
        );
    }
    Ok(())
}

#[cfg(unix)]
#[test]
fn formatter_changes_are_previewed_when_changelog_initialization_is_unchanged() -> gix_testtools::Result {
    let dir = fixture()?;
    let root = dir.path();
    let run = |args: &[&str]| -> std::io::Result<std::process::Output> {
        let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-smart-release"));
        gix_testtools::configure_git_environment(&mut command, root)
            .current_dir(root.join("release"))
            .env("CARGO_HOME", root.join("cargo-home"))
            .env("CARGO_NET_OFFLINE", "true")
            .env("BAT_PAGER", "cat")
            .args(args)
            .output()
    };
    let output = run(&["changelog", "release-test", "--write", "--allow-dirty", "--no-preview"])?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let changelog_path = root.join("release/CHANGELOG.md");
    let previous_content = fs::read_to_string(&changelog_path)?;
    assert!(previous_content.contains("Initial"), "{previous_content}");

    let output = run(&[
        "smart-release",
        "release-test",
        "--execute",
        "--allow-dirty",
        "--no-publish",
        "--no-push",
        "--no-changelog-github-release",
        "--no-bump-on-demand",
        "--bump",
        "keep",
        "--bump-dependencies",
        "keep",
        "--allow-fully-generated-changelogs",
        "--allow-empty-release-message",
        "--formatter",
        "sed 's/Initial/Formatted/g'",
    ])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains("Will leave alone the changelog for 'release-test'"),
        "{stderr}"
    );
    let formatted_content = fs::read_to_string(changelog_path)?;
    assert!(formatted_content.contains("Formatted"), "{formatted_content}");
    assert_ne!(formatted_content, previous_content);
    assert!(stderr.contains("About to preview 1 pending changelog(s)"), "{stderr}");
    Ok(())
}

#[test]
fn unreachable_tag_aliases_preserve_reachable_release_boundaries() -> gix_testtools::Result {
    for namespaced_reachable in [false, true] {
        let dir = fixture()?;
        let root = dir.path();
        let mut tags = ["release-test-v0.8.0", "release-test/v0.8.0"];
        if namespaced_reachable {
            tags.reverse();
        }
        write(root, "release/src/lib.rs", "pub fn before() {}\n")?;
        git(root, "commit -am 'feat!: previously released change'")?;
        git(root, &format!("tag {}", tags[0]))?;
        let off_branch = git(root, "commit-tree HEAD^{tree} -m 'off-branch release'")?;
        git(root, &format!("tag {} {}", tags[1], off_branch.trim()))?;
        write(root, "release/src/lib.rs", "pub fn after() {}\n")?;
        git(root, "commit -am 'fix: after reachable release'")?;

        let output = release(root, "auto", &[])?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stderr}");
        assert!(stderr.contains("'release-test' from 0.8.0 to 0.8.1"), "{stderr}");

        let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-smart-release"));
        let output = gix_testtools::configure_git_environment(&mut command, root)
            .current_dir(root.join("release"))
            .env("CARGO_HOME", root.join("cargo-home"))
            .env("CARGO_NET_OFFLINE", "true")
            .args(["changelog", "release-test", "--write", "--allow-dirty", "--no-preview"])
            .output()?;
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let changelog = fs::read_to_string(root.join("release/CHANGELOG.md"))?;
        assert_eq!(changelog.matches("## v0.8.0").count(), 1, "{changelog}");
        let (unreleased, released) = changelog.split_once("## v0.8.0").expect("release section exists");
        assert!(unreleased.contains("after reachable release"), "{changelog}");
        assert!(!unreleased.contains("previously released change"), "{changelog}");
        assert!(released.contains("previously released change"), "{changelog}");
    }
    Ok(())
}

#[test]
fn detached_head_supports_dry_runs_but_cannot_push_a_release() -> gix_testtools::Result {
    let dir = fixture()?;
    let root = dir.path();
    git(root, "tag release-test/v0.8.0")?;
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
