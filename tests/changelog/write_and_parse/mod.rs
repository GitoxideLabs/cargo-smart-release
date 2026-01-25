use std::{collections::BTreeMap, convert::TryFrom};

use cargo_smart_release::{
    changelog,
    changelog::{section, section::segment::conventional, Section},
    ChangeLog,
};
use gix_testtools::bstr::ByteSlice;

use crate::{changelog::hex_to_id, Result};

/// Test for issue #30: Top-level unordered lists in commit message bodies should not
/// be flattened into separate changelog entries.
///
/// When a conventional commit has a body containing a top-level unordered list like:
/// ```
/// fix: Remove hidden bogosort functionality
///
/// If users turn out to be depending on bogosort, we may:
///
/// - Add instructions for using an earlier version.
/// - Add back bogosort and document it properly.
/// ```
///
/// The list items in the body should remain as part of the same changelog entry,
/// not become separate top-level entries in the changelog.
#[test]
fn issue_30_body_with_unordered_list_does_not_flatten() -> Result {
    let log = ChangeLog {
        sections: vec![Section::Release {
            heading_level: 2,
            version_prefix: Section::DEFAULT_PREFIX.into(),
            date: Some(jiff::Timestamp::new(0, 0)?.to_zoned(jiff::tz::TimeZone::UTC)),
            name: changelog::Version::Semantic("1.0.0".parse()?),
            removed_messages: vec![],
            segments: vec![section::Segment::Conventional(section::segment::Conventional {
                kind: "fix",
                is_breaking: false,
                removed: vec![],
                messages: vec![
                    conventional::Message::Generated {
                        id: hex_to_id("0000000000000000000000000000000000000001"),
                        title: "Remove hidden bogosort functionality".into(),
                        body: Some(
                            "If users turn out to be depending on bogosort, we may:\n\n\
                             - Add instructions for using an earlier version.\n\
                             - Add back bogosort and document it properly.\n\n\
                             We defaulted to bogosort on Tuesdays based on these mistaken beliefs:\n\n\
                             - Bogosort runs in O(n log n log log n log log log n) if you have an odd number of RAM sticks.\n\
                             - The software can never be run on Tuesday."
                                .into(),
                        ),
                    },
                    conventional::Message::Generated {
                        id: hex_to_id("0000000000000000000000000000000000000002"),
                        title: "Time zones are remembered across sessions".into(),
                        body: None,
                    },
                ],
            })],
            unknown: String::new(),
        }],
    };

    // Write the changelog to markdown
    let mut md = String::new();
    log.write_to(
        &mut md,
        &changelog::write::Linkables::AsText,
        changelog::write::Components::all(),
        false,
    )?;

    // Verify the markdown structure: There should be exactly 2 top-level bullet points
    // (one for each conventional::Message::Generated), not 6 (as would happen if
    // the list items in the body were flattened).
    let top_level_bullets = md.lines().filter(|line| line.starts_with(" - ")).count();
    assert_eq!(
        top_level_bullets, 2,
        "Expected 2 top-level bullet points (one per message), but got {}.\n\
         The body's list items may have been flattened into separate entries.\n\
         Markdown:\n{}",
        top_level_bullets, md
    );

    // Parse back and verify round-trip stability
    let parsed_log = ChangeLog::from_markdown(&md);
    assert_eq!(parsed_log, log, "should round-trip losslessly");

    insta::assert_snapshot!(md);
    Ok(())
}

#[test]
fn conventional_write_empty_messages() -> Result {
    let first_message = hex_to_id("0000000000000000000000000000000000000001");
    let second_message = hex_to_id("0000000000000000000000000000000000000002");

    let log = ChangeLog {
        sections: vec![Section::Release {
            heading_level: 4,
            version_prefix: Section::DEFAULT_PREFIX.into(),
            date: Some(jiff::Timestamp::new(0, 0)?.to_zoned(jiff::tz::TimeZone::UTC)),
            name: changelog::Version::Semantic("1.0.2-beta.2".parse()?),
            removed_messages: vec![second_message],
            segments: vec![section::Segment::Conventional(section::segment::Conventional {
                kind: "feat",
                is_breaking: true,
                removed: vec![first_message],
                messages: vec![
                    conventional::Message::User {
                        markdown: " - verbatim `whatever` the _user_ writes [hello](world)".into(),
                    },
                    conventional::Message::Generated {
                        id: hex_to_id("0000000000000000000000000000000000000003"),
                        title: "this messages comes straight from git conventional and _may_ contain markdown".into(),
                        body: Some("first line\nsecond line\n\nanother paragraph".into()),
                    },
                    conventional::Message::Generated {
                        id: hex_to_id("0000000000000000000000000000000000000004"),
                        title: "spelling. Hello".into(),
                        body: None,
                    },
                    conventional::Message::User {
                        markdown:
                            " - just another user message, this time\n   with multiple lines\n\n   and a new paragraph"
                                .into(),
                    },
                ],
            })],
            unknown: String::new(),
        }],
    };

    for link_mode in &[
        changelog::write::Linkables::AsText,
        changelog::write::Linkables::AsLinks {
            repository_url: gix::Url::try_from(b"https://github.com/user/repo.git".as_bstr())?.into(),
        },
    ] {
        let log = log.clone();
        for _round in 1..=2 {
            let mut md = String::new();
            log.write_to(&mut md, link_mode, changelog::write::Components::all(), false)?;
            insta::assert_snapshot!(md);

            let parsed_log = ChangeLog::from_markdown(&md);
            assert_eq!(parsed_log, log, "we can parse this back losslessly");
        }
    }
    for components in &[
        changelog::write::Components::empty(),
        changelog::write::Components::all(),
    ] {
        for section in &log.sections {
            let mut buf = String::new();
            section.write_to(&mut buf, &changelog::write::Linkables::AsText, *components, false)?;
            insta::assert_snapshot!(buf);
        }
    }
    Ok(())
}

#[test]
fn all_section_types_round_trips_lossy() -> Result {
    let log = ChangeLog {
        sections: vec![
            Section::Verbatim {
                text: "# Changelog\n\nmy very own header\n\n".into(),
                generated: false,
            },
            Section::Release {
                heading_level: 2,
                removed_messages: vec![],
                date: None,
                name: changelog::Version::Unreleased,
                version_prefix: "".into(),
                segments: Vec::new(),
                unknown: "hello\nworld\n".into(),
            },
            Section::Release {
                heading_level: 4,
                version_prefix: "".into(),
                removed_messages: vec![],
                date: Some(jiff::Timestamp::new(0, 0)?.to_zoned(jiff::tz::TimeZone::UTC)),
                name: changelog::Version::Semantic("1.0.2-beta.2".parse()?),
                segments: vec![
                    section::Segment::User {
                        markdown: "* hello world\n\tthis\n\n".into(),
                    },
                    section::Segment::Clippy(section::Data::Generated(section::segment::ThanksClippy { count: 42 })),
                    section::Segment::Statistics(section::Data::Generated(section::segment::CommitStatistics {
                        count: 100,
                        duration: Some(32),
                        conventional_count: 20,
                        time_passed_since_last_release: Some(60),
                        unique_issues: vec![
                            section::segment::details::Category::Issue("1".into()),
                            section::segment::details::Category::Uncategorized,
                            section::segment::details::Category::Issue("42".into()),
                        ],
                    })),
                    section::Segment::Details(section::Data::Generated(section::segment::Details {
                        commits_by_category: {
                            let mut h = BTreeMap::default();
                            h.insert(
                                section::segment::details::Category::Uncategorized,
                                vec![
                                    section::segment::details::Message {
                                        title: "Just the title".into(),
                                        id: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                                    },
                                    section::segment::details::Message {
                                        title: "Title and body".into(),
                                        id: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5392"),
                                    },
                                ],
                            );
                            h.insert(
                                section::segment::details::Category::Issue("42".into()),
                                vec![
                                    section::segment::details::Message {
                                        title: "Just the title".into(),
                                        id: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5392"),
                                    },
                                    section::segment::details::Message {
                                        title: "Another title".into(),
                                        id: hex_to_id("e69de29bb2d1d6434b8b29ae775ad8c2e48c5391"),
                                    },
                                ],
                            );
                            h
                        },
                    })),
                ],
                unknown: String::new(),
            },
        ],
    };

    for link_mode in &[
        changelog::write::Linkables::AsText,
        changelog::write::Linkables::AsLinks {
            repository_url: gix::Url::try_from(b"https://github.com/user/repo".as_bstr())?.into(),
        },
    ] {
        // NOTE: we can't run this a second time as the statistical information will be gone (it was never parsed back)
        let mut md = String::new();
        log.write_to(&mut md, link_mode, changelog::write::Components::all(), false)?;
        insta::assert_snapshot!(md);

        let parsed_log = ChangeLog::from_markdown(&md);
        assert_eq!(parsed_log, log, "we must be able to parse the exact input back");
    }

    for components in &[
        changelog::write::Components::empty(),
        changelog::write::Components::all(),
        changelog::write::Components::DETAIL_TAGS,
    ] {
        for section in &log.sections {
            let mut buf = String::new();
            section.write_to(&mut buf, &changelog::write::Linkables::AsText, *components, false)?;
            insta::assert_snapshot!(buf);
        }
    }
    Ok(())
}
