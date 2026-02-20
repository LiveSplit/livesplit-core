//! Provides the parser for OpenSplit splits files.
//!
// https://github.com/ZellyDev-Games/OpenSplit

use crate::{Run, Segment, Time, TimeSpan, platform::prelude::*};
use alloc::{borrow::Cow, collections::BTreeMap};
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
    #[serde(default)]
    segments: Vec<SegmentPayload<'a>>,
    attempts: u32,
    #[serde(default)]
    runs: Vec<RunPayload<'a>>,
    #[serde(default)]
    offset: i64,
    #[serde(default, borrow)]
    platform: Cow<'a, str>,
}

#[derive(Deserialize)]
struct RunPayload<'a> {
    total_time: i64,
    completed: bool,
    #[serde(default, borrow)]
    splits: BTreeMap<Cow<'a, str>, SplitPayload>,
}

#[derive(Deserialize)]
struct SplitPayload {
    #[allow(dead_code)]
    current_cumulative: i64,
    current_duration: i64,
}

#[derive(Deserialize)]
struct SegmentPayload<'a> {
    #[serde(borrow)]
    id: Cow<'a, str>,
    #[serde(borrow)]
    name: Cow<'a, str>,
    gold: i64,
    pb: i64,
    #[serde(default, borrow)]
    children: Vec<SegmentPayload<'a>>,
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

fn integer_time(milliseconds: i64) -> TimeSpan {
    crate::platform::Duration::milliseconds(milliseconds).into()
}

fn flatten_leaf_segments<'a>(segments: Vec<SegmentPayload<'a>>) -> Vec<SegmentPayload<'a>> {
    let mut leaf_segments = Vec::with_capacity(segments.len());
    let mut stack = Vec::with_capacity(segments.len());
    stack.extend(segments.into_iter().rev());

    while let Some(mut segment) = stack.pop() {
        if segment.children.is_empty() {
            leaf_segments.push(segment);
        } else {
            stack.extend(segment.children.drain(..).rev());
        }
    }

    leaf_segments
}

/// Attempts to parse an OpenSplit splits file.
pub fn parse(source: &str) -> Result<Run> {
    let splits: SplitFilePayload<'_> =
        serde_json::from_str(source).map_err(|source| Error::Json { source })?;

    let mut run = Run::new();

    run.set_game_name(splits.game_name);
    run.set_category_name(splits.game_category);
    run.set_attempt_count(splits.attempts);
    run.set_offset(integer_time(splits.offset));
    run.metadata_mut().set_platform_name(splits.platform);

    let leaf_segments = flatten_leaf_segments(splits.segments);

    let mut segment_ids = Vec::with_capacity(leaf_segments.len());
    let mut cumulative_pb = TimeSpan::zero();

    for segment_payload in leaf_segments {
        segment_ids.push(segment_payload.id);

        let mut segment = Segment::new(segment_payload.name);

        segment.set_best_segment_time(nullable(integer_time(segment_payload.gold)));

        cumulative_pb += integer_time(segment_payload.pb);
        segment.set_personal_best_split_time(Time::new().with_real_time(
            if segment_payload.pb != 0 {
                Some(cumulative_pb)
            } else {
                None
            },
        ));

        run.push_segment(segment);
    }

    let mut attempt_history_index = 1;

    for run_payload in splits.runs {
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

        let last = segment_ids
            .iter()
            .enumerate()
            .rfind(|(_, segment_id)| run_payload.splits.contains_key(*segment_id))
            .map_or(0, |(index, _)| index + 1);

        for (segment_id, segment) in segment_ids[..last].iter().zip(run.segments_mut()) {
            let mut time = Time::new();
            if let Some(split_payload) = run_payload.splits.get(segment_id) {
                time = time.with_real_time(Some(integer_time(split_payload.current_duration)));
            }
            segment
                .segment_history_mut()
                .insert(attempt_history_index, time);
        }

        attempt_history_index += 1;
    }

    Ok(run)
}
