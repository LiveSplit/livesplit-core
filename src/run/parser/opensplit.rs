//! Provides the parser for OpenSplit splits files.
//!
// https://github.com/ZellyDev-Games/OpenSplit

use crate::{Run, Segment, Time, TimeSpan, platform::prelude::*};
use alloc::borrow::Cow;
use core::result::Result as StdResult;
use serde_derive::Deserialize;
use serde_json::Error as JsonError;

/// The Error type for splits files that couldn't be parsed by the OpenSplit
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

/// The Result type for the OpenSplit Parser.
pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
struct SplitFilePayload<'a> {
    #[serde(borrow)]
    game_name: Cow<'a, str>,
    #[serde(borrow)]
    game_category: Cow<'a, str>,
    segments: Option<Vec<SegmentPayload<'a>>>,
    attempts: u32,
    runs: Option<Vec<RunPayload<'a>>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RunPayload<'a> {
    total_time: i64,
    completed: bool,
    #[serde(borrow)]
    split_payloads: Option<Vec<SplitPayload<'a>>>,
}

#[derive(Deserialize)]
struct SplitPayload<'a> {
    #[serde(borrow)]
    split_segment_id: Cow<'a, str>,
    // FIXME: Is current_time the correct field?
    // current_time: TimeSpan,
    current_duration: i64,
}

#[derive(Deserialize)]
struct SegmentPayload<'a> {
    #[serde(borrow)]
    id: Cow<'a, str>,
    #[serde(borrow)]
    name: Cow<'a, str>,
    best_time: TimeSpan,
    // FIXME: Would need to be stored as part of the segment history
    // average_time: TimeSpan,
}

fn nullable(real_time: TimeSpan) -> Time {
    // Empty Time is stored as zero
    let real_time = if real_time != TimeSpan::zero() {
        Some(real_time)
    } else {
        None
    };

    Time::new().with_real_time(real_time)
}

fn integer_time(nanos: i64) -> TimeSpan {
    crate::platform::Duration::nanoseconds(nanos).into()
}

/// Attempts to parse an OpenSplit splits file.
pub fn parse(source: &str) -> Result<Run> {
    let splits: SplitFilePayload<'_> =
        serde_json::from_str(source).map_err(|source| Error::Json { source })?;

    let mut run = Run::new();

    run.set_game_name(splits.game_name);
    run.set_category_name(splits.game_category);
    run.set_attempt_count(splits.attempts);

    if let Some(segments) = splits.segments {
        let mut segment_ids = Vec::with_capacity(segments.len());

        for segment_payload in segments {
            segment_ids.push(segment_payload.id);
            let mut segment = Segment::new(segment_payload.name);
            segment.set_personal_best_split_time(nullable(segment_payload.best_time));
            run.push_segment(segment);
        }

        let mut attempt_history_index = 1;

        if let Some(runs) = splits.runs {
            for run_payload in runs {
                run.add_attempt_with_index(
                    Time::new().with_real_time(if run_payload.completed {
                        Some(integer_time(run_payload.total_time))
                    } else {
                        None
                    }),
                    attempt_history_index,
                    None,
                    None,
                    None,
                );

                let mut current_time = 0;
                let mut previous_idx = None;

                if let Some(split_payloads) = run_payload.split_payloads {
                    for split_payload in split_payloads {
                        if let Some(idx) = segment_ids
                            .iter()
                            .position(|id| *id == split_payload.split_segment_id)
                            && previous_idx.is_none_or(|prev| idx > prev)
                        {
                            let segment_time = split_payload.current_duration - current_time;

                            run.segments_mut()[idx].segment_history_mut().insert(
                                attempt_history_index,
                                Time::new().with_real_time(Some(integer_time(segment_time))),
                            );

                            current_time = split_payload.current_duration;
                            previous_idx = Some(idx);
                        }
                    }
                }

                attempt_history_index += 1;
            }
        }
    }

    for segment in run.segments_mut() {
        if let Some(segment_time) = segment
            .segment_history()
            .iter()
            .filter_map(|(_, time)| time.real_time)
            .min()
        {
            segment.set_best_segment_time(Time::new().with_real_time(Some(segment_time)));
        }
    }

    Ok(run)
}
