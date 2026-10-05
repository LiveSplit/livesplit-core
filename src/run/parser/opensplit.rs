//! Provides the parser for OpenSplit splits files.
//!
// https://github.com/ZellyDev-Games/OpenSplit

use crate::{
    Lang, Run, Segment, Time, TimeSpan,
    comparison::world_record,
    platform::prelude::*,
    run::{SegmentGroup, SegmentGroups},
    timing::formatter::{self, TimeFormatter},
};
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
    #[serde(default, borrow)]
    variables: Vec<VariablePayload<'a>>,
    #[serde(default)]
    wr: Option<WorldRecordPayload>,
}

#[derive(Deserialize)]
struct VariablePayload<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    #[serde(borrow)]
    label: Cow<'a, str>,
}

#[derive(Deserialize)]
struct WorldRecordPayload {
    real_time: f64,
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

struct FlattenedSegments<'a> {
    segments: Vec<SegmentPayload<'a>>,
    groups: Vec<SegmentGroup>,
}

fn flatten_segments<'a>(segments: Vec<SegmentPayload<'a>>) -> FlattenedSegments<'a> {
    let mut flattened = FlattenedSegments {
        segments: Vec::with_capacity(segments.len()),
        groups: Vec::new(),
    };

    for segment in segments {
        flatten_segment(segment, true, &mut flattened);
    }

    flattened
}

fn flatten_segment<'a>(
    mut segment: SegmentPayload<'a>,
    allow_group: bool,
    flattened: &mut FlattenedSegments<'a>,
) {
    if segment.children.is_empty() {
        flattened.segments.push(segment);
        return;
    }

    let start = flattened.segments.len();
    for child in segment.children.drain(..) {
        flatten_segment(child, false, flattened);
    }
    let end = flattened.segments.len();

    if allow_group {
        flattened.groups.push(SegmentGroup::new_unchecked(
            start,
            end,
            Some(segment.name.into_owned()),
        ));
    }
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

    let metadata = run.metadata_mut();
    metadata.set_platform_name(splits.platform);
    for variable in splits.variables {
        metadata.set_speedrun_com_variable(variable.name, variable.label);
    }
    if let Some(wr) = splits.wr
        && wr.real_time > 0.0
    {
        metadata
            .custom_variable_mut(world_record::NAME)
            .permanent()
            // FIXME: This should probably depend on the locale or:
            // FIXME: Custom variables should support TimeSpans directly.
            .set_value(
                formatter::Regular::new()
                    .format(Some(TimeSpan::from_seconds(wr.real_time)), Lang::English)
                    .to_string(),
            );
    }

    let FlattenedSegments {
        segments: leaf_segments,
        groups,
    } = flatten_segments(splits.segments);

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
    *run.segment_groups_mut() = SegmentGroups::from_vec_lossy(groups, run.len());

    for (attempt_history_index, run_payload) in (1..).zip(splits.runs) {
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
    }

    Ok(run)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_speedrun_com_metadata() {
        let run = parse(
            r#"{
                "game_name": "Game",
                "speedrun_game_id": "game-id",
                "game_category": "Any%",
                "speedrun_game_category_id": "category-id",
                "attempts": 0,
                "platform": "GameCube",
                "variables": [
                    {
                        "id": "difficulty-id",
                        "name": "Difficulty",
                        "value": "hard-id",
                        "label": "Hard"
                    },
                    {
                        "id": "players-id",
                        "name": "Players",
                        "value": "one-player-id",
                        "label": "1 Player"
                    }
                ],
                "wr": {
                    "show": true,
                    "run_id": "world-record-run-id",
                    "players": ["Runner"],
                    "real_time": 3723.456,
                    "in_game_time": 3600.0
                },
                "segments": []
            }"#,
        )
        .unwrap();

        let metadata = run.metadata();
        assert_eq!(metadata.platform_name(), "GameCube");
        assert_eq!(
            metadata
                .speedrun_com_variables()
                .map(|(name, value)| (name, value.as_str()))
                .collect::<Vec<_>>(),
            [("Difficulty", "Hard"), ("Players", "1 Player")]
        );

        let world_record = metadata.custom_variable(world_record::NAME).unwrap();
        assert_eq!(world_record.value, "1:02:03");
        assert!(world_record.is_permanent);
    }

    #[test]
    fn ignores_unavailable_world_record() {
        let run = parse(
            r#"{
                "game_name": "Game",
                "game_category": "Any%",
                "attempts": 0,
                "wr": {
                    "show": false,
                    "run_id": "",
                    "players": [],
                    "real_time": 0,
                    "in_game_time": 0
                },
                "segments": []
            }"#,
        )
        .unwrap();

        assert!(run.metadata().custom_variable(world_record::NAME).is_none());
    }

    #[test]
    fn segment_children_become_segment_groups() {
        let run = parse(
            r#"{
                "game_name": "Game",
                "game_category": "Any%",
                "attempts": 0,
                "segments": [
                    { "id": "intro", "name": "Intro", "gold": 0, "pb": 0 },
                    {
                        "id": "chapter",
                        "name": "Chapter",
                        "gold": 0,
                        "pb": 0,
                        "children": [
                            { "id": "a", "name": "A", "gold": 0, "pb": 0 },
                            {
                                "id": "nested",
                                "name": "Nested",
                                "gold": 0,
                                "pb": 0,
                                "children": [
                                    { "id": "b", "name": "B", "gold": 0, "pb": 0 },
                                    { "id": "c", "name": "C", "gold": 0, "pb": 0 }
                                ]
                            }
                        ]
                    },
                    { "id": "outro", "name": "Outro", "gold": 0, "pb": 0 }
                ]
            }"#,
        )
        .unwrap();

        assert_eq!(
            run.segments()
                .iter()
                .map(|segment| segment.name())
                .collect::<Vec<_>>(),
            ["Intro", "A", "B", "C", "Outro"]
        );
        assert_eq!(run.segment_groups().groups().len(), 1);
        let group = &run.segment_groups().groups()[0];
        assert_eq!((group.start(), group.end()), (1, 4));
        assert_eq!(group.name(), Some("Chapter"));
    }
}
