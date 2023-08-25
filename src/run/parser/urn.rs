//! Provides the parser for Urn splits files.

use crate::{platform::prelude::*, Run, Segment, Time, TimeSpan};
use alloc::borrow::Cow;
use core::result::Result as StdResult;
use serde_derive::Deserialize;
use serde_json::Error as JsonError;

/// The Error type for splits files that couldn't be parsed by the Urn
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

/// The Result type for the Urn Parser.
pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
struct Splits<'a> {
    #[serde(borrow)]
    title: Option<Cow<'a, str>>,
    attempt_count: Option<u32>,
    start_delay: Option<TimeSpan>,
    splits: Option<Vec<Split<'a>>>,
}

#[derive(Deserialize)]
struct Split<'a> {
    #[serde(borrow)]
    title: Option<Cow<'a, str>>,
    time: Option<TimeSpan>,
    best_time: Option<TimeSpan>,
    best_segment: Option<TimeSpan>,
}

fn parse_time(real_time: TimeSpan) -> Time {
    // Empty Time is stored as zero
    let real_time = if real_time != TimeSpan::zero() {
        Some(real_time)
    } else {
        None
    };

    Time::new().with_real_time(real_time)
}

/// Attempts to parse an Urn splits file.
pub fn parse(source: &str) -> Result<Run> {
    let splits: Splits<'_> =
        serde_json::from_str(source).map_err(|source| Error::Json { source })?;

    let mut run = Run::new();

    if let Some(title) = splits.title {
        run.set_category_name(title);
    }
    if let Some(attempt_count) = splits.attempt_count {
        run.set_attempt_count(attempt_count);
    }
    if let Some(start_delay) = splits.start_delay {
        run.set_offset(-start_delay);
    }

    // Best Split Times can be used for the Segment History Every single best
    // split time should be included as its own run, since the best split times
    // could be apart from each other less than the best segments, so we have to
    // assume they are from different runs.
    let mut attempt_history_index = 1;

    if let Some(splits) = splits.splits {
        for split in splits {
            let mut segment = Segment::new(split.title.unwrap_or_default());
            if let Some(time) = split.time {
                segment.set_personal_best_split_time(parse_time(time));
            }
            if let Some(best_segment) = split.best_segment {
                segment.set_best_segment_time(parse_time(best_segment));
            }

            if let Some(best_time) = split.best_time {
                let best_split_time = parse_time(best_time);
                if best_split_time.real_time.is_some() {
                    run.add_attempt_with_index(
                        Time::default(),
                        attempt_history_index,
                        None,
                        None,
                        None,
                    );

                    // Insert a new run that skips to the current split
                    for already_inserted_segment in run.segments_mut() {
                        already_inserted_segment
                            .segment_history_mut()
                            .insert(attempt_history_index, Time::default());
                    }

                    segment
                        .segment_history_mut()
                        .insert(attempt_history_index, best_split_time);

                    attempt_history_index += 1;
                }
            }

            run.push_segment(segment);
        }
    }

    Ok(run)
}
