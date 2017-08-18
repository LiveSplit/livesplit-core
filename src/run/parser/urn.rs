use std::io::Read;
use std::result::Result as StdResult;
use serde_json::de::from_reader;
use serde_json::Error as JsonError;
use {time, Run, Segment, Time, TimeSpan};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Time(err: time::ParseError) {
            from()
        }
        Json(err: JsonError) {
            from()
        }
    }
}

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

pub fn parse<R: Read>(source: R) -> Result<Run> {
    let mut run = Run::new();

    let splits: Splits = from_reader(source)?;

    if let Some(title) = splits.title {
        run.category_name = title;
    }
    if let Some(attempt_count) = splits.attempt_count {
        run.attempt_count = attempt_count;
    }
    if let Some(start_delay) = splits.start_delay {
        run.offset = -start_delay.parse()?;
    }

    // Best Split Times can be used for the Segment History
    // Every single best split time should be included as its own run,
    // since the best split times could be apart from each other less
    // than the best segments, so we have to assume they are from different runs.
    let mut attempt_history_index = 1;

    if let Some(splits) = splits.splits {
        for split in splits {
            let mut segment = Segment::new(split.title.unwrap_or_default());
            if let Some(time) = split.time {
                segment.set_personal_best_split_time(parse_time(&time)?);
            }
            if let Some(best_segment) = split.best_segment {
                segment.best_segment_time = parse_time(&best_segment)?;
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
                    for already_inserted_segment in &mut run.segments {
                        already_inserted_segment
                            .segment_history
                            .insert(attempt_history_index, Time::default());
                    }

                    segment
                        .segment_history
                        .insert(attempt_history_index, best_split_time);

                    attempt_history_index += 1;
                }
            }

            run.segments.push(segment);
        }
    }

    Ok(run)
}
