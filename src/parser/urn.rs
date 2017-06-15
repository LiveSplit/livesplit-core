use std::io::Read;
use std::result::Result as StdResult;
use serde_json::de::from_reader;
use serde_json::Error as JsonError;
use time_span::ParseError as TimeSpanError;
use {Run, TimeSpan, Time, Segment};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Time(err: TimeSpanError) {
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
    title: String,
    attempt_count: u32,
    start_delay: String,
    splits: Vec<Split>,
}

#[derive(Deserialize)]
struct Split {
    title: String,
    time: String,
    best_time: String,
    best_segment: String,
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

    run.set_category_name(splits.title);
    run.set_attempt_count(splits.attempt_count);
    run.set_offset(-splits.start_delay.parse()?);

    // Best Split Times can be used for the Segment History
    // Every single best split time should be included as its own run,
    // since the best split times could be apart from each other less
    // than the best segments, so we have to assume they are from different runs.
    let mut attempt_history_index = 1;

    for split in splits.splits {
        let mut segment = Segment::new(split.title);
        segment.set_personal_best_split_time(parse_time(&split.time)?);
        segment.set_best_segment_time(parse_time(&split.best_segment)?);

        let best_split_time = parse_time(&split.best_time)?;
        if best_split_time.real_time.is_some() {
            run.add_attempt_with_index(Time::default(), attempt_history_index, None, None, None);

            // Insert a new run that skips to the current split
            for already_inserted_segment in run.segments_mut() {
                already_inserted_segment.segment_history_mut().insert(
                    attempt_history_index,
                    Time::default(),
                );
            }

            segment.segment_history_mut().insert(
                attempt_history_index,
                best_split_time,
            );

            attempt_history_index += 1;
        }

        run.push_segment(segment);
    }

    Ok(run)
}
