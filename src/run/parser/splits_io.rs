//! Provides the parser for generic Splits I/O splits files.

use serde_json::de::from_reader;
use serde_json::Error as JsonError;
use std::io::Read;
use std::result::Result as StdResult;
use {Run, Segment as LiveSplitSegment, Time, TimeSpan};

quick_error! {
    /// The Error type for splits files that couldn't be parsed by the generic
    /// Splits I/O Parser.
    #[derive(Debug)]
    pub enum Error {
        /// Failed to parse JSON.
        Json(err: JsonError) {
            from()
        }
    }
}

/// The Result type for the generic Splits I/O Parser.
pub type Result<T> = StdResult<T, Error>;

/// Duration holds a realtime duration and a gametime duration.
#[serde(rename = "duration")]
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
struct Duration {
    /// Gametime (Milliseconds) is a duration of milliseconds in game-world time.
    #[serde(rename = "gametimeMS")]
    gametime_ms: Option<f64>,
    /// Realtime (Milliseconds) is a duration of milliseconds in real-world time.
    #[serde(rename = "realtimeMS")]
    realtime_ms: Option<f64>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Attempt {
    /// Attempt Number is the number of lifetime attempts the runner will have made after this one.
    /// The Attempt Number for an attempt is a label, not an index; the first attempt for a
    /// category has an Attempt Number of 1 (not 0).
    #[serde(rename = "attemptNumber")]
    attempt_number: u32,
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
struct CategoryLinks {
    /// Speedrun.com ID specifies the category's Speedrun.com ID.
    #[serde(rename = "speedruncomID")]
    speedruncom_id: Option<String>,
    /// Splits I/O ID specifies the category's Splits I/O ID.
    #[serde(rename = "splitsioID")]
    splitsio_id: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Category {
    /// Links specifies the category's identity in other services.
    links: Option<CategoryLinks>,
    /// Longname is a human-readable category name, intended for display to users.
    longname: String,
    /// Shortname is a machine-readable category name, intended for use in APIs, databases, URLs,
    /// and filenames.
    shortname: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
struct GameLinks {
    /// Speedrun.com ID specifies the game's Speedrun.com ID.
    #[serde(rename = "speedruncomID")]
    speedruncom_id: Option<String>,
    /// Splits I/O ID specifies the game's Splits I/O ID.
    #[serde(rename = "splitsioID")]
    splitsio_id: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Game {
    /// Links specifies the game's identity in other services.
    links: Option<GameLinks>,
    /// Longname is a human-readable game name, intended for display to users.
    longname: String,
    /// Shortname is a machine-readable game name, intended for use in APIs, databases, URLs, and
    /// filenames.
    shortname: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
struct RunLinks {
    /// Speedrun.com ID is the run's ID on Speedrun.com. This can be used to communicate with the
    /// Speedrun.com API.
    #[serde(rename = "speedruncomID")]
    speedruncom_id: Option<String>,
    /// Splits I/O ID is the run's ID on Splits I/O. This can be used to communicate with the
    /// Splits I/O API.
    #[serde(rename = "splitsioID")]
    splitsio_id: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Pause {
    /// Ended At is the date and time at which the pause was ended, specified in RFC 3339 format.
    #[serde(rename = "endedAt")]
    ended_at: Option<String>,
    /// Started At is the date and time at which the pause was started, specified in RFC 3339
    /// format.
    #[serde(rename = "startedAt")]
    started_at: String,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
struct RunnerLinks {
    /// Speedrun.com ID specifies the runner's Speedrun.com ID.
    #[serde(rename = "speedruncomID")]
    speedruncom_id: Option<String>,
    /// Splits I/O ID specifies the runner's Splits I/O ID.
    #[serde(rename = "splitsioID")]
    splitsio_id: Option<String>,
    /// Twitch ID specifies the runner's Twitch ID.
    #[serde(rename = "twitchID")]
    twitch_id: Option<String>,
    /// Twitter ID specifies the runner's Twitter ID.
    #[serde(rename = "twitterID")]
    twitter_id: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Runner {
    /// Links specifies the runner's identity in other services.
    links: Option<RunnerLinks>,
    /// Longname is a human-readable runner name, intended for display to users.
    longname: Option<String>,
    /// Shortname is a machine-readable runner name, intended for use in APIs, databases, URLs, and
    /// filenames.
    shortname: String,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct History {
    /// Attempt Number is the number of lifetime attempts the runner will have made on this
    /// category after this one. Generally these attempt numbers should correspond to those in
    /// Attempts -> History, although a number given here may not be present there if the run was
    /// reset before completion.
    #[serde(rename = "attemptNumber")]
    attempt_number: f64,
    duration: Option<Duration>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
struct Segment {
    #[serde(rename = "bestDuration")]
    best_duration: Option<Duration>,
    duration: Option<Duration>,
    /// Histories is an array of previous completions of this segment by this runner.
    histories: Option<Vec<History>>,
    /// Is Skipped should be true if the runner skipped over the split that ends this segment,
    /// rather than splitting. If so, this segment's Duration is ignored and the next segment's
    /// Duration will be treated as invalid for historical purposes (but may still used for display
    /// purposes).
    #[serde(rename = "isSkipped")]
    is_skipped: Option<bool>,
    /// Name is the runner-provided name of this segment
    name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Timer {
    /// Longname is a human-readable timer name, intended for display to users.
    longname: String,
    /// Shortname is a machine-readable timer name, intended for use in APIs, databases, URLs, and
    /// filenames.
    shortname: String,
    /// Version is the version of the timer used to record this run. Semantic Versioning is
    /// strongly recommended but not enforced.
    version: String,
    /// Website is the URL for the timer's website.
    website: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
struct Splits {
    /// Schema Version specifies which version of the Splits I/O JSON Schema is being used. This
    /// schema specifies only v1.0.0.
    #[serde(rename = "_schemaVersion")]
    _schemaversion: String,
    /// Attempts contains historical information about previous runs by this runner in this
    /// category.
    attempts: Option<Attempts>,
    /// Category specifies information about the category being run.
    category: Option<Category>,
    /// Ended At is the date and time at which the run was ended, specified in RFC 3339 format.
    #[serde(rename = "endedAt")]
    ended_at: Option<String>,
    /// Game specifies information about the game being run.
    game: Option<Game>,
    /// Image URL is the location of an image associated with this run. Often this is a screenshot
    /// of the timer at run completion, but can be anything the runner wants displayed alongside
    /// the run.
    #[serde(rename = "imageURL")]
    image_url: Option<String>,
    /// Links specifies the run's identity in other services.
    links: Option<RunLinks>,
    /// Pauses holds runner-caused pauses that took place during the run.
    pauses: Option<Vec<Pause>>,
    /// Runners is an array of people who participated in this run. Some games and categories call
    /// for cooperative play, but otherwise this will usually be just one person.
    runners: Option<Vec<Runner>>,
    /// Segments is an array of all segments for this run.
    segments: Option<Vec<Segment>>,
    /// Started At is the date and time at which the run was started, specified in RFC 3339 format.
    #[serde(rename = "startedAt")]
    started_at: Option<String>,
    /// Timer holds information about the timer used to record the run.
    timer: Timer,
    /// Video URL is the location of a VOD of the run.
    #[serde(rename = "videoURL")]
    video_url: Option<String>,
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

/// Attempts to parse a Portal 2 Live Timer splits file.
pub fn parse<R: Read>(source: R) -> Result<Run> {
    let splits: Splits = from_reader(source)?;

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
        for attempt in attempts.histories.into_iter().flat_map(|x| x) {
            run.add_attempt(attempt.duration.into(), None, None, None);
        }
    }

    let (mut accum_real, mut accum_game) = (TimeSpan::zero(), TimeSpan::zero());
    for split in splits.segments.into_iter().flat_map(|x| x) {
        let mut segment = LiveSplitSegment::new(split.name.unwrap_or_default());
        if let Some(duration) = split.duration {
            let mut pb_time = Time::new();
            let time = Time::from(duration);
            if let Some(real) = time.real_time {
                accum_real += real;
                pb_time.real_time = Some(accum_real);
            }
            if let Some(game) = time.game_time {
                accum_game += game;
                pb_time.game_time = Some(accum_game);
            }
            segment.set_personal_best_split_time(pb_time);
        }
        segment.set_best_segment_time(split.best_duration.into());

        run.push_segment(segment);
    }

    if let Some(links) = splits.links {
        if let Some(link) = links.speedruncom_id {
            run.metadata_mut().run_id = link;
        }
    }

    Ok(run)
}
