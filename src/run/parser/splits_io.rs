//! Provides the parser for generic Splits I/O splits files.

use crate::{
    platform::prelude::*, util::PopulateString, Run, Segment as LiveSplitSegment, Time, TimeSpan,
};
use alloc::borrow::Cow;
use core::result::Result as StdResult;
use serde_derive::{Deserialize, Serialize};
use serde_json::Error as JsonError;

/// The Error type for splits files that couldn't be parsed by the generic
/// Splits I/O Parser.
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

/// The Result type for the generic Splits I/O Parser.
pub type Result<T> = StdResult<T, Error>;

/// Duration holds a realtime duration and a gametime duration.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "duration")]
struct Duration {
    /// Gametime (Milliseconds) is a duration of milliseconds in game-world time.
    #[serde(rename = "gametimeMS")]
    gametime_ms: Option<f64>,
    /// Realtime (Milliseconds) is a duration of milliseconds in real-world time.
    #[serde(rename = "realtimeMS")]
    realtime_ms: Option<f64>,
}
/// Run Time represents a moment inside a run, and indicates the duration of the run so far at that
/// moment. It holds a realtime run duration so far and a gametime run duration so far.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename = "runTime")]
struct RunTime {
    /// Gametime (Milliseconds) is a duration a run so far in milliseconds.
    #[serde(rename = "gametimeMS")]
    gametime_ms: Option<f64>,
    /// Realtime (Milliseconds) is a duration of a run so far in milliseconds.
    #[serde(rename = "realtimeMS")]
    realtime_ms: Option<f64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Attempt {
    /// Attempt Number is the number of lifetime attempts the runner will have made after this one.
    /// The Attempt Number for an attempt is a label, not an index; the first attempt for a
    /// category has an Attempt Number of 1 (not 0).
    #[serde(rename = "attemptNumber")]
    attempt_number: i64,
    duration: Option<Duration>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
struct Attempts {
    /// Histories is an array of previous attempts by this runner of this category.
    histories: Option<Vec<Attempt>>,
    /// Total holds the total number of attempts for this category.
    total: Option<u32>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
struct CategoryLinks<'a> {
    /// Speedrun.com ID specifies the category's Speedrun.com ID.
    #[serde(rename = "speedruncomID")]
    #[serde(borrow)]
    speedruncom_id: Option<Cow<'a, str>>,
    /// Splits I/O ID specifies the category's Splits I/O ID.
    #[serde(rename = "splitsioID")]
    #[serde(borrow)]
    splitsio_id: Option<Cow<'a, str>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Category<'a> {
    /// Links specifies the category's identity in other services.
    links: Option<CategoryLinks<'a>>,
    /// Longname is a human-readable category name, intended for display to users.
    #[serde(borrow)]
    longname: Cow<'a, str>,
    /// Shortname is a machine-readable category name, intended for use in APIs, databases, URLs,
    /// and filenames.
    #[serde(borrow)]
    shortname: Option<Cow<'a, str>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
struct GameLinks<'a> {
    /// Speedrun.com ID specifies the game's Speedrun.com ID.
    #[serde(rename = "speedruncomID")]
    #[serde(borrow)]
    speedruncom_id: Option<Cow<'a, str>>,
    /// Splits I/O ID specifies the game's Splits I/O ID.
    #[serde(rename = "splitsioID")]
    #[serde(borrow)]
    splitsio_id: Option<Cow<'a, str>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Game<'a> {
    /// Links specifies the game's identity in other services.
    links: Option<GameLinks<'a>>,
    /// Longname is a human-readable game name, intended for display to users.
    #[serde(borrow)]
    longname: Cow<'a, str>,
    /// Shortname is a machine-readable game name, intended for use in APIs, databases, URLs, and
    /// filenames.
    #[serde(borrow)]
    shortname: Option<Cow<'a, str>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
struct RunLinks<'a> {
    /// Speedrun.com ID is the run's ID on Speedrun.com. This can be used to communicate with the
    /// Speedrun.com API.
    #[serde(rename = "speedruncomID")]
    #[serde(borrow)]
    speedruncom_id: Option<Cow<'a, str>>,
    /// Splits I/O ID is the run's ID on Splits I/O. This can be used to communicate with the
    /// Splits I/O API.
    #[serde(rename = "splitsioID")]
    #[serde(borrow)]
    splitsio_id: Option<Cow<'a, str>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Pause<'a> {
    /// Ended At is the date and time at which the pause was ended, specified in RFC 3339 format.
    #[serde(rename = "endedAt")]
    #[serde(borrow)]
    ended_at: Option<Cow<'a, str>>,
    /// Started At is the date and time at which the pause was started, specified in RFC 3339
    /// format.
    #[serde(rename = "startedAt")]
    #[serde(borrow)]
    started_at: Cow<'a, str>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
struct RunnerLinks<'a> {
    /// Speedrun.com ID specifies the runner's Speedrun.com ID.
    #[serde(rename = "speedruncomID")]
    #[serde(borrow)]
    speedruncom_id: Option<Cow<'a, str>>,
    /// Splits I/O ID specifies the runner's Splits I/O ID.
    #[serde(rename = "splitsioID")]
    #[serde(borrow)]
    splitsio_id: Option<Cow<'a, str>>,
    /// Twitch ID specifies the runner's Twitch ID.
    #[serde(rename = "twitchID")]
    #[serde(borrow)]
    twitch_id: Option<Cow<'a, str>>,
    /// Twitter ID specifies the runner's Twitter ID.
    #[serde(rename = "twitterID")]
    #[serde(borrow)]
    twitter_id: Option<Cow<'a, str>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Runner<'a> {
    /// Links specifies the runner's identity in other services.
    links: Option<RunnerLinks<'a>>,
    /// Longname is a human-readable runner name, intended for display to users.
    #[serde(borrow)]
    longname: Option<Cow<'a, str>>,
    /// Shortname is a machine-readable runner name, intended for use in APIs, databases, URLs, and
    /// filenames.
    #[serde(borrow)]
    shortname: Cow<'a, str>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct SegmentHistoryElement {
    /// Attempt Number is the number of lifetime attempts the runner will have made on this
    /// category after this one. Generally these attempt numbers should correspond to those in
    /// Attempts -> History, although a number given here may not be present there if the run was
    /// reset before completion.
    #[serde(rename = "attemptNumber")]
    attempt_number: i64,
    #[serde(rename = "endedAt")]
    ended_at: Option<RunTime>,
    /// Is Reset should be true if the runner reset the run during this segment. If so, this and
    /// all future segments' Ended Ats for this run are ignored.
    #[serde(rename = "isReset")]
    is_reset: Option<bool>,
    /// Is Skipped should be true if the runner skipped over the split that ends this segment,
    /// rather than splitting. If so, this segment's Ended At is ignored.
    #[serde(rename = "isSkipped")]
    is_skipped: Option<bool>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
struct Segment<'a> {
    #[serde(rename = "bestDuration")]
    best_duration: Option<Duration>,
    #[serde(rename = "endedAt")]
    ended_at: Option<RunTime>,
    /// Histories is an array of previous completions of this segment by this runner.
    histories: Option<Vec<SegmentHistoryElement>>,
    /// Is Reset should be true if the runner reset the run during this segment. If so, this and
    /// all future segments' Ended Ats for this run are ignored.
    #[serde(rename = "isReset")]
    is_reset: Option<bool>,
    /// Is Skipped should be true if the runner skipped over the split that ends this segment,
    /// rather than splitting. If so, this segment's Ended At is ignored.
    #[serde(rename = "isSkipped")]
    is_skipped: Option<bool>,
    /// Name is the runner-provided name of this segment
    #[serde(borrow)]
    name: Option<Cow<'a, str>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Timer<'a> {
    /// Longname is a human-readable timer name, intended for display to users.
    #[serde(borrow)]
    longname: Cow<'a, str>,
    /// Shortname is a machine-readable timer name, intended for use in APIs, databases, URLs, and
    /// filenames.
    #[serde(borrow)]
    shortname: Cow<'a, str>,
    /// Version is the version of the timer used to record this run. Semantic Versioning is
    /// strongly recommended but not enforced.
    #[serde(borrow)]
    version: Cow<'a, str>,
    /// Website is the URL for the timer's website.
    #[serde(borrow)]
    website: Option<Cow<'a, str>>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Splits<'a> {
    /// Schema Version specifies which version of the Splits I/O JSON Schema is being used. This
    /// schema specifies only v1.0.0.
    #[serde(rename = "_schemaVersion")]
    #[serde(borrow)]
    _schemaversion: Cow<'a, str>,
    /// Attempts contains historical information about previous runs by this runner in this
    /// category.
    attempts: Option<Attempts>,
    /// Category specifies information about the category being run.
    category: Option<Category<'a>>,
    /// Ended At is the date and time at which the run was ended, specified in RFC 3339 format.
    #[serde(rename = "endedAt")]
    #[serde(borrow)]
    ended_at: Option<Cow<'a, str>>,
    /// Game specifies information about the game being run.
    game: Option<Game<'a>>,
    /// Image URL is the location of an image associated with this run. Often this is a screenshot
    /// of the timer at run completion, but can be anything the runner wants displayed alongside
    /// the run.
    #[serde(rename = "imageURL")]
    #[serde(borrow)]
    image_url: Option<Cow<'a, str>>,
    /// Links specifies the run's identity in other services.
    links: Option<RunLinks<'a>>,
    /// Pauses holds runner-caused pauses that took place during the run.
    pauses: Option<Vec<Pause<'a>>>,
    /// Runners is an array of people who participated in this run. Some games and categories call
    /// for cooperative play, but otherwise this will usually be just one person.
    runners: Option<Vec<Runner<'a>>>,
    /// Segments is an array of all segments for this run.
    segments: Option<Vec<Segment<'a>>>,
    /// Started At is the date and time at which the run was started, specified in RFC 3339 format.
    #[serde(rename = "startedAt")]
    #[serde(borrow)]
    started_at: Option<Cow<'a, str>>,
    /// Timer holds information about the timer used to record the run.
    timer: Timer<'a>,
    /// Video URL is the location of a VOD of the run.
    #[serde(rename = "videoURL")]
    #[serde(borrow)]
    video_url: Option<Cow<'a, str>>,
}

impl From<Option<Duration>> for Time {
    fn from(duration: Option<Duration>) -> Time {
        duration.map(Into::into).unwrap_or_default()
    }
}

impl From<Duration> for Time {
    fn from(duration: Duration) -> Time {
        let mut time = Time::new();
        if let Some(ms) = duration.realtime_ms {
            time.real_time = Some(TimeSpan::from_milliseconds(ms));
        }
        if let Some(ms) = duration.gametime_ms {
            time.game_time = Some(TimeSpan::from_milliseconds(ms));
        }
        time
    }
}

impl From<Option<RunTime>> for Time {
    fn from(time: Option<RunTime>) -> Time {
        time.map(Into::into).unwrap_or_default()
    }
}

impl From<RunTime> for Time {
    fn from(run_time: RunTime) -> Time {
        let mut time = Time::new();
        if let Some(ms) = run_time.realtime_ms {
            time.real_time = Some(TimeSpan::from_milliseconds(ms));
        }
        if let Some(ms) = run_time.gametime_ms {
            time.game_time = Some(TimeSpan::from_milliseconds(ms));
        }
        time
    }
}

/// Attempts to parse a generic Splits I/O splits file.
pub fn parse(source: &str) -> Result<(Run, Cow<'_, str>)> {
    let splits: Splits<'_> =
        serde_json::from_str(source).map_err(|source| Error::Json { source })?;

    let mut run = Run::new();

    if let Some(game) = splits.game {
        run.set_game_name(game.longname);
    }
    if let Some(category) = splits.category {
        run.set_category_name(category.longname);
    }
    if let Some(attempts) = splits.attempts {
        if let Some(total) = attempts.total {
            run.set_attempt_count(total);
        }
        for attempt in attempts.histories.into_iter().flatten() {
            run.add_attempt_with_index(
                attempt.duration.into(),
                attempt.attempt_number as i32,
                None,
                None,
                None,
            );
        }
    }

    if let Some(runner) = splits
        .runners
        .and_then(|runners| runners.into_iter().next())
    {
        let name = runner.longname.unwrap_or(runner.shortname);
        if !name.trim_start().is_empty() {
            run.metadata_mut()
                .custom_variable_mut("Runner")
                .permanent()
                .set_value(name);
        }
        if let Some(links) = runner.links {
            if let Some(twitter_id) = links.twitter_id {
                run.metadata_mut()
                    .custom_variable_mut("Twitter")
                    .permanent()
                    .set_value(twitter_id);
            }
            if let Some(twitch_id) = links.twitch_id {
                run.metadata_mut()
                    .custom_variable_mut("Twitch")
                    .permanent()
                    .set_value(twitch_id);
            }
            if let Some(speedruncom_id) = links.speedruncom_id {
                run.metadata_mut()
                    .custom_variable_mut("speedrun.com")
                    .permanent()
                    .set_value(speedruncom_id);
            }
            if let Some(splitsio_id) = links.splitsio_id {
                run.metadata_mut()
                    .custom_variable_mut("Splits I/O")
                    .permanent()
                    .set_value(splitsio_id);
            }
        }
    }

    if let Some(segments) = splits.segments {
        run.segments_mut().extend(segments.into_iter().map(|split| {
            let mut segment = LiveSplitSegment::new(split.name.unwrap_or_default());
            segment.set_personal_best_split_time(split.ended_at.into());
            segment.set_best_segment_time(split.best_duration.into());
            if let Some(mut history) = split.histories {
                let segment_history = segment.segment_history_mut();
                history.sort_unstable_by_key(|x| x.attempt_number);
                for element in history {
                    segment_history.insert(element.attempt_number as i32, element.ended_at.into());
                }
            }
            segment
        }));
    }

    if let Some(links) = splits.links {
        if let Some(link) = links.speedruncom_id {
            link.populate(&mut run.metadata_mut().run_id);
        }
    }

    let timer = if splits.timer.longname.is_empty() {
        "Generic Timer".into()
    } else {
        splits.timer.longname
    };

    Ok((run, timer))
}
