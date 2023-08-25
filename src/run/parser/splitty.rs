//! Provides the parser for Splitty splits files.

use crate::{platform::prelude::*, Run, Segment, Time, TimeSpan, TimingMethod};
use alloc::borrow::Cow;
use core::result::Result as StdResult;
use serde_derive::Deserialize;
use serde_json::Error as JsonError;

/// The Error type for splits files that couldn't be parsed by the Splitty
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
}

/// The Result type for the Splitty Parser.
pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
struct Splits<'a> {
    #[serde(borrow)]
    run_name: Cow<'a, str>,
    start_delay: f64,
    run_count: u32,
    splits: Vec<Split<'a>>,
    timer_type: u8,
}

#[derive(Deserialize)]
struct Split<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    pb_split: Option<f64>,
    split_best: Option<f64>,
}

fn parse_time(milliseconds: Option<f64>, method: TimingMethod) -> Time {
    let mut time = Time::new();

    if let Some(milliseconds) = milliseconds {
        time[method] = Some(TimeSpan::from_milliseconds(milliseconds));
    }

    time
}

/// Attempts to parse a Splitty splits file.
pub fn parse(source: &str) -> Result<Run> {
    let splits: Splits<'_> =
        serde_json::from_str(source).map_err(|source| Error::Json { source })?;

    let mut run = Run::new();

    run.set_game_name(splits.run_name);
    run.set_attempt_count(splits.run_count);
    run.set_offset(TimeSpan::from_milliseconds(-splits.start_delay));

    let method = if splits.timer_type == 0 {
        TimingMethod::RealTime
    } else {
        TimingMethod::GameTime
    };

    run.segments_mut()
        .extend(splits.splits.into_iter().map(|split| {
            let mut segment = Segment::new(split.name);
            segment.set_personal_best_split_time(parse_time(split.pb_split, method));
            segment.set_best_segment_time(parse_time(split.split_best, method));

            segment
        }));

    Ok(run)
}
