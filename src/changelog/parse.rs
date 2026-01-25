use std::{
    convert::TryFrom,
    iter::{FromIterator, Peekable},
    ops::Range,
    str::FromStr,
};

use gix::bstr::ByteSlice;
use pulldown_cmark::{CowStr, Event, HeadingLevel, OffsetIter, Tag};
use winnow::{
    ascii,
    combinator::{alt, delimited, opt, preceded, separated_pair, terminated},
    error::{FromExternalError, ParserError},
    prelude::*,
    token::{literal, take_till, take_while},
};

use crate::{
    changelog,
    changelog::{
        section,
        section::{
            segment::{conventional::as_headline, Conventional},
            Segment,
        },
        Section,
    },
    ChangeLog,
};

impl ChangeLog {
    /// Obtain as much information as possible from `input` and keep everything we didn't understand in respective sections.
    pub fn from_markdown(input: &str) -> ChangeLog {
        let mut sections = Vec::new();
        let mut section_body = String::new();
        let mut previous_headline = None::<Headline>;
        let mut first_heading_level = None;
        for line in input.as_bytes().as_bstr().lines_with_terminator() {
            let line = line.to_str().expect("valid UTF-8");
            match Headline::try_from(line) {
                Ok(headline) => {
                    first_heading_level.get_or_insert(headline.level);
                    match previous_headline {
                        Some(mut headline) => {
                            headline.level = first_heading_level.expect("set first");
                            sections.push(Section::from_headline_and_body(
                                headline,
                                std::mem::take(&mut section_body),
                            ));
                        }
                        None => {
                            if !section_body.is_empty() {
                                sections.push(Section::Verbatim {
                                    text: std::mem::take(&mut section_body),
                                    generated: false,
                                })
                            }
                        }
                    };
                    previous_headline = Some(headline);
                }
                Err(()) => {
                    section_body.push_str(line);
                }
            }
        }

        match previous_headline {
            Some(headline) => {
                sections.push(Section::from_headline_and_body(
                    headline,
                    std::mem::take(&mut section_body),
                ));
            }
            None => sections.push(Section::Verbatim {
                text: section_body,
                generated: false,
            }),
        }

        let insert_sorted_at_pos = sections.first().map_or(0, |s| match s {
            Section::Verbatim { .. } => 1,
            Section::Release { .. } => 0,
        });
        let mut non_release_sections = Vec::new();
        let mut release_sections = Vec::new();
        for section in sections {
            match section {
                Section::Verbatim { .. } => non_release_sections.push(section),
                Section::Release { .. } => release_sections.push(section),
            }
        }
        release_sections.sort_by(|lhs, rhs| match (lhs, rhs) {
            (
                Section::Release {
                    name: lhs_name,
                    date: lhs_date,
                    ..
                },
                Section::Release {
                    name: rhs_name,
                    date: rhs_date,
                    ..
                },
            ) => {
                match (lhs_name, rhs_name) {
                    // Unreleased sections always come first
                    (changelog::Version::Unreleased, changelog::Version::Unreleased) => std::cmp::Ordering::Equal,
                    (changelog::Version::Unreleased, _) => std::cmp::Ordering::Less,
                    (_, changelog::Version::Unreleased) => std::cmp::Ordering::Greater,
                    // For released versions, sort by date (newest first)
                    (changelog::Version::Semantic(_), changelog::Version::Semantic(_)) => {
                        match (lhs_date, rhs_date) {
                            // Both have dates: sort by date descending
                            (Some(lhs_d), Some(rhs_d)) => rhs_d.cmp(lhs_d),
                            // If one has no date, put it after those with dates
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            // Both have no date: fall back to version comparison (descending)
                            (None, None) => lhs_name.cmp(rhs_name).reverse(),
                        }
                    }
                }
            }
            _ => unreachable!("BUG: there are only release sections here"),
        });
        let mut sections = Vec::from_iter(non_release_sections.drain(..insert_sorted_at_pos));
        sections.append(&mut release_sections);
        sections.append(&mut non_release_sections);
        ChangeLog { sections }
    }
}

impl Section {
    fn from_headline_and_body(
        Headline {
            level,
            version_prefix,
            version,
            date,
        }: Headline,
        body: String,
    ) -> Self {
        let mut events = pulldown_cmark::Parser::new_ext(&body, pulldown_cmark::Options::all())
            .into_offset_iter()
            .peekable();
        let mut unknown = String::new();
        let mut segments = Vec::new();

        let mut unknown_range = None;
        let mut removed_messages = Vec::new();
        while let Some((e, range)) = events.next() {
            match e {
                Event::Html(text) | Event::InlineHtml(text) if text.starts_with(Section::UNKNOWN_TAG_START) => {
                    record_unknown_range(&mut segments, unknown_range.take(), &body);
                    for (event, _range) in events.by_ref().take_while(
                        |(e, _range)| !matches!(e, Event::Html(text) | Event::InlineHtml(text) if text.starts_with(Section::UNKNOWN_TAG_END)),
                    ) {
                        track_unknown_event(event, &mut unknown);
                    }
                }
                Event::Html(text) | Event::InlineHtml(text)
                    if text.starts_with(section::segment::Conventional::REMOVED_HTML_PREFIX) =>
                {
                    if let Some(id) = parse_message_id(text.as_ref()) {
                        if !removed_messages.contains(&id) {
                            removed_messages.push(id);
                        }
                    }
                }
                // Ignore HtmlBlock start/end tags - the actual HTML content is in Event::Html or Event::InlineHtml
                Event::Start(Tag::HtmlBlock) | Event::End(pulldown_cmark::TagEnd::HtmlBlock) => {}
                Event::Start(Tag::Heading { level: indent, .. }) => {
                    record_unknown_range(&mut segments, unknown_range.take(), &body);
                    enum State {
                        ParseConventional { title: String },
                        SkipGenerated,
                        ConsiderUserAuthored,
                    }
                    let state = match events.next() {
                        Some((Event::Text(title), _range))
                            if title.starts_with(section::segment::ThanksClippy::TITLE) =>
                        {
                            segments.push(Segment::Clippy(section::Data::Parsed));
                            State::SkipGenerated
                        }
                        Some((Event::Text(title), _range))
                            if title.starts_with(section::segment::CommitStatistics::TITLE) =>
                        {
                            segments.push(Segment::Statistics(section::Data::Parsed));
                            State::SkipGenerated
                        }
                        Some((Event::Text(title), _range)) if title.starts_with(section::segment::Details::TITLE) => {
                            segments.push(Segment::Details(section::Data::Parsed));
                            State::SkipGenerated
                        }
                        Some((Event::Text(title), _range))
                            if title.starts_with(as_headline("feat").expect("valid"))
                                || title.starts_with(as_headline("add").expect("valid"))
                                || title.starts_with(as_headline("revert").expect("valid"))
                                || title.starts_with(as_headline("remove").expect("valid"))
                                || title.starts_with(as_headline("change").expect("valid"))
                                || title.starts_with(as_headline("docs").expect("valid"))
                                || title.starts_with(as_headline("perf").expect("valid"))
                                || title.starts_with("refactor")
                                || title.starts_with("other")
                                || title.starts_with("style")
                                || title.starts_with(as_headline("fix").expect("valid")) =>
                        {
                            State::ParseConventional {
                                title: title.into_string(),
                            }
                        }
                        Some((_event, next_range)) => {
                            update_unknown_range(&mut unknown_range, range);
                            update_unknown_range(&mut unknown_range, next_range);
                            State::ConsiderUserAuthored
                        }
                        None => State::ConsiderUserAuthored,
                    };

                    events
                        .by_ref()
                        .take_while(|(e, range)| {
                            if matches!(state, State::ConsiderUserAuthored) {
                                update_unknown_range(&mut unknown_range, range.clone());
                            }
                            !matches!(e, Event::End(pulldown_cmark::TagEnd::Heading(_)))
                        })
                        .count();
                    match state {
                        State::ParseConventional { title } => {
                            segments.push(parse_conventional_to_next_section_title(
                                &body,
                                title,
                                &mut events,
                                indent,
                                &mut unknown,
                            ));
                        }
                        State::SkipGenerated => {
                            skip_to_next_section_title(&mut events, indent);
                        }
                        State::ConsiderUserAuthored => {}
                    }
                }
                _unknown_event => update_unknown_range(&mut unknown_range, range),
            };
        }
        record_unknown_range(&mut segments, unknown_range.take(), &body);
        Section::Release {
            name: match version {
                Some(version) => changelog::Version::Semantic(version),
                None => changelog::Version::Unreleased,
            },
            version_prefix,
            date,
            removed_messages,
            heading_level: level,
            segments,
            unknown,
        }
    }
}

fn parse_conventional_to_next_section_title(
    markdown: &str,
    title: String,
    events: &mut Peekable<OffsetIter<'_>>,
    level: HeadingLevel,
    unknown: &mut String,
) -> Segment {
    let is_breaking = title.ends_with(section::segment::Conventional::BREAKING_TITLE_ENCLOSED);
    let kind = [
        "fix", "add", "feat", "revert", "remove", "change", "docs", "perf", "refactor", "other", "style",
    ]
    .iter()
    .find(|kind| {
        let headline = section::segment::conventional::as_headline(kind).unwrap_or(*kind);
        let common_len = headline.len().min(title.len());
        title
            .get(..common_len)
            .and_then(|t| headline.get(..common_len).map(|h| t.eq_ignore_ascii_case(h)))
            .unwrap_or(false)
    })
    .expect("BUG: this list needs an update too if new kinds of conventional messages are added");

    let mut conventional = section::segment::Conventional {
        kind,
        is_breaking,
        removed: vec![],
        messages: vec![],
    };
    while let Some((event, _range)) = events.peek() {
        match event {
            Event::Start(Tag::Heading { level: indent, .. }) if *indent == level => break,
            _ => {
                let (event, _range) = events.next().expect("peeked before so event is present");
                match event {
                    Event::Html(ref tag) | Event::InlineHtml(ref tag) => match parse_message_id(tag.as_ref()) {
                        Some(id) => {
                            if !conventional.removed.contains(&id) {
                                conventional.removed.push(id)
                            }
                        }
                        None => track_unknown_event(event, unknown),
                    },
                    Event::Start(Tag::List(_)) => {
                        while let Some((event, item_range)) = events.next() {
                            match event {
                                Event::Start(Tag::Item) => {
                                    if let Some((possibly_html, _)) = events.next() {
                                        match possibly_html {
                                            Event::Start(Tag::Paragraph) => {
                                                if let Some((possibly_html, _)) = events.next() {
                                                    match possibly_html {
                                                        Event::Html(tag) | Event::InlineHtml(tag) => {
                                                            parse_id_fallback_to_user_message(
                                                                markdown,
                                                                events,
                                                                &mut conventional,
                                                                item_range,
                                                                tag,
                                                            );
                                                        }
                                                        _other_event => make_user_message_and_consume_item(
                                                            markdown,
                                                            events,
                                                            &mut conventional,
                                                            item_range,
                                                        ),
                                                    }
                                                }
                                            }
                                            Event::Html(tag) | Event::InlineHtml(tag) => {
                                                parse_id_fallback_to_user_message(
                                                    markdown,
                                                    events,
                                                    &mut conventional,
                                                    item_range,
                                                    tag,
                                                );
                                            }
                                            _other_event => make_user_message_and_consume_item(
                                                markdown,
                                                events,
                                                &mut conventional,
                                                item_range,
                                            ),
                                        }
                                    }
                                }
                                Event::End(pulldown_cmark::TagEnd::List(_)) => break,
                                event => track_unknown_event(event, unknown),
                            }
                        }
                    }
                    event => track_unknown_event(event, unknown),
                }
                continue;
            }
        }
    }
    section::Segment::Conventional(conventional)
}

fn parse_id_fallback_to_user_message(
    markdown: &str,
    events: &mut Peekable<OffsetIter<'_>>,
    conventional: &mut Conventional,
    item_range: Range<usize>,
    tag: CowStr<'_>,
) {
    match parse_message_id(tag.as_ref()) {
        Some(id) => {
            let mut ranges = Vec::new();
            consume_item_events(events, |range| ranges.push(range));
            let start = ranges.first();
            let end = ranges.last().or(start);
            if let Some(title_and_body) = start
                .map(|r| r.start)
                .and_then(|start| end.map(|r| markdown[start..r.end].trim()))
            {
                let mut lines = title_and_body
                    .as_bytes()
                    .as_bstr()
                    .lines_with_terminator()
                    .map(|b| b.to_str().expect("always valid as source is UTF-8"));
                conventional
                    .messages
                    .push(section::segment::conventional::Message::Generated {
                        id,
                        title: lines.next().map_or("", |l| l.trim()).to_owned(),
                        body: lines
                            .map(|l| {
                                // Strip exactly 3 leading spaces (the indentation we add during writing),
                                // preserving any additional indentation for nested lists.
                                // This fixes issue #30 where nested list items were being flattened.
                                const INDENT_TO_STRIP: usize = 3;
                                let leading_spaces = l.chars().take_while(|c| *c == ' ').count();
                                let chars_to_strip = leading_spaces.min(INDENT_TO_STRIP);
                                &l[chars_to_strip..]
                            })
                            .fold(None::<String>, |mut acc, l| {
                                acc.get_or_insert_with(String::new).push_str(l);
                                acc
                            }),
                    });
            }
        }
        None => make_user_message_and_consume_item(markdown, events, conventional, item_range),
    };
}

fn make_user_message_and_consume_item(
    markdown: &str,
    events: &mut Peekable<OffsetIter<'_>>,
    conventional: &mut Conventional,
    range: Range<usize>,
) {
    conventional
        .messages
        .push(section::segment::conventional::Message::User {
            markdown: markdown[range].trim_end().to_owned(),
        });
    consume_item_events(events, |_| {});
}

/// Consume events until the end of the current list item, properly handling nested items.
/// We start at depth 1 because we're already inside one Item.
/// The callback is invoked with the range of each event consumed.
fn consume_item_events(events: &mut Peekable<OffsetIter<'_>>, mut on_event: impl FnMut(Range<usize>)) {
    let mut item_depth: usize = 1;
    for (event, range) in events.by_ref() {
        match &event {
            Event::Start(Tag::Item) => item_depth += 1,
            Event::End(pulldown_cmark::TagEnd::Item) => {
                item_depth -= 1;
                if item_depth == 0 {
                    break;
                }
            }
            _ => {}
        }
        on_event(range);
    }
}

fn parse_message_id(html: &str) -> Option<gix::hash::ObjectId> {
    let html = html.strip_prefix(section::segment::Conventional::REMOVED_HTML_PREFIX)?;
    let end_of_hex = html.find(|c| {
        !matches!(c,
            'a'..='f' | '0'..='9'
        )
    })?;
    gix::hash::ObjectId::from_hex(&html.as_bytes()[..end_of_hex]).ok()
}

fn update_unknown_range(target: &mut Option<Range<usize>>, source: Range<usize>) {
    match target {
        Some(range_thus_far) => {
            if source.end > range_thus_far.end {
                range_thus_far.end = source.end;
            }
        }
        None => *target = source.into(),
    }
}

fn record_unknown_range(out: &mut Vec<section::Segment>, range: Option<Range<usize>>, markdown: &str) {
    if let Some(range) = range {
        out.push(Segment::User {
            markdown: markdown[range].to_owned(),
        })
    }
}

fn track_unknown_event(unknown_event: Event<'_>, unknown: &mut String) {
    log::trace!("Cannot handle {unknown_event:?}");
    match unknown_event {
        Event::Html(text)
        | Event::InlineHtml(text)
        | Event::Code(text)
        | Event::Text(text)
        | Event::FootnoteReference(text)
        | Event::Start(Tag::FootnoteDefinition(text) | Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(text))) => {
            unknown.push_str(text.as_ref())
        }
        Event::Start(Tag::Link { dest_url, .. } | Tag::Image { dest_url, .. }) => unknown.push_str(dest_url.as_ref()),
        _ => {}
    }
}

fn skip_to_next_section_title(events: &mut Peekable<OffsetIter<'_>>, level: HeadingLevel) {
    while let Some((event, _range)) = events.peek() {
        match event {
            Event::Start(Tag::Heading { level: indent, .. }) if *indent == level => break,
            _ => {
                events.next();
                continue;
            }
        }
    }
}

struct Headline {
    level: usize,
    version_prefix: String,
    version: Option<semver::Version>,
    date: Option<jiff::Zoned>,
}

impl<'a> TryFrom<&'a str> for Headline {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        headline::<()>.parse(value).map_err(|err| err.into_inner())
    }
}

fn headline<'a, E: ParserError<&'a str> + FromExternalError<&'a str, ()>>(i: &mut &'a str) -> ModalResult<Headline, E> {
    let hashes = take_while(0.., |c: char| c == '#');
    let greedy_whitespace = |i: &mut &'a str| take_while(0.., char::is_whitespace).parse_next(i);
    let take_n_digits =
        |n: usize| take_while(n, |c: char| c.is_ascii_digit()).try_map(|num| u32::from_str(num).map_err(|_| ()));

    terminated(
        (
            separated_pair(
                hashes,
                greedy_whitespace,
                alt((
                    (
                        opt("v"),
                        take_till(0.., char::is_whitespace)
                            .try_map(|v| semver::Version::parse(v).map_err(|_| ()).map(Some)),
                    ),
                    literal(ascii::Caseless("unreleased")).map(|_| (None, None)),
                )),
            ),
            opt(preceded(
                greedy_whitespace,
                delimited(
                    "(",
                    (take_n_digits(4), "-", take_n_digits(2), "-", take_n_digits(2)).try_map(
                        |(year, _, month, _, day)| {
                            jiff::civil::Date::new(year as i16, month as i8, day as i8)
                                .map_err(|_| ())
                                .and_then(|d| d.to_zoned(jiff::tz::TimeZone::UTC).map_err(|_| ()))
                        },
                    ),
                    ")",
                ),
            )),
        ),
        greedy_whitespace,
    )
    .map(|((hashes, (prefix, version)), date)| Headline {
        level: hashes.len(),
        version_prefix: prefix.map_or_else(String::new, ToOwned::to_owned),
        version,
        date,
    })
    .parse_next(i)
}
