//! Provides the parser for ShitSplit splits files.

use crate::{timing, GameTime, Run, Segment, TimeSpan};
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::result::Result as StdResult;

quick_error! {
    /// The Error type for splits files that couldn't be parsed by the ShitSplit
    /// Parser.
    #[derive(Debug)]
    pub enum Error {
        /// An empty splits file was provided.
        Empty {}
        /// Expected the name of the category, but didn't find it.
        ExpectedCategoryName {}
        /// Expected the attempt count, but didn't find it.
        ExpectedAttemptCount {}
        /// Expected the name of the world, but didn't find it.
        ExpectedWorldName {}
        /// Expected the time of the world, but didn't find it.
        ExpectedWorldTime {}
        /// Failed to parse the amount of attempts.
        Attempt(err: ParseIntError) {
            from()
        }
        /// Failed to parse a time.
        Time(err: timing::ParseError) {
            from()
        }
        /// Failed to read from the source.
        Io(err: io::Error) {
            from()
        }
    }
}

/// The Result type for the ShitSplit Parser.
pub type Result<T> = StdResult<T, Error>;

/// Attempts to parse a ShitSplit splits file.
pub fn parse<R: BufRead>(source: R) -> Result<Run> {
    let mut lines = source.lines();

    let line = lines.next().ok_or(Error::Empty)??;

    let mut splits = line.split('|');
    let category_name = splits.next().ok_or(Error::ExpectedCategoryName)?;
    if !category_name.starts_with('#') {
        return Err(Error::ExpectedCategoryName);
    }

    let mut run = Run::new();

    run.set_category_name(&category_name[1..]);
    run.set_attempt_count(splits.next().ok_or(Error::ExpectedAttemptCount)?.parse()?);
    let mut total_time = TimeSpan::zero();
    let mut next_line = lines.next();
    while let Some(line) = next_line {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let mut splits = line.split('|');
        let world_name = splits.next().ok_or(Error::ExpectedWorldName)?;
        total_time += splits.next().ok_or(Error::ExpectedWorldTime)?.parse()?;
        next_line = lines.next();
        let mut has_acts = false;
        while let Some(line) = next_line {
            let line = line?;
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
