//! Parses the community-maintained auto splitter list and looks up games.
//!
//! [`List`] borrows XML text and parses matching entries on demand. With the
//! `networking` feature, `Downloader` fetches the source text and referenced files
//! asynchronously into memory. The caller owns the downloaded text and can
//! borrow it with [`List::new`]. Caching and persistence remain with the caller.

use crate::util::xml::{
    Event, Reader,
    helper::{Error as XmlError, end_tag, parse_base, parse_children, text},
};
use alloc::borrow::Cow;
use snafu::Snafu;

#[cfg(feature = "networking")]
mod download;
#[cfg(feature = "networking")]
pub use download::Downloader;

/// The URL of the community-maintained auto splitter list.
pub const DEFAULT_LIST_URL: &str = "https://raw.githubusercontent.com/LiveSplit/LiveSplit.AutoSplitters/master/LiveSplit.AutoSplitters.xml";

/// An error while parsing the auto splitter list.
#[derive(Debug, Snafu)]
pub enum Error {
    /// Failed to parse the XML list.
    Xml {
        /// The underlying XML parsing error.
        source: XmlError,
    },
    /// An auto splitter is missing a required element.
    #[snafu(display("The auto splitter list is missing the {element} element."))]
    MissingElement {
        /// The name of the missing element.
        element: &'static str,
    },
}

impl From<XmlError> for Error {
    fn from(source: XmlError) -> Self {
        Self::Xml { source }
    }
}

/// A borrowed view of the community-maintained auto splitter list.
///
/// Entries are parsed on demand when looking up a game. The list does not own
/// the XML or retain parsed entries. It also includes entries for runtimes other
/// than the Auto Splitting Runtime.
#[derive(Clone, Copy, Debug)]
pub struct List<'a> {
    source: &'a str,
}

/// The auto splitter found for a game. Text borrows from the list's XML wherever
/// possible and is allocated only when XML entities need to be decoded.
#[derive(Debug, PartialEq)]
pub struct AutoSplitter<'a> {
    urls: Vec<Cow<'a, str>>,
    uses_auto_splitting_runtime: bool,
    /// The description, preferring the runtime-specific override when present.
    pub description: Cow<'a, str>,
    /// The website, preferring the runtime-specific override when present.
    pub website: Option<Cow<'a, str>>,
}

struct AutoSplittingRuntime<'a> {
    url: Cow<'a, str>,
    description: Option<Cow<'a, str>>,
    website: Option<Cow<'a, str>>,
}

impl Default for List<'_> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a> List<'a> {
    /// Creates an empty auto splitter list.
    pub const fn empty() -> Self {
        Self::new("<AutoSplitters/>")
    }

    /// Creates a view of the XML without parsing or allocating. Malformed XML
    /// and missing required elements are reported when a query encounters them.
    pub const fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Returns the borrowed XML source.
    pub const fn source(&self) -> &'a str {
        self.source
    }

    /// Finds the first auto splitter for an exact, case-sensitive game name.
    ///
    /// Each call scans the XML from the beginning, reading game names and
    /// skipping unrelated metadata. Only the matching entry is parsed in full;
    /// entries after it are not visited. Unknown elements are ignored. Errors
    /// in the scanned XML or required fields of the matching entry are returned.
    /// Query once and retain the result when accessing several of its fields.
    pub fn get_for_game(&self, game_name: &str) -> Result<Option<AutoSplitter<'a>>, Error> {
        let mut reader = Reader::new(self.source);
        parse_base(&mut reader, "AutoSplitters", |_, _| Ok::<_, Error>(()))?;

        loop {
            match reader.read_event().ok_or(XmlError::Xml)? {
                Event::Start(tag) => {
                    if tag.name_and_attributes().0.name() == "AutoSplitter" {
                        // Revisit just this entry if its game names match, so
                        // metadata can appear before or after <Games>.
                        let mut entry = reader.clone();

                        if matches_game(&mut reader, game_name)? {
                            return parse_auto_splitter(&mut entry).map(Some);
                        }
                    } else {
                        end_tag::<Error>(&mut reader)?;
                    }
                }
                Event::End(_) => return Ok(None),
                Event::Ended => return Err(XmlError::UnexpectedEndOfFile.into()),
                _ => {}
            }
        }
    }

    /// Looks up a game's website, preferring the runtime-specific override.
    /// This scans the list on each call, like [`Self::get_for_game`].
    pub fn get_website_for_game(&self, game_name: &str) -> Result<Option<Cow<'a, str>>, Error> {
        Ok(self
            .get_for_game(game_name)?
            .and_then(|splitter| splitter.website))
    }

    /// Looks up a game's description, preferring the runtime-specific override.
    /// This scans the list on each call, like [`Self::get_for_game`].
    pub fn get_description_for_game(&self, game_name: &str) -> Result<Option<Cow<'a, str>>, Error> {
        Ok(self
            .get_for_game(game_name)?
            .map(|splitter| splitter.description))
    }
}

impl<'a> AutoSplitter<'a> {
    /// Returns the download URLs, preferring the nested runtime module when
    /// present. Otherwise these include all regular URLs and auxiliary files.
    pub fn urls(&self) -> &[Cow<'a, str>] {
        &self.urls
    }

    /// Returns whether the entry supports the Auto Splitting Runtime, either
    /// through its script type or a nested runtime-specific entry.
    pub const fn is_using_auto_splitting_runtime(&self) -> bool {
        self.uses_auto_splitting_runtime
    }
}

fn required<T>(value: Option<T>, element: &'static str) -> Result<T, Error> {
    value.ok_or(Error::MissingElement { element })
}

fn matches_game(reader: &mut Reader<'_>, game_name: &str) -> Result<bool, Error> {
    let mut found = None;

    parse_children(reader, |reader, tag, _| {
        if tag.name() != "Games" {
            return end_tag::<Error>(reader);
        }

        let matches = found.get_or_insert(false);

        parse_children(reader, |reader, tag, _| {
            if tag.name() == "Game" {
                text(reader, |name| *matches |= name == game_name)
            } else {
                end_tag::<Error>(reader)
            }
        })
    })?;

    required(found, "Games")
}

fn parse_urls<'a>(reader: &mut Reader<'a>) -> Result<Vec<Cow<'a, str>>, Error> {
    let mut urls = Vec::new();

    parse_children(reader, |reader, tag, _| {
        if tag.name() == "URL" {
            text(reader, |value| urls.push(value))
        } else {
            end_tag::<Error>(reader)
        }
    })?;

    Ok(urls)
}

fn parse_auto_splitter<'a>(reader: &mut Reader<'a>) -> Result<AutoSplitter<'a>, Error> {
    let (mut urls, mut description, mut website, mut runtime) = (None, None, None, None);
    let mut uses_auto_splitting_runtime = false;

    parse_children(reader, |reader, tag, _| match tag.name() {
        "URLs" => {
            urls = Some(parse_urls(reader)?);
            Ok(())
        }
        "ScriptType" => text(reader, |value| {
            uses_auto_splitting_runtime = value == "AutoSplittingRuntime"
        }),
        "Description" => text(reader, |value| description = Some(value)),
        "Website" => text(reader, |value| website = Some(value)),
        "AutoSplittingRuntime" => {
            runtime = Some(parse_runtime(reader)?);
            Ok(())
        }
        _ => end_tag::<Error>(reader),
    })?;

    let mut splitter = AutoSplitter {
        urls: required(urls, "URLs")?,
        uses_auto_splitting_runtime,
        description: required(description, "Description")?,
        website,
    };

    if let Some(runtime) = runtime {
        splitter.urls.clear();
        splitter.urls.push(runtime.url);
        splitter.uses_auto_splitting_runtime = true;

        if let Some(description) = runtime.description {
            splitter.description = description;
        }

        if let Some(website) = runtime.website {
            splitter.website = Some(website);
        }
    }

    Ok(splitter)
}

fn parse_runtime<'a>(reader: &mut Reader<'a>) -> Result<AutoSplittingRuntime<'a>, Error> {
    let (mut url, mut description, mut website) = (None, None, None);

    parse_children(reader, |reader, tag, _| match tag.name() {
        "URL" => text(reader, |value| url = Some(value)),
        "Description" => text(reader, |value| description = Some(value)),
        "Website" => text(reader, |value| website = Some(value)),
        _ => end_tag::<Error>(reader),
    })?;

    Ok(AutoSplittingRuntime {
        url: required(url, "URL")?,
        description,
        website,
    })
}

#[cfg(test)]
mod tests;
