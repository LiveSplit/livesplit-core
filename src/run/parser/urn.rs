//! Provides the parser for Urn splits files.

use serde_json::de::from_reader;
use serde_json::Error as JsonError;
use std::io::Read;
use std::result::Result as StdResult;
use {time, Run, Segment, Time, TimeSpan};

quick_error! {
    /// The Error type for splits files that couldn't be parsed by the Urn
    /// Parser.
    #[derive(Debug)]
    pub enum Error {
        /// Failed to parse a time.
        Time(err: time::ParseError) {
            from()
        }
        /// Failed to parse JSON.
        Json(err: JsonError) {
            from()
        }
    }
}

/// The Result type for the Urn Parser.
pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
struct Splits {
    title: Option<String>,
    attempt_count: Option<u32>,
    start_delay: Option<String>,
    splits: Option<Vec<Split>>,
}

#[derive(Deserialize)]
struct Split {
    title: Option<String>,
    time: Option<String>,
    best_time: Option<String>,
    best_segment: Option<String>,
}

fn parse_time(time: &str) -> Result<Time> {
    let real_time = time.parse::<TimeSpan>()?;

    // Empty Time is stored as zero
    let real_time = if real_time != TimeSpan::zero() {
        Some(real_time)
    } else {
        None
    };

    Ok(Time::new().with_real_time(real_time))
}

/// Attempts to parse an Urn splits file.
pub fn parse<R: Read>(source: R) -> Result<Run> {
    let mut run = Run::new();

    let splits: Splits = from_reader(source)?;

    if let Some(title) = splits.title {
        run.set_category_name(title);
    }
    if let Some(attempt_count) = splits.attempt_count {
        run.set_attempt_count(attempt_count);
    }
    if let Some(start_delay) = splits.start_delay {
        run.set_offset(-start_delay.parse()?);
    }

    // Best Split Times can be used for the Segment History Every single best
    // split time should be included as its own run, since the best split times
    // could be apart from each other less than the best segments, so we have to
    // assume they are from different runs.
    let mut attempt_history_index = 1;

    if let Some(splits) = splits.splits {
        for split in splits {
            let mut segment = Segment::new(split.title.unwrap_or_default());
            if let Some(time) = split.time {
                segment.set_personal_best_split_time(parse_time(&time)?);
            }
            if let Some(best_segment) = split.best_segment {
                segment.set_best_segment_time(parse_time(&best_segment)?);
            }

            if let Some(best_time) = split.best_time {
                let best_split_time = parse_time(&best_time)?;
                if best_split_time.real_time.is_some() {
                    run.add_attempt_with_index(
                        Time::default(),
                        attempt_history_index,
                        None,
                        None,
                        None,
                    );

                    // Insert a new run that skips to the current split
                    for already_inserted_segment in run.segments_mut() {
                        already_inserted_segment
                            .segment_history_mut()
                            .insert(attempt_history_index, Time::default());
                    }

                    segment
                        .segment_history_mut()
                        .insert(attempt_history_index, best_split_time);

                    attempt_history_index += 1;
                }
            }

            run.push_segment(segment);
        }
    }

    Ok(run)
}
