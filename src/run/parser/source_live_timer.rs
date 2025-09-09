//! Provides the parser for the SourceLiveTimer splits files.

use crate::{platform::prelude::*, GameTime, Run, Segment, TimeSpan};
use alloc::borrow::Cow;
use core::result::Result as StdResult;
use serde_derive::Deserialize;
use serde_json::Error as JsonError;

/// The Error type for splits files that couldn't be parsed by the
/// SourceLiveTimer Parser.
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

/// The Result type for the SourceLiveTimer parser.
pub type Result<T> = StdResult<T, Error>;

#[expect(non_snake_case)]
#[derive(Deserialize)]
struct Splits<'a> {
    #[serde(borrow)]
    Category: Cow<'a, str>,
    #[serde(borrow)]
    RunName: Option<Cow<'a, str>>,
    Splits: Option<Vec<Split<'a>>>,
}

#[expect(non_snake_case)]
#[derive(Deserialize)]
struct Split<'a> {
    #[serde(borrow)]
    Map: Cow<'a, str>,
    #[serde(borrow)]
    Name: Option<Cow<'a, str>>,
    Ticks: Option<u64>,
    BestSegment: Option<u64>,
}

fn time_span_from_ticks(is_portal2: bool, ticks: u64) -> TimeSpan {
    let seconds = if is_portal2 {
        ticks as f64 / 30.0
    } else {
        // Game is either Portal or Half Life 2
        ticks as f64 / 66.666_666_666_666_7
    };

    TimeSpan::from_seconds(seconds)
}

/// Attempts to parse a SourceLiveTimer splits file.
pub fn parse(source: &str) -> Result<Run> {
    let splits: Splits<'_> =
        serde_json::from_str(source).map_err(|source| Error::Json { source })?;

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
        let is_portal2 = run.category_name().starts_with("Portal 2");

        run.segments_mut().extend(segments.into_iter().map(|split| {
            let mut segment = Segment::new(split.Name.unwrap_or(split.Map));

            if let Some(ticks) = split.Ticks {
                let pb_split_time = Some(time_span_from_ticks(is_portal2, ticks));
                segment.set_personal_best_split_time(GameTime(pb_split_time).into());
            }

            if let Some(best) = split.BestSegment {
                let best_segment_time = Some(time_span_from_ticks(is_portal2, best));
                segment.set_best_segment_time(GameTime(best_segment_time).into());
            }

            segment
        }));
    }

    Ok(run)
}
