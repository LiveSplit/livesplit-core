//! Provides the parser for Flitter splits files.

use crate::{comparison::world_record, platform::prelude::*, Run, Segment, Time, TimeSpan};
use core::result::Result as StdResult;
use serde_derive::Deserialize;

mod s_expressions;

pub use self::s_expressions::Error;

/// The Result type for the Flitter Parser.
pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
struct Splits<'a> {
    title: &'a str,
    category: &'a str,
    attempts: u32,
    #[serde(borrow)]
    split_names: Vec<&'a str>,
    golds: Option<Vec<Gold>>,
    personal_best: Option<Comparison>,
    world_record: Option<Comparison>,
}

#[derive(Deserialize)]
struct Gold {
    duration: Option<TimeSpan>,
}

#[derive(Deserialize)]
struct Comparison {
    splits: Vec<Split>,
}

#[derive(Deserialize)]
struct Split {
    time: Option<TimeSpan>,
}

/// Attempts to parse a Flitter splits file.
pub fn parse(source: &str) -> Result<Run> {
    let splits: Splits<'_> = self::s_expressions::from_str(source)?;

    let mut run = Run::new();

    run.set_game_name(splits.title);
    run.set_category_name(splits.category);
    run.set_attempt_count(splits.attempts);

    if splits.world_record.is_some() {
        run.add_custom_comparison(world_record::NAME).unwrap();
    }

    let segments = run.segments_mut();

    segments.extend(splits.split_names.into_iter().map(Segment::new));

    if let Some(pb) = splits.personal_best {
        for (segment, pb) in segments.iter_mut().zip(pb.splits) {
            segment.set_personal_best_split_time(Time::new().with_real_time(pb.time));
        }
    }

    if let Some(golds) = splits.golds {
        for (segment, gold) in segments.iter_mut().zip(golds) {
            segment.set_best_segment_time(Time::new().with_real_time(gold.duration));
        }
    }

    if let Some(wr) = splits.world_record {
        for (segment, wr) in segments.iter_mut().zip(wr.splits) {
            segment.comparison_mut(world_record::NAME).real_time = wr.time;
        }
    }

    Ok(run)
}
