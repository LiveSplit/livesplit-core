//! Provides the parser for the SourceLiveTimer splits files.

use std::io::Read;
use std::result::Result as StdResult;
use serde_json::de::from_reader;
use serde_json::Error as JsonError;
use {GameTime, Run, Segment, Time, TimeSpan};

quick_error! {
    /// The Error type for splits files that couldn't be parsed by the
    /// SourceLiveTimer Parser.
    #[derive(Debug)]
    pub enum Error {
        /// Failed to parse JSON.
        Json(err: JsonError) {
            from()
        }
    }
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
    Segment: Option<u64>,
    BestSegment: Option<u64>,
}

fn time_span_from_ticks(category_name: &str, ticks: u64) -> TimeSpan {
    let seconds = if category_name.starts_with("Portal 2") {
        ticks as f64 / 30.0
    } else {
        // Game is either Portal or Half Life 2
        ticks as f64 / 66.6666666666667
    };

    TimeSpan::from_seconds(seconds)
}

/// Attempts to parse a SourceLiveTimer splits file.
pub fn parse<R: Read>(source: R) -> Result<Run> {
    let mut run = Run::new();
    let splits: Splits = from_reader(source)?;

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

            let pb_split_time = if let Some(segment) = split.Segment {
                time_span_from_ticks(run.category_name(), segment)
            } else {
                TimeSpan::zero()
            };

            let best_split_time = if let Some(best) = split.BestSegment {
                time_span_from_ticks(run.category_name(), best)
            } else {
                pb_split_time
            };

            segment.set_personal_best_split_time(Time::from(GameTime(Some(pb_split_time))));
            segment.set_best_segment_time(Time::from(GameTime(Some(best_split_time))));
            run.push_segment(segment);
        }
    }

    Ok(run)
}
