//! Provides the parser for ShitSplit splits files.

use crate::{GameTime, Lang, Run, Segment, TimeSpan, timing};
use core::{fmt, num::ParseIntError, result::Result as StdResult};

/// The Error type for splits files that couldn't be parsed by the ShitSplit
/// Parser.
#[derive(Debug)]
pub enum Error {
    /// An empty splits file was provided.
    Empty,
    /// Expected the name of the category, but didn't find it.
    ExpectedCategoryName,
    /// Expected the attempt count, but didn't find it.
    ExpectedAttemptCount,
    /// Failed to parse the attempt count.
    ParseAttemptCount {
        /// The underlying error.
        source: ParseIntError,
    },
    /// Expected the name of a world, but didn't find it.
    ExpectedWorldName,
    /// Expected the time of a world, but didn't find it.
    ExpectedWorldTime,
    /// Failed to parse the time of a world.
    ParseWorldTime {
        /// The underlying error.
        source: timing::ParseError,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("An empty splits file was provided."),
            Self::ExpectedCategoryName => {
                f.write_str("Expected the name of the category, but didn't find it.")
            }
            Self::ExpectedAttemptCount => {
                f.write_str("Expected the attempt count, but didn't find it.")
            }
            Self::ParseAttemptCount { .. } => f.write_str("Failed to parse the attempt count."),
            Self::ExpectedWorldName => {
                f.write_str("Expected the name of a world, but didn't find it.")
            }
            Self::ExpectedWorldTime => {
                f.write_str("Expected the time of a world, but didn't find it.")
            }
            Self::ParseWorldTime { .. } => f.write_str("Failed to parse the time of a world."),
        }
    }
}

impl core::error::Error for Error {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::ParseAttemptCount { source } => Some(source),
            Self::ParseWorldTime { source } => Some(source),
            _ => None,
        }
    }
}

/// The Result type for the ShitSplit Parser.
pub type Result<T> = StdResult<T, Error>;

/// Attempts to parse a ShitSplit splits file.
pub fn parse(source: &str) -> Result<Run> {
    let mut lines = source.lines();

    let line = lines.next().ok_or(Error::Empty)?;

    let mut splits = line.split('|');
    let category_name = splits.next().ok_or(Error::ExpectedCategoryName)?;

    let mut run = Run::new();

    run.set_category_name(
        category_name
            .strip_prefix('#')
            .ok_or(Error::ExpectedCategoryName)?,
    );
    run.set_attempt_count(
        splits
            .next()
            .ok_or(Error::ExpectedAttemptCount)?
            .parse()
            .map_err(|source| Error::ParseAttemptCount { source })?,
    );
    let mut total_time = TimeSpan::zero();
    let mut next_line = lines.next();
    while let Some(line) = next_line {
        if line.is_empty() {
            break;
        }
        let mut splits = line.split('|');
        let world_name = splits.next().ok_or(Error::ExpectedWorldName)?;
        total_time += TimeSpan::parse(
            splits.next().ok_or(Error::ExpectedWorldTime)?,
            Lang::English,
        )
        .map_err(|source| Error::ParseWorldTime { source })?;
        next_line = lines.next();
        let mut has_acts = false;
        while let Some(line) = next_line {
            if let Some(segment_name) = line.strip_prefix('*') {
                run.push_segment(Segment::new(segment_name));
                has_acts = true;
                next_line = lines.next();
            } else {
                next_line = Some(line);
                break;
            }
        }
        let time = GameTime(Some(total_time)).into();
        if has_acts {
            run.segments_mut()
                .last_mut()
                .unwrap()
                .set_personal_best_split_time(time);
        } else {
            let mut segment = Segment::new(world_name);
            segment.set_personal_best_split_time(time);
            run.push_segment(segment);
        }
    }

    Ok(run)
}
