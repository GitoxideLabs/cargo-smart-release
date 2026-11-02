use std::{fs, path::Path, process::Command};

use gix_testtools::{git, tempfile};

#[test]
fn api_checks_prefer_namespaced_release_tags() -> gix_testtools::Result {
    let dir = fixture(Some(0), "")?;
    git(dir.path(), "tag csr-api-facade/v1.2.3 csr-api-facade-v1.2.3")?;
    let output = release(dir.path(), &[])?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let request: serde_json::Value = serde_json::from_str(&fs::read_to_string(dir.path().join("request.json"))?)?;
    assert_eq!(request["baseline_revision"], "csr-api-facade/v1.2.3");
    Ok(())
}

#[test]
fn private_breaking_dependencies_stop_at_verified_boundaries() -> gix_testtools::Result {
    let dir = fixture(Some(0), "")?;
    let output = release(dir.path(), &[])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("'csr-api-facade' from 1.2.3 to 1.2.4"), "{stderr}");
    assert!(stderr.contains("'csr-api-client' from 1.2.3 to 1.2.4"), "{stderr}");
    assert!(!stderr.contains("Stable-crate breaking-change report:"), "{stderr}");
    assert!(
        !stderr.contains("to 2.0.0"),
        "a verified private dependency must not cause a major bump: {stderr}"
    );
    let request: serde_json::Value = serde_json::from_str(&fs::read_to_string(dir.path().join("request.json"))?)?;
    assert_eq!(request["unstable_features"], serde_json::json!(["unstable"]));
    assert_eq!(request["baseline_revision"], "csr-api-facade-v1.2.3");
    let leaf = request["dependencies"]
        .as_array()
        .expect("request dependencies")
        .iter()
        .find(|dep| dep["name"] == "csr-api-leaf")
        .expect("leaf candidate");
    assert_eq!(leaf["current_version"], "0.2.0");
    assert_eq!(leaf["next_version"], "0.3.0");
    assert_eq!(leaf["breaking"], true);
    assert!(
        git(dir.path(), "diff HEAD --")?.is_empty(),
        "dry run keeps tracked files intact"
    );
    Ok(())
}

#[test]
fn rejected_boundaries_and_unchecked_crates_keep_conservative_bumps() -> gix_testtools::Result {
    for result in [Some(10), None] {
        let dir = fixture(result, "")?;
        let output = release(dir.path(), &[])?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stderr}");
        assert!(stderr.contains("'csr-api-facade' from 1.2.3 to 2.0.0"), "{stderr}");
        assert!(stderr.contains("'csr-api-client' from 1.2.3 to 2.0.0"), "{stderr}");
        assert!(
            stderr.contains("Stable-crate breaking-change report: WOULD affect 2 stable crates"),
            "{stderr}"
        );
        let cause = if result.is_some() {
            "API compatibility check reported an incompatible stable API"
        } else {
            "conservative safety bump due to breaking dependency 'csr-api-leaf'"
        };
        assert!(
            stderr.contains(&format!("'csr-api-facade' 1.2.3 -> 2.0.0 (publish): {cause}")),
            "{stderr}"
        );
        let dependency = if result.is_some() {
            "csr-api-facade"
        } else {
            "csr-api-leaf"
        };
        assert!(
            stderr.contains(&format!("'csr-api-client' 1.2.3 -> 2.0.0 (publish): conservative safety bump due to breaking dependency '{dependency}'")),
            "{stderr}"
        );
        assert!(!stderr.contains("'csr-api-leaf' 0.2.0 ->"), "{stderr}");
    }
    Ok(())
}

#[test]
fn unstable_consumers_keep_conservative_bumps() -> gix_testtools::Result {
    for features in ["features = [\"unstable\"],", "features = [\"experimental\"],"] {
        let dir = fixture(Some(0), features)?;
        let output = release(dir.path(), &[])?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stderr}");
        assert!(stderr.contains("'csr-api-facade' from 1.2.3 to 1.2.4"), "{stderr}");
        assert!(stderr.contains("'csr-api-client' from 1.2.3 to 2.0.0"), "{stderr}");
    }
    Ok(())
}

#[test]
fn experimental_compatibility_controls_patch_and_minor_releases() -> gix_testtools::Result {
    for (experimental_result, expected) in [(0, "1.2.4"), (10, "1.3.0")] {
        let dir = fixture(Some(0), "")?;
        let client = dir.path().join("client/Cargo.toml");
        fs::write(&client, format!("{}\n[features]\nunstable = [\"csr-api-facade/unstable\"]\n[package.metadata.smart-release]\napi-check = [\"sh\", \"client-check.sh\"]\nunstable-features = [\"unstable\"]\n", fs::read_to_string(&client)?))?;
        fs::write(
            dir.path().join("client-check.sh"),
            r#"#!/bin/sh
request=$(cat)
case "$request" in
    *'"experimental":true'*'"experimental_breaking":true'*) exit 10 ;;
    *) exit 0 ;;
esac
"#,
        )?;
        fs::write(
            dir.path().join("check.sh"),
            format!(
                r#"#!/bin/sh
request=$(cat)
printf '%s\n' "$request" > request.json
case "$request" in
    *'"experimental":true'*) exit {experimental_result} ;;
    *) exit 0 ;;
esac
"#
            ),
        )?;
        git(dir.path(), "add client-check.sh")?;
        git(dir.path(), "commit -am 'configure experimental API check'")?;
        let output = release(dir.path(), &[])?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stderr}");
        assert!(
            stderr.contains(&format!("'csr-api-facade' from 1.2.3 to {expected}")),
            "experimental incompatibility requires a minor release, while compatibility permits patches: {stderr}"
        );
        assert!(
            stderr.contains(&format!("'csr-api-client' from 1.2.3 to {expected}")),
            "experimental breaks propagate to checked consumers without becoming stable major releases: {stderr}"
        );
        let output = preflight(dir.path(), "csr-api-client", "auto")?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(experimental_result), "{stderr}");
        assert!(
            stderr.contains(&format!("Release candidate csr-api-client: 1.2.3 -> {expected}")),
            "{stderr}"
        );
    }
    Ok(())
}

#[test]
fn experimental_breaks_outside_the_selected_graph_receive_manifest_bumps() -> gix_testtools::Result {
    let dir = fixture(Some(0), "")?;
    let root = dir.path();
    let manifest = root.join("client/Cargo.toml");
    fs::write(
        &manifest,
        format!(
            "{}\n[package.metadata.smart-release]\napi-check = [\"sh\", \"client-check.sh\"]\n",
            fs::read_to_string(&manifest)?
        ),
    )?;
    fs::write(
        root.join("client-check.sh"),
        r#"#!/bin/sh
request=$(cat)
case "$request" in
    *'"experimental":true'*) exit 10 ;;
    *) exit 0 ;;
esac
"#,
    )?;
    git(root, "add client-check.sh")?;
    git(
        root,
        "commit -am 'check an experimental API outside the selected graph'",
    )?;

    let output = release_package(root, "csr-api-facade", &["--execute", "--no-changelog"])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        fs::read_to_string(manifest)?.contains("version = \"1.3.0\""),
        "{stderr}"
    );
    assert!(
        !stderr.contains("'csr-api-client' from"),
        "the dependent outside the selected graph needs only a manifest bump: {stderr}"
    );
    Ok(())
}

#[test]
fn unstable_dependency_requirements_allow_only_patch_updates() -> gix_testtools::Result {
    for (features, requested, expected, changed_dependency) in [
        ("", "1.2.3", "1.2.3", true),
        ("features = [\"experimental\"],", "1.2.3", "~1.2.4", true),
        ("features = [\"unstable\"],", "~1.2.3", "~1.2.3", true),
        ("features = [\"unstable\"],", "1.2.3", "~1.2.3", false),
        ("features = [\"unstable\"],", "=1.2.3", "=1.2.3", false),
    ] {
        let dir = fixture(Some(0), features)?;
        if !changed_dependency {
            fs::write(dir.path().join("leaf/src/lib.rs"), "pub fn value() {}\n")?;
        }
        let manifest = dir.path().join("client/Cargo.toml");
        let original = fs::read_to_string(&manifest)?;
        fs::write(
            &manifest,
            original.replace("version = \"1.2.3\",", &format!("version = \"{requested}\",")),
        )?;
        git(
            dir.path(),
            "commit --allow-empty -am 'configure dependency requirement'",
        )?;
        let output = release(dir.path(), &["--execute", "--no-changelog"])?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success(),
            "release updates dependency requirements successfully: {stderr}"
        );
        assert!(
            fs::read_to_string(manifest)?.contains(&format!("version = \"{expected}\",")),
            "experimental dependency requirements constrain the minor series: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn inherited_experimental_requirements_are_constrained_in_package_workspaces() -> gix_testtools::Result {
    let dir = fixture(Some(0), "")?;
    let root = dir.path();
    let manifest = root.join("Cargo.toml");
    fs::create_dir(root.join("src"))?;
    fs::write(root.join("src/lib.rs"), "")?;
    fs::write(
        &manifest,
        format!(
            r#"{}
[package]
name = "csr-api-workspace"
version = "0.1.0"
edition = "2021"
[workspace.dependencies]
api = {{ package = "csr-api-facade", path = "facade", version = "1.2.3", }}
"#,
            fs::read_to_string(&manifest)?
        ),
    )?;
    let client = root.join("client/Cargo.toml");
    fs::write(
        &client,
        fs::read_to_string(&client)?.replace(
            "csr-api-facade = { path = \"../facade\", version = \"1.2.3\",  }",
            "api = { workspace = true, features = [\"experimental\"] }",
        ),
    )?;
    git(root, "add .")?;
    git(root, "commit -m 'inherit an aliased experimental dependency'")?;
    let output = release(root, &["--execute", "--no-changelog"])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(fs::read_to_string(manifest)?.contains("version = \"~1.2.4\""), "workspace requirements inherit the consumer's experimental opt-in even when the root is also a package: {stderr}");
    Ok(())
}

#[test]
fn checker_failure_aborts_before_release_mutations() -> gix_testtools::Result {
    let dir = fixture(Some(3), "")?;
    let head = git(dir.path(), "rev-parse HEAD")?;
    let output = release(dir.path(), &["--execute"])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "checker failure must reject the release: {stderr}"
    );
    assert!(stderr.contains("API checker for 'csr-api-facade' failed"), "{stderr}");
    assert!(stderr.contains("--no-api-check"), "{stderr}");
    assert!(stderr.contains("csr-api-facade-v1.2.3"), "{stderr}");
    assert!(stderr.contains("missing forwarded Cargo features"), "{stderr}");
    assert!(stderr.contains("only skips checks that cannot complete"), "{stderr}");
    assert_eq!(git(dir.path(), "rev-parse HEAD")?, head);
    assert!(git(dir.path(), "diff HEAD --")?.is_empty());
    Ok(())
}

#[test]
fn override_keeps_completed_verdicts_and_only_skips_failed_checks_for_one_invocation() -> gix_testtools::Result {
    for (result, expected) in [(0, "1.2.4"), (10, "2.0.0"), (3, "2.0.0")] {
        let dir = fixture(Some(result), "")?;
        let output = release(dir.path(), &["--no-api-check", "--execute", "--no-changelog"])?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stderr}");
        assert!(dir.path().join("request.json").exists(), "checker must run");
        for package in ["facade", "client"] {
            assert!(
                fs::read_to_string(dir.path().join(package).join("Cargo.toml"))?
                    .contains(&format!("version = \"{expected}\"")),
                "completed verdicts are retained; incomplete checks use conservative dependency bumps: {stderr}"
            );
        }
        assert_eq!(
            stderr.contains("using conservative version and dependency rules"),
            result == 3,
            "{stderr}"
        );
        assert!(git(dir.path(), "diff HEAD --")?.is_empty());

        // Make the next invocation require planning again, without changing the checker policy.
        fs::remove_file(dir.path().join("request.json"))?;
        fs::write(dir.path().join("leaf/src/lib.rs"), "pub fn another_replacement() {}\n")?;
        git(
            dir.path(),
            "commit -am 'change(csr-api-leaf)!: replace the leaf API again'",
        )?;
        let output = release(dir.path(), &[])?;
        assert_eq!(output.status.success(), result != 3, "{output:?}");
        assert!(
            dir.path().join("request.json").exists(),
            "next invocation runs the checker again"
        );
    }
    Ok(())
}

#[test]
fn completed_checkers_are_retained_when_another_checker_fails() -> gix_testtools::Result {
    for (result, expected) in [(0, "1.2.4"), (10, "2.0.0")] {
        let dir = fixture(Some(3), "")?;
        let manifest = dir.path().join("client/Cargo.toml");
        fs::write(
            &manifest,
            format!(
                "{}\n[package.metadata.smart-release]\napi-check = [\"sh\", \"client-check.sh\"]\n",
                fs::read_to_string(&manifest)?
            ),
        )?;
        fs::write(
            dir.path().join("client-check.sh"),
            format!("#!/bin/sh\ncat > client-requests.jsonl\nexit {result}\n"),
        )?;
        git(dir.path(), "add client-check.sh")?;
        git(dir.path(), "commit -am 'configure client checker'")?;
        let output = release(dir.path(), &["--no-api-check"])?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stderr}");
        assert!(stderr.contains("'csr-api-facade' from 1.2.3 to 2.0.0"), "{stderr}");
        assert!(
            stderr.contains(&format!("'csr-api-client' from 1.2.3 to {expected}")),
            "{stderr}"
        );
        assert!(dir.path().join("request.json").exists());
        assert!(dir.path().join("client-requests.jsonl").exists());
    }
    Ok(())
}

#[test]
fn completed_profiles_are_retained_when_another_profile_fails() -> gix_testtools::Result {
    for (stable, experimental, facade, client) in [(0, 3, "1.3.0", "1.2.4"), (3, 0, "2.0.0", "2.0.0")] {
        let dir = fixture(Some(0), "")?;
        fs::write(
            dir.path().join("check.sh"),
            format!(
                r#"#!/bin/sh
request=$(cat)
printf '%s\n' "$request" >> request.json
case "$request" in
    *'"experimental":true'*) exit {experimental} ;;
    *) exit {stable} ;;
esac
"#
            ),
        )?;
        git(dir.path(), "commit -am 'configure profile failure'")?;
        let output = release(dir.path(), &["--no-api-check"])?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stderr}");
        assert!(
            stderr.contains(&format!("'csr-api-facade' from 1.2.3 to {facade}")),
            "{stderr}"
        );
        assert!(
            stderr.contains(&format!("'csr-api-client' from 1.2.3 to {client}")),
            "{stderr}"
        );
        let requests = fs::read_to_string(dir.path().join("request.json"))?
            .lines()
            .map(serde_json::from_str::<serde_json::Value>)
            .collect::<std::result::Result<Vec<_>, _>>()?;
        for (profile, result) in [(false, stable), (true, experimental)] {
            let count = requests
                .iter()
                .filter(|request| (request["experimental"].as_bool() == Some(true)) == profile)
                .count();
            if result == 3 {
                assert_eq!(
                    count, 1,
                    "incomplete comparisons are skipped for the rest of the invocation"
                );
            } else {
                assert!(
                    count > 1,
                    "completed comparisons must be rechecked when candidates change"
                );
            }
        }
        assert!(
            !stderr.contains("Stable API of 'csr-api-facade' requires a breaking release"),
            "an incomplete check is not an incompatibility verdict: {stderr}"
        );
    }
    Ok(())
}

#[test]
fn missing_checker_and_malformed_metadata_abort() -> gix_testtools::Result {
    for config in [
        "api-check = []",
        "api-check = [\"./missing-checker\"]",
        "api-check = true",
        "unstable-features = [\"unstable\"]",
        "api-check = [\"sh\", \"check.sh\"]\nunstable-features = [\"missing\"]",
    ] {
        let dir = fixture(None, "")?;
        let manifest = dir.path().join("facade/Cargo.toml");
        fs::write(
            &manifest,
            format!(
                "{}\n[package.metadata.smart-release]\n{config}\n",
                fs::read_to_string(&manifest)?
            ),
        )?;
        git(dir.path(), "add .")?;
        git(dir.path(), "commit -m 'configure checker'")?;
        let output = release(dir.path(), &[])?;
        assert!(
            !output.status.success(),
            "invalid checker configuration must abort: {config}"
        );
        if config == "api-check = [\"./missing-checker\"]" {
            assert!(String::from_utf8_lossy(&output.stderr).contains("--no-api-check"));
        }
        let output = release(dir.path(), &["--no-api-check"])?;
        assert_eq!(
            output.status.success(),
            config == "api-check = [\"./missing-checker\"]",
            "the override skips execution but still rejects malformed metadata: {config}: {output:?}"
        );
    }
    Ok(())
}

#[test]
fn checks_are_repeated_when_another_boundary_changes_candidate_versions() -> gix_testtools::Result {
    let dir = fixture(Some(10), "")?;
    let root = dir.path();
    let manifest = root.join("client/Cargo.toml");
    fs::write(
        &manifest,
        format!(
            "{}\n[package.metadata.smart-release]\napi-check = [\"sh\", \"client-check.sh\"]\n",
            fs::read_to_string(&manifest)?
        ),
    )?;
    fs::write(
        root.join("client-check.sh"),
        r#"#!/bin/sh
request=$(cat)
printf '%s\n' "$request" >> client-requests.jsonl
case "$request" in
    *'"next_version":"2.0.0"'*) exit 10 ;;
    *) exit 0 ;;
esac
"#,
    )?;
    git(root, "add .")?;
    git(root, "commit -m 'configure dependent API checker'")?;
    let output = release(root, &[])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("'csr-api-client' from 1.2.3 to 2.0.0"), "{stderr}");
    let requests = fs::read_to_string(root.join("client-requests.jsonl"))?
        .lines()
        .map(serde_json::from_str::<serde_json::Value>)
        .collect::<std::result::Result<Vec<_>, _>>()?;
    assert_eq!(
        requests
            .iter()
            .filter(|request| request["experimental"].as_bool() != Some(true))
            .count(),
        2,
        "the dependent is rechecked when the rejected facade changes its candidate version"
    );
    Ok(())
}

#[test]
fn default_and_forwarded_features_count_as_unstable_consumers() -> gix_testtools::Result {
    for (manifest_path, features) in [
        ("facade/Cargo.toml", "default = [\"unstable\"]\n"),
        (
            "client/Cargo.toml",
            "[features]\nexpose = [\"csr-api-facade/experimental\"]\n",
        ),
    ] {
        let dir = fixture(Some(0), "")?;
        let manifest = dir.path().join(manifest_path);
        let original = fs::read_to_string(&manifest)?;
        fs::write(
            &manifest,
            if manifest_path.starts_with("facade") {
                original.replace("[features]\n", &format!("[features]\n{features}"))
            } else {
                format!("{original}\n{features}")
            },
        )?;
        git(dir.path(), "commit -am 'enable unstable forwarding'")?;
        let output = release(dir.path(), &[])?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stderr}");
        assert!(stderr.contains("'csr-api-client' from 1.2.3 to 2.0.0"), "{stderr}");
    }
    Ok(())
}

#[test]
fn direct_breaking_changes_are_not_overridden_by_a_successful_check() -> gix_testtools::Result {
    let dir = fixture(Some(0), "")?;
    fs::write(dir.path().join("facade/src/lib.rs"), "pub fn different() {}\n")?;
    git(
        dir.path(),
        "commit -am 'change(csr-api-facade)!: replace the facade API'",
    )?;
    let output = release(dir.path(), &[])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("'csr-api-facade' from 1.2.3 to 2.0.0"), "{stderr}");
    assert!(stderr.contains("'csr-api-client' from 1.2.3 to 2.0.0"), "{stderr}");
    let commit = git(dir.path(), "rev-parse HEAD")?;
    assert!(
        stderr.contains(&format!(
            "'csr-api-facade' 1.2.3 -> 2.0.0 (publish): breaking commit {}: replace the facade API",
            commit.trim()
        )),
        "{stderr}"
    );
    Ok(())
}

#[test]
fn unstable_dependants_outside_the_selected_graph_receive_manifest_bumps() -> gix_testtools::Result {
    let dir = fixture(Some(0), "")?;
    let root = dir.path();
    let manifest = root.join("Cargo.toml");
    fs::write(
        &manifest,
        fs::read_to_string(&manifest)?.replace("\"client\"]", "\"client\", \"sibling\"]"),
    )?;
    fs::create_dir_all(root.join("sibling/src"))?;
    fs::write(root.join("sibling/src/lib.rs"), "")?;
    fs::write(
        root.join("sibling/Cargo.toml"),
        r#"[package]
name = "csr-api-sibling"
version = "1.2.3"
[dependencies]
csr-api-facade = { path = "../facade", version = "1.2.3", features = ["unstable"] }
"#,
    )?;
    git(root, "add .")?;
    git(root, "commit -m 'add an unstable consumer outside the selected graph'")?;
    let output = release(root, &[])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("'csr-api-sibling' 1.2.3 ➡ 2.0.0"), "{stderr}");
    assert!(
        stderr.contains("'csr-api-sibling' 1.2.3 -> 2.0.0 (manifest only): conservative safety bump due to breaking dependency 'csr-api-leaf'"),
        "{stderr}"
    );
    Ok(())
}

#[test]
fn stable_minor_dependencies_do_not_force_major_releases() -> gix_testtools::Result {
    let dir = fixture(None, "")?;
    let root = dir.path();
    for path in ["leaf/Cargo.toml", "facade/Cargo.toml"] {
        let manifest = root.join(path);
        fs::write(&manifest, fs::read_to_string(&manifest)?.replace("0.2.0", "1.2.3"))?;
    }
    git(root, "commit -am 'stabilize the leaf dependency'")?;
    git(root, "tag csr-api-leaf-v1.2.3")?;
    fs::write(
        root.join("leaf/src/lib.rs"),
        "pub fn replacement() {}\npub fn additional() {}\n",
    )?;
    git(root, "commit -am 'feat(csr-api-leaf): add an API'")?;
    let output = release(root, &[])?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("'csr-api-leaf' from 1.2.3 to 1.3.0"), "{stderr}");
    assert!(stderr.contains("'csr-api-client' from 1.2.3 to 1.2.4"), "{stderr}");
    assert!(!stderr.contains("Stable-crate breaking-change report:"), "{stderr}");
    assert!(
        !stderr.contains("to 2.0.0"),
        "stable minor features remain compatible: {stderr}"
    );
    Ok(())
}

#[test]
fn published_baselines_include_unchanged_dependencies() -> gix_testtools::Result {
    let dir = fixture(Some(0), "")?;
    let root = dir.path();
    for path in ["leaf/Cargo.toml", "facade/Cargo.toml"] {
        let manifest = root.join(path);
        fs::write(&manifest, fs::read_to_string(&manifest)?.replace("0.2.0", "0.3.0"))?;
    }
    git(root, "commit -am 'prepare the next leaf version'")?;
    git(root, "tag csr-api-leaf-v0.3.0")?;
    let (index_path, _) = crates_index::local_path_and_canonical_url_with_hash_kind(
        crates_index::sparse::URL,
        Some(&root.join("cargo-home")),
        &crates_index::HashKind::Stable,
    )?;
    let cache = index_path.join(".cache/cs/r-/csr-api-leaf");
    fs::create_dir_all(cache.parent().expect("the sparse cache entry has a parent"))?;
    fs::write(
        cache,
        concat!(
            "\x03\x02\0\0\0etag: \"test\"\0",
            "0.2.0\0",
            r#"{"name":"csr-api-leaf","vers":"0.2.0","deps":[],"cksum":"0000000000000000000000000000000000000000000000000000000000000000","features":{},"yanked":false}"#,
            "\0"
        ),
    )?;

    let output = release(root, &[])?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let request: serde_json::Value = serde_json::from_str(&fs::read_to_string(root.join("request.json"))?)?;
    let leaf = request["dependencies"]
        .as_array()
        .expect("the request lists dependencies")
        .iter()
        .find(|dep| dep["name"] == "csr-api-leaf")
        .expect("the leaf is a workspace dependency");
    assert_eq!(leaf["current_version"], "0.3.0");
    assert_eq!(leaf["next_version"], "0.3.0");
    assert_eq!(leaf["baseline_version"], "0.2.0");
    assert_eq!(leaf["breaking"], true);
    Ok(())
}

#[test]
fn rejected_api_checks_preserve_explicit_major_bumps() -> gix_testtools::Result {
    let dir = fixture(None, "")?;
    let root = dir.path();
    let manifest = root.join("leaf/Cargo.toml");
    fs::write(
        &manifest,
        format!(
            "{}\n[package.metadata.smart-release]\napi-check = [\"sh\", \"check.sh\", \"10\"]\n",
            fs::read_to_string(&manifest)?
        ),
    )?;
    git(root, "commit -am 'check the leaf API'")?;
    let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-smart-release"));
    let output = gix_testtools::configure_git_environment(&mut command, root)
        .current_dir(root)
        .env("CARGO_HOME", root.join("cargo-home"))
        .env("CARGO_NET_OFFLINE", "true")
        .args([
            "smart-release",
            "csr-api-leaf",
            "--bump",
            "major",
            "--no-bump-on-demand",
            "--registry",
            "unused",
            "--no-publish",
            "--no-push",
            "--no-tag",
            "--no-changelog-github-release",
        ])
        .output()?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("'csr-api-leaf' from 0.2.0 to 1.0.0"), "{stderr}");
    Ok(())
}

#[test]
fn release_preflight_uses_the_release_requests_and_keeps_tracked_files() -> gix_testtools::Result {
    for result in [0, 10] {
        let dir = fixture(Some(result), "")?;
        let root = dir.path();
        let head = git(root, "rev-parse HEAD")?;
        let output = release_package(root, "csr-api-facade", &[])?;
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let request = fs::read_to_string(root.join("request.json"))?;

        let output = preflight(root, "csr-api-facade", "auto")?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(result), "{stderr}");
        assert_eq!(
            fs::read_to_string(root.join("request.json"))?,
            request,
            "preflight and release must check the same baseline and proposed dependency identities"
        );
        assert!(stderr.contains("csr-api-leaf: 0.2.0 -> 0.3.0"), "{stderr}");
        assert!(stderr.contains("csr-api-facade-v1.2.3"), "{stderr}");
        assert_eq!(git(root, "rev-parse HEAD")?, head, "preflight must not create commits");
        assert!(
            git(root, "diff HEAD --")?.is_empty(),
            "preflight must not rewrite tracked files"
        );
    }
    Ok(())
}

#[test]
fn release_preflight_rejects_unchecked_crates_and_checker_failures() -> gix_testtools::Result {
    for (checker, diagnostic) in [
        (None, "has no enabled release API-check policy"),
        (Some(3), "API checker for 'csr-api-facade' failed"),
    ] {
        let dir = fixture(checker, "")?;
        let output = preflight(dir.path(), "csr-api-facade", "auto")?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "{stderr}");
        assert!(stderr.contains(diagnostic), "{stderr}");
        assert!(
            git(dir.path(), "diff HEAD --")?.is_empty(),
            "failed preflight must not rewrite files"
        );
    }
    Ok(())
}

#[test]
fn release_preflight_checks_an_unchanged_selected_crate() -> gix_testtools::Result {
    let dir = fixture(Some(0), "")?;
    let root = dir.path();
    git(root, "tag -f csr-api-leaf-v0.2.0")?;
    fs::write(root.join("facade/src/lib.rs"), "pub fn uncommitted() {}\n")?;
    let dirty = git(root, "diff HEAD --")?;

    let output = preflight(root, "csr-api-facade", "auto")?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    assert!(
        stderr.contains("Stable API of 'csr-api-facade': compatible"),
        "{stderr}"
    );
    assert!(
        root.join("request.json").is_file(),
        "an unchanged selected crate must still be checked"
    );
    assert_eq!(
        git(root, "diff HEAD --")?,
        dirty,
        "preflight must preserve uncommitted changes"
    );
    Ok(())
}

#[test]
fn release_preflight_does_not_rewrite_a_stale_tracked_lockfile() -> gix_testtools::Result {
    let dir = fixture(Some(0), "")?;
    let root = dir.path();
    let initial = preflight(root, "csr-api-facade", "auto")?;
    assert!(initial.status.success(), "{}", String::from_utf8_lossy(&initial.stderr));
    let lockfile = fs::read(root.join("Cargo.lock"))?;
    git(root, "add -f Cargo.lock")?;
    git(root, "commit -m 'track the release lockfile'")?;
    for path in ["leaf/Cargo.toml", "facade/Cargo.toml"] {
        let manifest = root.join(path);
        fs::write(&manifest, fs::read_to_string(&manifest)?.replace("0.2.0", "0.3.0"))?;
    }
    let dirty = git(root, "diff HEAD --")?;

    let output = preflight(root, "csr-api-facade", "auto")?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(2), "{stderr}");
    assert!(stderr.contains("--locked"), "{stderr}");
    assert_eq!(
        fs::read(root.join("Cargo.lock"))?,
        lockfile,
        "stale lockfiles must remain byte-for-byte unchanged"
    );
    assert_eq!(
        git(root, "diff HEAD --")?,
        dirty,
        "preflight must preserve all tracked changes"
    );
    Ok(())
}

#[test]
fn release_preflight_accounts_for_manually_bumped_dependency_versions() -> gix_testtools::Result {
    let dir = fixture(Some(0), "")?;
    let root = dir.path();
    for path in ["leaf/Cargo.toml", "facade/Cargo.toml"] {
        let manifest = root.join(path);
        fs::write(&manifest, fs::read_to_string(&manifest)?.replace("0.2.0", "0.3.0"))?;
    }
    let (index_path, _) = crates_index::local_path_and_canonical_url_with_hash_kind(
        crates_index::sparse::URL,
        Some(&root.join("cargo-home")),
        &crates_index::HashKind::Stable,
    )?;
    let cache = index_path.join(".cache/cs/r-/csr-api-leaf");
    fs::create_dir_all(cache.parent().expect("the sparse cache entry has a parent"))?;
    fs::write(
        cache,
        concat!(
            "\x03\x02\0\0\0etag: \"test\"\0",
            "0.2.0\0",
            r#"{"name":"csr-api-leaf","vers":"0.2.0","deps":[],"cksum":"0000000000000000000000000000000000000000000000000000000000000000","features":{},"yanked":false}"#,
            "\0"
        ),
    )?;
    let dirty = git(root, "diff HEAD --")?;

    let output = preflight(root, "csr-api-facade", "keep")?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(0), "{stderr}");
    let request: serde_json::Value = serde_json::from_str(&fs::read_to_string(root.join("request.json"))?)?;
    let leaf = request["dependencies"]
        .as_array()
        .expect("the request lists dependencies")
        .iter()
        .find(|dep| dep["name"] == "csr-api-leaf")
        .expect("the leaf is a workspace dependency");
    assert_eq!(leaf["current_version"], "0.3.0");
    assert_eq!(leaf["next_version"], "0.3.0");
    assert_eq!(leaf["baseline_version"], "0.2.0");
    assert_eq!(
        leaf["breaking"], true,
        "the published identity, not the edited manifest, defines compatibility"
    );
    assert_eq!(
        git(root, "diff HEAD --")?,
        dirty,
        "preflight must preserve manually bumped manifests"
    );
    Ok(())
}

fn preflight(root: &Path, package: &str, bump_dependencies: &str) -> std::io::Result<std::process::Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-api-check"));
    gix_testtools::configure_git_environment(&mut command, root)
        .current_dir(root)
        .env("CARGO_HOME", root.join("cargo-home"))
        .env("CARGO_NET_OFFLINE", "true")
        .args([
            "api-check",
            package,
            "--release-plan",
            "--bump",
            "patch",
            "--bump-dependencies",
            bump_dependencies,
            "--no-bump-on-demand",
            "--auto-publish-of-stable-crates",
        ])
        .output()
}

fn fixture(result: Option<i32>, dependency_features: &str) -> gix_testtools::Result<tempfile::TempDir> {
    let dir = tempfile::tempdir()?;
    let root = dir.path();
    fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"leaf\", \"facade\", \"client\"]\nresolver = \"2\"\n",
    )?;
    for (directory, version, dependency) in [
        ("leaf", "0.2.0", String::new()),
        ("facade", "1.2.3", "[dependencies]\ncsr-api-leaf = { path = \"../leaf\", version = \"0.2.0\" }\n".into()),
        ("client", "1.2.3", format!("[dependencies]\ncsr-api-facade = {{ path = \"../facade\", version = \"1.2.3\", {dependency_features} }}\n")),
    ] {
        fs::create_dir_all(root.join(directory).join("src"))?;
        fs::write(root.join(directory).join("src/lib.rs"), "pub fn value() {}\n")?;
        let metadata = if directory == "facade" {
            let check = result.map_or(String::new(), |result| format!("[package.metadata.smart-release]\napi-check = [\"sh\", \"check.sh\", \"{result}\"]\nunstable-features = [\"unstable\"]\n"));
            format!("[features]\nunstable = []\nexperimental = [\"unstable\"]\n{check}")
        } else { String::new() };
        fs::write(root.join(directory).join("Cargo.toml"), format!("[package]\nname = \"csr-api-{directory}\"\nversion = \"{version}\"\nedition = \"2021\"\n{dependency}{metadata}"))?;
    }
    fs::write(root.join("check.sh"), "#!/bin/sh\ncat > request.json\nexit \"$1\"\n")?;
    fs::write(
        root.join(".gitignore"),
        "/cargo-home/\n/Cargo.lock\n/request.json\n/target/\n",
    )?;
    git(root, "init")?;
    git(root, "add .")?;
    git(root, "commit -m initial")?;
    for tag in ["csr-api-leaf-v0.2.0", "csr-api-facade-v1.2.3", "csr-api-client-v1.2.3"] {
        git(root, &format!("tag {tag}"))?;
    }
    fs::write(root.join("leaf/src/lib.rs"), "pub fn replacement() {}\n")?;
    fs::write(
        root.join("client/src/lib.rs"),
        "pub fn value() {} // adapt to the leaf update\n",
    )?;
    git(root, "commit -am 'change(csr-api-leaf)!: replace the leaf API'")?;
    Ok(dir)
}

fn release(root: &Path, extra: &[&str]) -> std::io::Result<std::process::Output> {
    release_package(root, "csr-api-client", extra)
}

fn release_package(root: &Path, package: &str, extra: &[&str]) -> std::io::Result<std::process::Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-smart-release"));
    gix_testtools::configure_git_environment(&mut command, root)
        .current_dir(root)
        .env("CARGO_HOME", root.join("cargo-home"))
        .env("CARGO_NET_OFFLINE", "true")
        .args([
            "smart-release",
            package,
            "--bump",
            "patch",
            "--bump-dependencies",
            "auto",
            "--no-bump-on-demand",
            "--auto-publish-of-stable-crates",
            "--registry",
            "unused",
            "--no-publish",
            "--no-push",
            "--no-tag",
            "--no-changelog-github-release",
        ])
        .args(extra)
        .output()
}
