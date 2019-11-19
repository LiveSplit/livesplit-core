//! Provides the parser for ShitSplit splits files.

use crate::{timing, GameTime, Run, Segment, TimeSpan};
use core::num::ParseIntError;
use core::result::Result as StdResult;
use snafu::{OptionExt, ResultExt};
use std::io::{self, BufRead};

/// The Error type for splits files that couldn't be parsed by the ShitSplit
/// Parser.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// An empty splits file was provided.
    Empty,
    /// Failed to read the title line.
    ReadTitleLine {
        /// The underlying error.
        source: io::Error,
    },
    /// Expected the name of the category, but didn't find it.
    ExpectedCategoryName,
    /// Expected the attempt count, but didn't find it.
    ExpectedAttemptCount,
    /// Failed to parse the attempt count.
    ParseAttemptCount {
        /// The underlying error.
        source: ParseIntError,
    },
    /// Failed to read the next world line.
    ReadWorldLine {
        /// The underlying error.
        source: io::Error,
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
    /// Failed to read the next segment line.
    ReadSegmentLine {
        /// The underlying error.
        source: io::Error,
    },
}

/// The Result type for the ShitSplit Parser.
pub type Result<T> = StdResult<T, Error>;

/// Attempts to parse a ShitSplit splits file.
pub fn parse<R: BufRead>(source: R) -> Result<Run> {
    let mut lines = source.lines();

    let line = lines.next().context(Empty)?.context(ReadTitleLine)?;

    let mut splits = line.split('|');
    let category_name = splits.next().context(ExpectedCategoryName)?;
    if !category_name.starts_with('#') {
        return Err(Error::ExpectedCategoryName);
    }

    let mut run = Run::new();

    run.set_category_name(&category_name[1..]);
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
        let line = line.context(ReadWorldLine)?;
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
            let line = line.context(ReadSegmentLine)?;
            if line.starts_with('*') {
                run.push_segment(Segment::new(&line[1..]));
                has_acts = true;
                next_line = lines.next();
            } else {
                next_line = Some(Ok(line));
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
