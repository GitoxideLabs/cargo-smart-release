use std::process::Stdio;

use anyhow::{anyhow, Context};
use cargo_metadata::{
    camino::{Utf8Component, Utf8Path},
    Dependency, DependencyKind, Metadata, Package, PackageId,
};
use gix::bstr::{BStr, ByteSlice};
use semver::{Version, VersionReq};

pub struct Program {
    pub found: bool,
}

impl Program {
    pub fn named(name: &'static str) -> Self {
        Program {
            found: std::process::Command::new(name)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .is_ok(),
        }
    }
}

pub fn will(not_really: bool) -> &'static str {
    if not_really {
        "WOULD"
    } else {
        "Will"
    }
}

/// Run `formatter` with POSIX-style argument splitting but no shell, feeding it `markdown` on stdin and returning its stdout.
pub fn format_markdown(formatter: &str, markdown: &str) -> anyhow::Result<String> {
    use std::io::Write;

    let parsed = gix::command::parse::command_line(formatter.as_bytes().as_bstr())
        .map_err(anyhow::Error::from)
        .context("Could not parse Markdown formatter command")?;
    let mut child = std::process::Command::new(parsed.command)
        .args(parsed.args)
        .envs(parsed.env)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| anyhow!("Markdown formatter stdin was not piped"))?;
    // Drain both output pipes while supplying input, so a streaming formatter cannot deadlock on a full pipe.
    let (output, input_result) = std::thread::scope(|scope| {
        let writer = scope.spawn(move || stdin.write_all(markdown.as_bytes()));
        let output = child.wait_with_output();
        (output, writer.join())
    });
    let output = output?;
    anyhow::ensure!(
        output.status.success(),
        "Markdown formatter '{}' failed ({}): {}",
        formatter,
        output.status,
        String::from_utf8_lossy(&output.stderr).trim()
    );
    input_result
        .map_err(|_| anyhow!("Markdown formatter stdin writer panicked"))?
        .context("Could not write Markdown to formatter stdin")?;
    Ok(String::from_utf8(output.stdout)?)
}

pub fn try_to_published_crate_and_new_version<'meta, 'a>(
    c: &'a crate::traverse::Dependency<'meta>,
) -> Option<(&'meta Package, &'a semver::Version)> {
    match &c.mode {
        crate::traverse::dependency::Mode::ToBePublished { adjustment } => {
            Some((c.package, &adjustment.bump().next_release))
        }
        _ => None,
    }
}

pub fn is_pre_release_version(semver: &Version) -> bool {
    semver.major == 0
}

pub fn is_top_level_package(manifest_path: &Utf8Path, repo: &gix::Repository) -> bool {
    manifest_path
        .strip_prefix(
            std::env::current_dir()
                .expect("cwd")
                .join(repo.workdir().as_ref().expect("repo with working tree")),
        )
        .is_ok_and(|p| p.components().count() == 1)
}

pub fn version_req_unset_or_default(req: &VersionReq) -> bool {
    req.comparators.last().is_none_or(|comp| comp.op == semver::Op::Caret)
}

pub fn package_eq_dependency_ignore_dev_without_version(package: &Package, dependency: &Dependency) -> bool {
    (dependency.kind != DependencyKind::Development || !version_req_unset_or_default(&dependency.req))
        && package.name.as_str() == dependency.name
}

pub fn workspace_package_by_dependency<'a>(meta: &'a Metadata, dep: &Dependency) -> Option<&'a Package> {
    meta.packages
        .iter()
        .find(|p| p.name.as_str() == dep.name && p.source.as_ref().is_none_or(|s| !s.is_crates_io()))
        .filter(|p| meta.workspace_members.iter().any(|m| m == &p.id))
}

pub fn package_by_name<'a>(meta: &'a Metadata, name: &str) -> anyhow::Result<&'a Package> {
    meta.packages
        .iter()
        .find(|p| p.name.as_str() == name && p.source.as_ref().is_none_or(|s| !s.is_crates_io()))
        .ok_or_else(|| anyhow!("workspace member '{}' must be a listed package", name))
}

pub fn names_and_versions<'a>(publishees: impl IntoIterator<Item = &'a (&'a Package, &'a semver::Version)>) -> String {
    publishees
        .into_iter()
        .map(|(p, nv)| format!("{} v{}", p.name, nv))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn package_by_id<'a>(meta: &'a Metadata, id: &PackageId) -> &'a Package {
    meta.packages
        .iter()
        .find(|p| &p.id == id)
        .expect("workspace members are in packages")
}

pub fn tag_prefix<'p>(package: &'p Package, repo: &gix::Repository) -> Option<&'p str> {
    if is_top_level_package(&package.manifest_path, repo) {
        None
    } else {
        Some(&package.name)
    }
}

pub fn tag_name(package: &Package, version: &semver::Version, repo: &gix::Repository) -> String {
    tag_name_inner(tag_prefix(package, repo), version)
}

pub fn release_tag_name(
    package: &Package,
    version: &semver::Version,
    repo: &gix::Repository,
) -> anyhow::Result<String> {
    let name = tag_name(package, version, repo);
    if let Some(prefix) = tag_prefix(package, repo) {
        if repo.try_find_reference(format!("refs/tags/{name}").as_str())?.is_none() {
            let legacy_name = format!("{prefix}-v{version}");
            if repo
                .try_find_reference(format!("refs/tags/{legacy_name}").as_str())?
                .is_some()
            {
                return Ok(legacy_name);
            }
        }
    }
    Ok(name)
}

fn tag_name_inner(package_name: Option<&str>, version: &semver::Version) -> String {
    match package_name {
        Some(name) => format!("{name}/v{version}"),
        None => format!("v{version}"),
    }
}

pub fn parse_possibly_prefixed_tag_version(package_name: Option<&str>, tag_name: &BStr) -> Option<Version> {
    match package_name {
        Some(name) => tag_name
            .strip_prefix(name.as_bytes())
            .and_then(|r| r.strip_prefix(b"/").or_else(|| r.strip_prefix(b"-")))
            .and_then(|possibly_version| parse_tag_version(possibly_version.as_bstr())),
        None => parse_tag_version(tag_name),
    }
}

pub fn parse_tag_version(name: &BStr) -> Option<Version> {
    let version = name
        .strip_prefix(b"vers")
        .or_else(|| name.strip_prefix(b"v"))
        .unwrap_or_else(|| name.as_bytes())
        .to_str()
        .ok()?;
    Version::parse(version).ok()
}

pub fn is_tag_version(name: &gix::bstr::BStr) -> bool {
    parse_tag_version(name).is_some()
}

pub fn component_to_bytes(c: Utf8Component<'_>) -> &[u8] {
    match c {
        Utf8Component::Normal(c) => c.as_bytes(),
        _ => unreachable!("only normal components are possible in paths here"),
    }
}

pub fn time_to_zoned_time(time: gix::date::Time) -> anyhow::Result<jiff::Zoned> {
    Ok(jiff::Timestamp::new(time.seconds, 0)?.to_zoned(jiff::tz::Offset::from_seconds(time.offset)?.to_time_zone()))
}

#[cfg(test)]
mod tests {
    mod format_markdown {
        use super::super::format_markdown;

        #[test]
        fn rejects_invalid_commands() {
            for (command, expected) in [
                ("", "missing command"),
                (" \t\n", "missing command"),
                ("FORMATTER_TEST_VALUE=value", "missing command"),
                ("printf 'unterminated", "missing closing quote"),
                ("printf \\", "missing byte after escape"),
            ] {
                let err = format_markdown(command, "").unwrap_err();
                assert_eq!(err.to_string(), "Could not parse Markdown formatter command");
                assert!(format!("{err:#}").contains(expected), "{err:#}");
            }
        }

        #[cfg(unix)]
        #[test]
        fn honors_quotes_and_escapes() {
            let output = format_markdown(
                r#""printf" '<%s>\n' 'single quoted' "double quoted" escaped\ space '' "" "escaped \"quote\"""#,
                "",
            )
            .unwrap();
            assert_eq!(
                output,
                "<single quoted>\n<double quoted>\n<escaped space>\n<>\n<>\n<escaped \"quote\">\n"
            );
        }

        #[cfg(unix)]
        #[test]
        fn does_not_interpret_shell_syntax() {
            let output = format_markdown(r#"printf '%s\n' $HOME $(echo) `echo` ; | > *.md ~"#, "").unwrap();
            assert_eq!(output, "$HOME\n$(echo)\n`echo`\n;\n|\n>\n*.md\n~\n");
        }

        #[cfg(unix)]
        #[test]
        fn applies_leading_environment_assignments() {
            let output = format_markdown("FORMATTER_TEST_VALUE='a value' printenv FORMATTER_TEST_VALUE", "").unwrap();
            assert_eq!(output, "a value\n");
        }

        #[cfg(unix)]
        #[test]
        fn feeds_markdown_to_stdin_and_returns_stdout() {
            let output = format_markdown("tr '[:lower:]' '[:upper:]'", "# Title\n").unwrap();
            assert_eq!(output, "# TITLE\n");
        }

        #[cfg(unix)]
        #[test]
        fn drains_output_while_writing_large_input() {
            let markdown = "# Large changelog entry\n".repeat(128 * 1024);
            let expected = markdown.clone();
            let (sender, receiver) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let _ = sender.send(format_markdown("tee /dev/stderr", &markdown));
            });
            let output = receiver
                .recv_timeout(std::time::Duration::from_secs(10))
                .expect("formatter must finish even when stdin, stdout, and stderr exceed pipe capacity")
                .unwrap();
            assert_eq!(output, expected);
        }

        #[cfg(unix)]
        #[test]
        fn reports_formatter_failure_when_stdin_closes_early() {
            let markdown = "# Large changelog entry\n".repeat(128 * 1024);
            let err = format_markdown("cat --formatter-invalid-option", &markdown).unwrap_err();
            let message = err.to_string();
            assert!(
                message.starts_with("Markdown formatter 'cat --formatter-invalid-option' failed ("),
                "{message}"
            );
            let (_, stderr) = message.split_once("): ").unwrap();
            assert!(!stderr.is_empty(), "{message}");
        }

        #[cfg(unix)]
        #[test]
        fn reports_write_failure_even_if_formatter_succeeds() {
            let markdown = "# Large changelog entry\n".repeat(128 * 1024);
            let err = format_markdown("true", &markdown).unwrap_err();
            assert_eq!(err.to_string(), "Could not write Markdown to formatter stdin");
            assert_eq!(
                err.downcast_ref::<std::io::Error>().unwrap().kind(),
                std::io::ErrorKind::BrokenPipe
            );
        }
    }

    #[test]
    fn release_tags_use_crate_namespaces() {
        let version = semver::Version::parse("1.2.3-beta.1+build.2").unwrap();
        assert_eq!(super::tag_name_inner(None, &version), "v1.2.3-beta.1+build.2");
        assert_eq!(
            super::tag_name_inner(Some("some-crate"), &version),
            "some-crate/v1.2.3-beta.1+build.2"
        );
    }

    #[test]
    fn namespaced_and_legacy_release_tags_are_recognized() {
        use gix::bstr::ByteSlice;

        for tag in ["some-crate/v1.2.3", "some-crate-v1.2.3"] {
            assert_eq!(
                super::parse_possibly_prefixed_tag_version(Some("some-crate"), tag.as_bytes().as_bstr()),
                semver::Version::parse("1.2.3").ok()
            );
        }
        for tag in ["some-crate-other/v1.2.3", "other-crate/v1.2.3", "some-crate/invalid"] {
            assert!(super::parse_possibly_prefixed_tag_version(Some("some-crate"), tag.as_bytes().as_bstr()).is_none());
        }
    }

    mod parse_possibly_prefixed_tag_version {
        mod matches {
            use std::str::FromStr;

            use gix::bstr::ByteSlice;
            use semver::Version;

            use crate::utils::{parse_possibly_prefixed_tag_version, tag_name_inner};

            #[test]
            fn whatever_tag_name_would_return() {
                assert_eq!(
                    parse_possibly_prefixed_tag_version(
                        "git-test".into(),
                        tag_name_inner("git-test".into(), &Version::from_str("1.0.1").unwrap())
                            .as_bytes()
                            .as_bstr()
                    ),
                    Version::parse("1.0.1").expect("valid").into()
                );

                assert_eq!(
                    parse_possibly_prefixed_tag_version(
                        "single".into(),
                        tag_name_inner("single".into(), &Version::from_str("0.0.1-beta.1").unwrap())
                            .as_bytes()
                            .as_bstr()
                    ),
                    Version::parse("0.0.1-beta.1").expect("valid").into()
                );

                assert_eq!(
                    parse_possibly_prefixed_tag_version(
                        None,
                        tag_name_inner(None, &Version::from_str("0.0.1+123.x").unwrap())
                            .as_bytes()
                            .as_bstr()
                    ),
                    Version::parse("0.0.1+123.x").expect("valid").into()
                );
            }
        }
    }

    mod is_tag_version {
        mod no_match {
            use gix::bstr::ByteSlice;

            use crate::utils::is_tag_version;

            #[test]
            fn not_enough_numbers() {
                assert!(!is_tag_version(b"v0.0".as_bstr()));
            }

            #[test]
            fn funky() {
                assert!(!is_tag_version(b"vHi.Ho.yada-anythingreally".as_bstr()));
            }

            #[test]
            fn prefixed() {
                assert!(!is_tag_version(b"cargo-v1.0.0".as_bstr()));
            }
        }
        mod matches {
            use gix::bstr::ByteSlice;

            #[test]
            fn no_prefix() {
                assert!(is_tag_version(b"0.0.1".as_bstr()));
            }

            #[test]
            fn custom_prefix() {
                assert!(is_tag_version(b"vers0.0.1".as_bstr()));
            }

            use crate::utils::is_tag_version;

            #[test]
            fn pre_release() {
                assert!(is_tag_version(b"v0.0.1".as_bstr()));
                assert!(is_tag_version(b"v0.10.0-beta.1".as_bstr()));
            }

            #[test]
            fn production() {
                assert!(is_tag_version(b"v1.0.1-alpha.1".as_bstr()));
                assert!(is_tag_version(b"v18.10.0+meta".as_bstr()));
            }
        }
    }
}
