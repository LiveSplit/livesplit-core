//! Provides the parser for Splitterino splits files.

use crate::{Run, Segment, Time, TimeSpan};
use serde::Deserialize;
use serde_json::de::from_reader;
use serde_json::Error as JsonError;
use snafu::ResultExt;
use std::io::Read;
use std::result::Result as StdResult;

/// The Error type for splits files that couldn't be parsed by the Splitterino
/// Parser.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Failed to parse JSON.
    Json {
        /// The underlying error.
        source: JsonError,
    },
}

/// The Result type for the Splitterino Parser.
pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
struct SplitsFormat {
    // version: String,
    splits: Splits,
}

/// Format in which splits are getting saved to file or should be transmitted
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Splits {
    /// The Game Information about this run
    game: GameInfo,
    /// The delay of how much time the timer should wait when starting a new run in milliseconds
    start_delay: Option<i64>,
    /// An array of segments which are associated to these splits
    segments: Vec<SplitterinoSegment>,
    // /// The timing-method which is used for the splits
    // timing: SplitterinoTimingMethod,
}

/// Timing methods which can be used for segment times
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum SplitterinoTimingMethod {
    Igt,
    Rta,
}

/// Detailed information about the game and run details
#[derive(Deserialize, Default)]
#[serde(default)]
struct GameInfo {
    /// Name of the Game that is currently being run
    name: String,
    /// Category that is currently being run
    category: String,
    /// The Platform on which the Game is being run on
    platform: String,
    /// The Region format the game is run in
    region: String,
}

/// Describes a single Segment
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SplitterinoSegment {
    // /// The ID which identifies the Segment
    // id: String,
    /// The name of the Segment
    name: String,
    /// The time of the personal best in milliseconds
    personal_best: Option<SegmentTime>,
    /// The time of the overall best in milliseconds
    overall_best: Option<SegmentTime>,
    // /// If the Segment has been passed successfully
    // passed: Option<bool>,
    /// If the Segment has been skipped
    #[serde(default)]
    skipped: bool,
}

/// Format for detailed times with multiple timing methods
#[derive(Deserialize)]
struct SegmentTime {
    /// The detailed time of a Segment for IGT
    igt: DetailedTime,
    /// The detailed time of a Segment for RTA
    rta: DetailedTime,
}

/// Format in which times can easily be represented and stored
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DetailedTime {
    /// The time of the segment in milliseconds which count towards the run
    raw_time: u64,
    /// The time of the segment in milliseconds which were spent paused
    pause_time: u64,
}

fn parse_segment_time(SegmentTime { igt, rta }: SegmentTime) -> [TimeSpan; 2] {
    [
        TimeSpan::from_milliseconds((rta.raw_time - rta.pause_time) as _),
        TimeSpan::from_milliseconds((igt.raw_time - igt.pause_time) as _),
    ]
}

fn parse_best_segment_time(segment_time: SegmentTime) -> Time {
    let [rta, igt] = parse_segment_time(segment_time);
    let mut time = Time::new();
    if rta != TimeSpan::zero() {
        time.real_time = Some(rta);
    }
    if igt != TimeSpan::zero() {
        time.game_time = Some(igt);
    }
    time
}

fn parse_split_time(
    total_rta: &mut TimeSpan,
    total_igt: &mut TimeSpan,
    segment_time: SegmentTime,
) -> Time {
    let [rta, igt] = parse_segment_time(segment_time);

    let real_time = if rta != TimeSpan::zero() {
        *total_rta += rta;
        Some(*total_rta)
    } else {
        None
    };

    let game_time = if igt != TimeSpan::zero() {
        *total_igt += igt;
        Some(*total_igt)
    } else {
        None
    };

    Time {
        real_time,
        game_time,
    }
}

/// Attempts to parse a Splitterino splits file.
pub fn parse<R: Read>(source: R) -> Result<Run> {
    let SplitsFormat { splits, .. } = from_reader(source).context(Json)?;

    let mut run = Run::new();

    run.set_game_name(splits.game.name);
    run.set_category_name(splits.game.category);
    run.metadata_mut().set_platform_name(splits.game.platform);
    run.metadata_mut().set_region_name(splits.game.region);

    // FIXME: Region may need to be translated to speedrun.com's region.
    // FIXME: Parse pause times.
    // FIXME: Parse default timing method, if we ever store that.

    if let Some(start_delay) = splits.start_delay {
        run.set_offset(TimeSpan::from_milliseconds(-start_delay as _));
    }

    let (mut total_rta, mut total_igt) = (TimeSpan::zero(), TimeSpan::zero());

    for split in splits.segments {
        let mut segment = Segment::new(split.name);

        if !split.skipped {
            if let Some(personal_best) = split.personal_best {
                segment.set_personal_best_split_time(parse_split_time(
                    &mut total_rta,
                    &mut total_igt,
                    personal_best,
                ));
            }
        }

        if let Some(overall_best) = split.overall_best {
            segment.set_best_segment_time(parse_best_segment_time(overall_best));
        }

        run.push_segment(segment);
    }

    Ok(run)
}
