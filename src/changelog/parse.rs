use std::{convert::TryFrom, str::FromStr};

use git_repository::bstr::ByteSlice;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till, take_while, take_while_m_n},
    combinator::{all_consuming, map, map_res, opt},
    error::{FromExternalError, ParseError},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

use crate::{changelog, changelog::Section, ChangeLog};
use pulldown_cmark::{Event, Parser, Tag};

impl ChangeLog {
    /// Obtain as much information as possible from `input` and keep everything we didn't understand in respective sections.
    pub fn from_markdown(input: &str) -> ChangeLog {
        let mut sections = Vec::new();
        let mut plain_text = String::new();
        let mut previous_headline = None;
        for line in input.as_bytes().as_bstr().lines_with_terminator() {
            let line = line.to_str().expect("valid UTF-8");
            match Headline::try_from(line) {
                Ok(headline) => {
                    match previous_headline {
                        Some(headline) => {
                            sections.push(Section::from_headline_and_body(
                                headline,
                                std::mem::take(&mut plain_text),
                            ));
                        }
                        None => sections.push(Section::Verbatim {
                            text: std::mem::take(&mut plain_text),
                            generated: false,
                        }),
                    };
                    previous_headline = Some(headline);
                }
                Err(()) => {
                    plain_text.push_str(line);
                }
            }
        }

        match previous_headline {
            Some(headline) => {
                sections.push(Section::from_headline_and_body(
                    headline,
                    std::mem::take(&mut plain_text),
                ));
            }
            None => sections.push(Section::Verbatim {
                text: plain_text,
                generated: false,
            }),
        }
        ChangeLog { sections }
    }
}

impl Section {
    fn from_headline_and_body(Headline { level, version, date }: Headline, body: String) -> Self {
        let mut events = pulldown_cmark::Parser::new(&body);
        let mut unknown = String::new();
        let mut thanks_clippy_count = 0;

        while let Some(e) = events.next() {
            match e {
                Event::Html(text) if text.starts_with(Section::UNKNOWN_TAG_START) => {
                    for text in events
                        .by_ref()
                        .take_while(|e| !matches!(e, Event::Html(text) if text.starts_with(Section::UNKNOWN_TAG_END)))
                        .filter_map(|e| match e {
                            Event::Html(text) => Some(text),
                            _ => None,
                        })
                    {
                        unknown.push_str(text.as_ref());
                    }
                }
                Event::Start(Tag::Heading(_indent)) => {
                    enum State {
                        ParseClippy,
                        DoNothing,
                    }
                    let state = match events.next() {
                        Some(Event::Text(title)) if title.starts_with(Section::THANKS_CLIPPY_TITLE) => {
                            State::ParseClippy
                        }
                        _ => State::DoNothing,
                    };
                    events
                        .by_ref()
                        .take_while(|e| !matches!(e, Event::End(Tag::Heading(_))))
                        .count();
                    match state {
                        State::ParseClippy => {
                            if let Some(p) = collect_paragraph(events.by_ref(), &mut unknown) {
                                thanks_clippy_count = p
                                    .split(' ')
                                    .filter_map(|num| num.parse::<usize>().ok())
                                    .next()
                                    .unwrap_or(0)
                            }
                        }
                        State::DoNothing => {}
                    }
                }
                unknown_event => track_unknown_event(unknown_event, &mut unknown),
            };
        }
        Section::Release {
            name: match version {
                Some(version) => changelog::Version::Semantic(version),
                None => changelog::Version::Unreleased,
            },
            date,
            heading_level: level,
            thanks_clippy_count,
            unknown,
        }
    }
}

fn track_unknown_event(unknown_event: Event<'_>, unknown: &mut String) {
    log::trace!("Cannot handle {:?}", unknown_event);
    match unknown_event {
        Event::Html(text)
        | Event::Code(text)
        | Event::Text(text)
        | Event::FootnoteReference(text)
        | Event::Start(Tag::FootnoteDefinition(text))
        | Event::Start(Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(text)))
        | Event::Start(Tag::Link(_, text, _))
        | Event::Start(Tag::Image(_, text, _)) => unknown.push_str(text.as_ref()),
        _ => {}
    }
}

fn collect_paragraph(events: &mut Parser, unknown: &mut String) -> Option<String> {
    match events.next() {
        Some(Event::Start(Tag::Paragraph)) => {
            return events
                .take_while(|e| !matches!(e, Event::End(Tag::Paragraph)))
                .filter_map(|e| match e {
                    Event::Text(text) => Some(text),
                    _ => None,
                })
                .fold(String::new(), |mut acc, text| {
                    acc.push_str(&text);
                    acc
                })
                .into()
        }
        Some(event) => track_unknown_event(event, unknown),
        None => {}
    };
    None
}

struct Headline {
    level: usize,
    version: Option<semver::Version>,
    date: Option<time::OffsetDateTime>,
}

impl<'a> TryFrom<&'a str> for Headline {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        all_consuming(headline::<()>)(value).finish().map(|(_, h)| h)
    }
}

fn headline<'a, E: ParseError<&'a str> + FromExternalError<&'a str, ()>>(i: &'a str) -> IResult<&'a str, Headline, E> {
    let hashes = take_while(|c: char| c == '#');
    let greedy_whitespace = |i| take_while(|c: char| c.is_whitespace())(i);
    let take_n_digits = |n: usize| {
        map_res(take_while_m_n(n, n, |c: char| c.is_digit(10)), |num| {
            u32::from_str(num).map_err(|_| ())
        })
    };
    map(
        terminated(
            tuple((
                separated_pair(
                    hashes,
                    greedy_whitespace,
                    alt((
                        preceded(
                            tag("v"),
                            map_res(take_till(|c: char| c.is_whitespace()), |v| {
                                semver::Version::parse(v).map_err(|_| ()).map(Some)
                            }),
                        ),
                        map(tag_no_case("unreleased"), |_| None),
                    )),
                ),
                opt(preceded(
                    greedy_whitespace,
                    delimited(
                        tag("("),
                        map_res(
                            tuple((take_n_digits(4), tag("-"), take_n_digits(2), tag("-"), take_n_digits(2))),
                            |(year, _, month, _, day)| {
                                time::Month::try_from(month as u8).map_err(|_| ()).and_then(|month| {
                                    time::Date::from_calendar_date(year as i32, month, day as u8)
                                        .map_err(|_| ())
                                        .map(|d| d.midnight().assume_utc())
                                })
                            },
                        ),
                        tag(")"),
                    ),
                )),
            )),
            greedy_whitespace,
        ),
        |((hashes, version), date)| Headline {
            level: hashes.len(),
            version,
            date,
        },
    )(i)
}
