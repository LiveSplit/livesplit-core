//! Provides the parser for Flitter splits files.

use crate::{
    platform::prelude::*,
    timing::{parse_custom, CustomParser},
    Run, Segment, Time,
};
use alloc::borrow::Cow;
use core::result::Result as StdResult;
use serde_derive::Deserialize;
use serde_json::Error as JsonError;
use snafu::ensure;

/// The Error type for splits files that couldn't be parsed by the Flitter
/// Parser.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    /// Failed to parse JSON.
    Json {
        /// The underlying error.
        #[cfg_attr(not(feature = "std"), snafu(source(false)))]
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
    let time = parse_custom::<FlitterParser>(real_time).ok()?;
    Some(Time::new().with_real_time(Some(time)))
}

/// Attempts to parse an Flitter splits file.
pub fn parse(source: &str) -> Result<Run> {
    let splits: Splits<'_> =
        serde_json::from_str(source).map_err(|source| Error::Json { source })?;

    ensure!(!splits.split_names.is_empty(), SplitNamesEmpty);
    ensure!(!splits.golds.is_empty(), GoldsEmpty);
    ensure!(!splits.personal_best.splits.is_empty(), PersonalBestEmpty);
    ensure!(
        splits.golds.len() == splits.split_names.len(),
        GoldCountMismatch
    );
    ensure!(
        splits.personal_best.splits.len() == splits.split_names.len(),
        PersonalBestCountMismatch
    );
    ensure!(
        splits.personal_best.splits.last().unwrap().is_some(),
        LastSplitNull
    );

    let mut run = Run::new();

    run.set_game_name(splits.title);
    run.set_category_name(splits.category);
    run.set_attempt_count(splits.attempts);

    let segments = run.segments_mut();

    segments.extend(splits.split_names.into_iter().map(Segment::new));

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
