//! Provides the parser for ShitSplit splits files.

use crate::{timing, GameTime, Run, Segment, TimeSpan};
use core::{num::ParseIntError, result::Result as StdResult};
use snafu::{OptionExt, ResultExt};

/// The Error type for splits files that couldn't be parsed by the ShitSplit
/// Parser.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
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

/// The Result type for the ShitSplit Parser.
pub type Result<T> = StdResult<T, Error>;

/// Attempts to parse a ShitSplit splits file.
pub fn parse(source: &str) -> Result<Run> {
    let mut lines = source.lines();

    let line = lines.next().context(Empty)?;

    let mut splits = line.split('|');
    let category_name = splits.next().context(ExpectedCategoryName)?;

    let mut run = Run::new();

    run.set_category_name(
        category_name
            .strip_prefix('#')
            .ok_or(Error::ExpectedCategoryName)?,
    );
    run.set_attempt_count(
        splits
            .next()
            .context(ExpectedAttemptCount)?
            .parse()
            .context(ParseAttemptCount)?,
    );
    let mut total_time = TimeSpan::zero();
    let mut next_line = lines.next();
    while let Some(line) = next_line {
        if line.is_empty() {
            break;
        }
        let mut splits = line.split('|');
        let world_name = splits.next().context(ExpectedWorldName)?;
        total_time += splits
            .next()
            .context(ExpectedWorldTime)?
            .parse()
            .context(ParseWorldTime)?;
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
