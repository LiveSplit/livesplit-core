//! Provides the parser for worstrun splits files.

use crate::{Run, Segment, Time, TimeSpan};
use core::result::Result as StdResult;
use serde::Deserialize;
use serde_json::de::from_reader;
use serde_json::Error as JsonError;
use snafu::ResultExt;
use std::io::Read;

/// The Error type for splits files that couldn't be parsed by the worstrun
/// Parser.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Failed to parse JSON.
    Json {
        /// The underlying error.
        source: JsonError,
    },
}

/// The Result type for the worstrun Parser.
pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
struct Splits {
    game: Option<String>,
    category: Option<String>,
    record_time: Option<i32>,
    // best_time: Option<i32>, Completely unused, even by worstrun
    initial_delay: Option<i32>,
    splits: Option<Vec<Split>>,
}

#[derive(Deserialize)]
struct Split {
    title: Option<String>,
    last_split: Option<i32>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Poke {
    game: String,
    category: String,
}

pub(super) fn poke<R: Read>(source: R) -> bool {
    from_reader::<_, Poke>(source).is_ok()
}

/// Attempts to parse a worstrun splits file.
pub fn parse<R: Read>(source: R) -> Result<Run> {
    let splits: Splits = from_reader(source).context(Json)?;

    let mut run = Run::new();

    if let Some(game) = splits.game {
        run.set_game_name(game);
    }
    if let Some(category) = splits.category {
        run.set_category_name(category);
    }

    if let Some(initial_delay) = splits.initial_delay {
        run.set_offset(-TimeSpan::from_milliseconds(f64::from(initial_delay)));
    }

    // worstrun splits don't actually store the PB splits. Instead they store
    // the last attempt's split times. Therefore we include that as a single
    // attempt in the history.
    let mut attempt_time = Time::default();
    {
        let splits = splits.splits.as_ref();
        catch! {
            let split_time = splits?.last()?.last_split?;
            if split_time > 0 {
                attempt_time.real_time = Some(TimeSpan::from_milliseconds(f64::from(split_time)));
            }
        };
    }

    run.add_attempt_with_index(attempt_time, 1, None, None, None);

    if let Some(splits) = splits.splits {
        let mut last_split_time = TimeSpan::zero();

        for split in splits {
            let mut segment = Segment::new(split.title.unwrap_or_default());
            let mut attempt_time = Time::default();
            if let Some(split_time) = split.last_split {
                if split_time > 0 {
                    let split_time = TimeSpan::from_milliseconds(f64::from(split_time));
                    let segment_time = split_time - last_split_time;
                    last_split_time = split_time;
                    attempt_time.real_time = Some(split_time);
                    segment.set_best_segment_time(Time::new().with_real_time(Some(segment_time)));
                }
            }
            segment.segment_history_mut().insert(1, attempt_time);

            run.push_segment(segment);
        }
    }

    // Either the record time or the last segment's recorded split time is the
    // PB split time of the last segment.
    if let Some(segment) = run.segments_mut().last_mut() {
        let mut pb_time = None;

        // Try the record time first.
        if let Some(record_time) = splits.record_time {
            if record_time > 0 {
                pb_time = Some(TimeSpan::from_milliseconds(f64::from(record_time)));
            }
        }
        // Now try the attempt time of the last segment. Only store it if it's
        // actually lower.
        if let Some(attempt_time) = segment.segment_history().get(1).and_then(|t| t.real_time) {
            if let Some(real_time) = pb_time {
                if attempt_time < real_time {
                    pb_time = Some(attempt_time);
                }
            } else {
                pb_time = Some(attempt_time);
            }
        }

        segment.set_personal_best_split_time(Time::new().with_real_time(pb_time));
    }

    Ok(run)
}
