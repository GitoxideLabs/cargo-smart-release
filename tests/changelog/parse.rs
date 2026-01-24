use std::path::Path;

use cargo_smart_release::{
    changelog::{
        section::{segment, Segment},
        Section, Version,
    },
    ChangeLog,
};

#[cfg(not(windows))]
fn fixup(v: String) -> String {
    v
}

#[cfg(windows)]
fn fixup(v: String) -> String {
    // Git checks out text files with line ending conversions, git itself will of course not put '\r\n' anywhere,
    // so that wouldn't be expected in an object and doesn't have to be parsed.
    v.replace("\r\n", "\n")
}

fn fixture(name: &str) -> std::io::Result<String> {
    let data = std::fs::read_to_string(gix_testtools::fixture_path(
        Path::new("changelog").join("parse").join(name),
    ))?;
    Ok(fixup(data))
}

#[test]
fn all_unknown_in_section() {
    let fixture = fixture("known-section-unknown-content.md").unwrap();
    let log = ChangeLog::from_markdown(&fixture);
    assert_eq!(
        log.sections,
        vec![
            Section::Release {
                name: Version::Unreleased,
                removed_messages: vec![],
                date: None,
                heading_level: 3,
                version_prefix: "".into(),
                segments: vec![Segment::User {
                    markdown: "- hello ~~this is not understood~~\n* this isn't either\n\n".into()
                }],
                unknown: String::new(),
            },
            Section::Release {
                name: Version::Semantic("1.0.0".parse().unwrap()),
                removed_messages: vec![],
                date: None,
                heading_level: 4,
                version_prefix: Section::DEFAULT_PREFIX.into(),
                segments: vec![Segment::User {
                    markdown: "Some free text in a paragraph\nthat won't parse.\n".into()
                }],
                unknown: String::new(),
            }
        ]
    )
}

#[test]
fn unknown_link_and_headline() {
    let fixture = fixture("known-section-unknown-headline-with-link.md").unwrap();
    let log = ChangeLog::from_markdown(&fixture);
    assert_eq!(
        log.sections,
        vec![Section::Release {
            name: Version::Unreleased,
            removed_messages: vec![],
            date: None,
            heading_level: 4,
            version_prefix: "".into(),
            segments: vec![Segment::User {
                markdown: "##### Special\n\nHello [there][194] period.\n".into()
            }],
            unknown: String::new(),
        },]
    )
}

#[test]
fn known_and_unknown_sections_are_sorted() {
    let fixture = fixture("unknown-known-unknown-known-unsorted.md").unwrap();
    let log = ChangeLog::from_markdown(&fixture);
    assert_eq!(
        log.sections,
        vec![
            Section::Verbatim {
                text: "Hello, this is a changelog.\n\n".into(),
                generated: false
            },
            Section::Release {
                name: Version::Unreleased,
                removed_messages: vec![],
                date: None,
                heading_level: 3,
                version_prefix: "".into(),
                unknown: "".into(),
                segments: vec![Segment::User {
                    markdown: "TBD\n".into()
                }]
            },
            Section::Release {
                name: Version::Semantic(semver::Version::parse("1.0.0").unwrap()),
                removed_messages: vec![],
                date: None,
                heading_level: 3,
                version_prefix: Section::DEFAULT_PREFIX.into(),
                unknown: "".into(),
                segments: vec![
                    Segment::User {
                        markdown: "- initial release\n\n".into()
                    },
                    Segment::User {
                        markdown: "### Something in between\n\nintermezzo\n".into()
                    },
                ]
            },
        ],
    )
}

#[test]
fn releases_are_sorted_by_date() {
    let fixture = fixture("releases-sorted-by-date.md").unwrap();
    let log = ChangeLog::from_markdown(&fixture);

    // Extract the version numbers and dates from the parsed sections
    let release_versions: Vec<_> = log
        .sections
        .iter()
        .filter_map(|s| match s {
            Section::Release {
                name: Version::Semantic(v),
                date,
                ..
            } => Some((v.clone(), date.clone())),
            _ => None,
        })
        .collect();

    // Verify they are sorted by date (newest first): 0.77.0, 0.76.0, 0.75.0, 0.74.1, 0.74.0
    assert_eq!(release_versions.len(), 5);
    assert_eq!(release_versions[0].0, semver::Version::parse("0.77.0").unwrap());
    assert_eq!(release_versions[1].0, semver::Version::parse("0.76.0").unwrap());
    assert_eq!(release_versions[2].0, semver::Version::parse("0.75.0").unwrap());
    assert_eq!(release_versions[3].0, semver::Version::parse("0.74.1").unwrap());
    assert_eq!(release_versions[4].0, semver::Version::parse("0.74.0").unwrap());
}

/// Test for issue #103: Nested unordered list items should not repeat on each release.
/// When a changelog entry has a nested list like:
/// ```
///  - <csr-id-...> Added the following methods:
///    - `is_empty`
///    - `len`
/// ```
/// The nested items should be preserved in the body and not cause accumulation
/// in `<csr-unknown>` sections on repeated parse/write cycles.
#[test]
fn nested_list_items_with_csr_id_round_trips_stably() {
    use cargo_smart_release::changelog::write::{Components, Linkables};

    let input = r#"## v0.1.2 (2021-08-06)

### Added

 - <csr-id-0000000000000000000000000000000000000002/> Added the following methods to `GitConfig`:
   - `is_empty`
   - `len`
   - `from_env`
   - `open`
"#;

    let log = ChangeLog::from_markdown(input);

    // Verify the nested list items are properly captured in the body
    assert_eq!(log.sections.len(), 1);
    match &log.sections[0] {
        Section::Release {
            segments, unknown, ..
        } => {
            // unknown should be empty - no accumulation
            assert!(unknown.is_empty(), "unknown should be empty, got: {unknown:?}");
            // There should be exactly one Conventional segment
            assert_eq!(segments.len(), 1);
            match &segments[0] {
                Segment::Conventional(segment::Conventional { messages, .. }) => {
                    // There should be exactly one message (the generated one with nested items in body)
                    assert_eq!(messages.len(), 1);
                    match &messages[0] {
                        segment::conventional::Message::Generated { title, body, .. } => {
                            assert_eq!(title, "Added the following methods to `GitConfig`:");
                            // The body should contain all nested list items
                            let body = body.as_ref().expect("body should be present");
                            assert!(body.contains("`is_empty`"), "body should contain is_empty");
                            assert!(body.contains("`len`"), "body should contain len");
                            assert!(body.contains("`from_env`"), "body should contain from_env");
                            assert!(body.contains("`open`"), "body should contain open");
                        }
                        _ => panic!("Expected Generated message"),
                    }
                }
                _ => panic!("Expected Conventional segment"),
            }
        }
        _ => panic!("Expected Release section"),
    }

    // Test round-trip stability: parse → write → parse → write should be stable
    let mut output1 = String::new();
    log.write_to(&mut output1, &Linkables::AsText, Components::all(), false)
        .unwrap();

    let log2 = ChangeLog::from_markdown(&output1);
    let mut output2 = String::new();
    log2.write_to(&mut output2, &Linkables::AsText, Components::all(), false)
        .unwrap();

    // Multiple round-trips should produce identical output
    for round in 3..=5 {
        let log_n = ChangeLog::from_markdown(&output2);
        let mut output_n = String::new();
        log_n
            .write_to(&mut output_n, &Linkables::AsText, Components::all(), false)
            .unwrap();
        assert_eq!(
            output2, output_n,
            "Round {round} output differs from previous - nested lists not stable"
        );
    }
}

/// Test that user messages with nested lists (no csr-id) also round-trip correctly.
#[test]
fn user_message_with_nested_list_round_trips_stably() {
    use cargo_smart_release::changelog::write::{Components, Linkables};

    let input = r#"## v0.1.2 (2021-08-06)

### Added

 - Added the following methods to `GitConfig`:
   - `is_empty`
   - `len`
   - `from_env`
   - `open`
"#;

    let log = ChangeLog::from_markdown(input);

    // Verify it parses as a User message (not Generated, since no csr-id)
    assert_eq!(log.sections.len(), 1);
    match &log.sections[0] {
        Section::Release {
            segments, unknown, ..
        } => {
            assert!(unknown.is_empty(), "unknown should be empty");
            assert_eq!(segments.len(), 1);
            match &segments[0] {
                Segment::Conventional(segment::Conventional { messages, .. }) => {
                    assert_eq!(messages.len(), 1);
                    match &messages[0] {
                        segment::conventional::Message::User { markdown } => {
                            // The user markdown should preserve the nested list structure
                            assert!(
                                markdown.contains("`is_empty`"),
                                "markdown should contain is_empty"
                            );
                            assert!(markdown.contains("`len`"), "markdown should contain len");
                        }
                        _ => panic!("Expected User message"),
                    }
                }
                _ => panic!("Expected Conventional segment"),
            }
        }
        _ => panic!("Expected Release section"),
    }

    // Test round-trip stability
    let mut output1 = String::new();
    log.write_to(&mut output1, &Linkables::AsText, Components::all(), false)
        .unwrap();

    let log2 = ChangeLog::from_markdown(&output1);
    let mut output2 = String::new();
    log2.write_to(&mut output2, &Linkables::AsText, Components::all(), false)
        .unwrap();

    assert_eq!(output1, output2, "User message with nested list should round-trip stably");
}
