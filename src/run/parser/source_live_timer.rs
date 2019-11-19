//! Provides the parser for the SourceLiveTimer splits files.

use crate::{GameTime, Run, Segment, TimeSpan};
use core::result::Result as StdResult;
use serde::Deserialize;
use serde_json::de::from_reader;
use serde_json::Error as JsonError;
use snafu::ResultExt;
use std::io::Read;

/// The Error type for splits files that couldn't be parsed by the
/// SourceLiveTimer Parser.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// Failed to parse JSON.
    Json {
        /// The underlying error.
        source: JsonError,
    },
}

/// The Result type for the SourceLiveTimer parser.
pub type Result<T> = StdResult<T, Error>;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Splits {
    Category: String,
    RunName: Option<String>,
    Splits: Option<Vec<Split>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Split {
    Map: String,
    Name: Option<String>,
    Ticks: Option<u64>,
    BestSegment: Option<u64>,
}

fn time_span_from_ticks(category_name: &str, ticks: u64) -> TimeSpan {
    let seconds = if category_name.starts_with("Portal 2") {
        ticks as f64 / 30.0
    } else {
        // Game is either Portal or Half Life 2
        ticks as f64 / 66.666_666_666_666_7
    };

    TimeSpan::from_seconds(seconds)
}

/// Attempts to parse a SourceLiveTimer splits file.
pub fn parse<R: Read>(source: R) -> Result<Run> {
    let splits: Splits = from_reader(source).context(Json)?;

    let mut run = Run::new();

    if splits.Category.starts_with("Portal 2") {
        run.set_game_name("Portal 2");
    } else if splits.Category.starts_with("Portal") {
        run.set_game_name("Portal");
    } else if splits.Category.starts_with("Half Life 2") {
        run.set_game_name("Half Life 2");
    }

    if let Some(run_name) = splits.RunName {
        if run_name != splits.Category {
            run.set_category_name(run_name);
        } else {
            run.set_category_name(splits.Category);
        }
    } else {
        run.set_category_name(splits.Category);
    }

    if let Some(segments) = splits.Splits {
        for split in segments {
            let name = if let Some(name) = split.Name {
                name.to_owned()
            } else {
                split.Map.to_owned()
            };

            let mut segment = Segment::new(name);

            if let Some(ticks) = split.Ticks {
                let pb_split_time = Some(time_span_from_ticks(run.category_name(), ticks));
                segment.set_personal_best_split_time(GameTime(pb_split_time).into());
            }

            if let Some(best) = split.BestSegment {
                let best_segment_time = Some(time_span_from_ticks(run.category_name(), best));
                segment.set_best_segment_time(GameTime(best_segment_time).into());
            }

            run.push_segment(segment);
        }
    }

    Ok(run)
}
