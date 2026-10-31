//! Provides the parser for Flitter splits files.

use crate::{
    Lang, Run, Segment, Time,
    platform::prelude::*,
    timing::{CustomParser, parse_custom},
};
use alloc::borrow::Cow;
use core::{fmt, result::Result as StdResult};
use serde_derive::Deserialize;
use serde_json::Error as JsonError;

/// The Error type for splits files that couldn't be parsed by the Flitter
/// Parser.
#[derive(Debug)]
pub enum Error {
    /// Failed to parse JSON.
    Json {
        /// The underlying error.
        source: JsonError,
    },
    /// The split names can't be empty.
    SplitNamesEmpty,
    /// The personal best can't be empty.
    PersonalBestEmpty,
    /// The golds can't be empty.
    GoldsEmpty,
    /// The split name count does not match the gold count.
    GoldCountMismatch,
    /// The split name count does not match the personal best split count.
    PersonalBestCountMismatch,
    /// The last split of the personal best can't be null.
    LastSplitNull,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json { .. } => f.write_str("Failed to parse JSON."),
            Self::SplitNamesEmpty => f.write_str("The split names can't be empty."),
            Self::PersonalBestEmpty => f.write_str("The personal best can't be empty."),
            Self::GoldsEmpty => f.write_str("The golds can't be empty."),
            Self::GoldCountMismatch => {
                f.write_str("The split name count does not match the gold count.")
            }
            Self::PersonalBestCountMismatch => {
                f.write_str("The split name count does not match the personal best split count.")
            }
            Self::LastSplitNull => {
                f.write_str("The last split of the personal best can't be null.")
            }
        }
    }
}

impl core::error::Error for Error {
    #[cfg(feature = "std")]
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Json { source } => Some(source),
            _ => None,
        }
    }
}

/// The Result type for the Flitter Parser.
pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
struct Splits<'a> {
    #[serde(borrow)]
    title: Cow<'a, str>,
    #[serde(borrow)]
    category: Cow<'a, str>,
    attempts: u32,
    // completed: u32,
    #[serde(borrow)]
    split_names: Vec<Cow<'a, str>>,
    #[serde(borrow)]
    golds: Vec<Option<Gold<'a>>>,
    #[serde(borrow)]
    personal_best: PersonalBest<'a>,
}

#[derive(Deserialize)]
struct Gold<'a> {
    #[serde(borrow)]
    duration: Cow<'a, str>,
}

#[derive(Deserialize)]
struct PersonalBest<'a> {
    // attempt: u32,
    #[serde(borrow)]
    splits: Vec<Option<Split<'a>>>,
}

#[derive(Deserialize)]
struct Split<'a> {
    #[serde(borrow)]
    time: Cow<'a, str>,
}

struct FlitterParser;

impl CustomParser for FlitterParser {
    const ASCII_ONLY: bool = true;
    const ALLOW_NEGATIVE: bool = false;
    const WITH_DAYS: bool = true;
}

fn parse_time(real_time: &str) -> Option<Time> {
    let time = parse_custom::<FlitterParser>(real_time, Lang::English).ok()?;
    Some(Time::new().with_real_time(Some(time)))
}

/// Attempts to parse an Flitter splits file.
pub fn parse(source: &str) -> Result<Run> {
    let splits: Splits<'_> =
        serde_json::from_str(source).map_err(|source| Error::Json { source })?;

    if splits.split_names.is_empty() {
        return Err(Error::SplitNamesEmpty);
    }
    if splits.golds.is_empty() {
        return Err(Error::GoldsEmpty);
    }
    if splits.personal_best.splits.is_empty() {
        return Err(Error::PersonalBestEmpty);
    }
    if splits.golds.len() != splits.split_names.len() {
        return Err(Error::GoldCountMismatch);
    }
    if splits.personal_best.splits.len() != splits.split_names.len() {
        return Err(Error::PersonalBestCountMismatch);
    }
    if splits.personal_best.splits.last().unwrap().is_none() {
        return Err(Error::LastSplitNull);
    }

    let mut run = Run::new();

    run.set_game_name(splits.title);
    run.set_category_name(splits.category);
    run.set_attempt_count(splits.attempts);

    for name in splits.split_names {
        run.push_segment(Segment::new(name));
    }

    let segments = run.segments_mut();

    for (segment, pb) in segments.iter_mut().zip(splits.personal_best.splits) {
        catch! {
            segment.set_personal_best_split_time(parse_time(&pb?.time)?);
        };
    }

    for (segment, gold) in segments.iter_mut().zip(splits.golds) {
        catch! {
            segment.set_best_segment_time(parse_time(&gold?.duration)?);
        };
    }

    Ok(run)
}
