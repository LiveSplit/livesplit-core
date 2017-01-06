use std::io::Read;
use std::result::Result as StdResult;
use serde_json::de::from_reader;
use serde_json::Error as JsonError;
use {Run, TimeSpan, Time, TimingMethod, Segment};

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Json(err: JsonError) {
            from()
        }
    }
}

pub type Result<T> = StdResult<T, Error>;

#[derive(Deserialize)]
struct Splits {
    run_name: String,
    start_delay: f64,
    run_count: u32,
    splits: Vec<Split>,
    timer_type: u8,
}

#[derive(Deserialize)]
struct Split {
    name: String,
    pb_split: Option<f64>,
    split_best: Option<f64>,
}

fn parse_time(milliseconds: Option<f64>, method: TimingMethod) -> Time {
    let mut time = Time::new();

    if let Some(milliseconds) = milliseconds {
        time[method] = Some(TimeSpan::from_milliseconds(milliseconds));
    }

    time
}

pub fn parse<R: Read>(source: R) -> Result<Run> {
    let mut run = Run::new(Vec::new());

    let splits: Splits = from_reader(source)?;

    run.set_game_name(splits.run_name);
    run.set_attempt_count(splits.run_count);
    run.set_offset(TimeSpan::from_milliseconds(-splits.start_delay));

    let method = if splits.timer_type == 0 {
        TimingMethod::RealTime
    } else {
        TimingMethod::GameTime
    };

    for split in splits.splits {
        let mut segment = Segment::new(split.name);
        segment.set_personal_best_split_time(parse_time(split.pb_split, method));
        segment.set_best_segment_time(parse_time(split.split_best, method));

        run.push_segment(segment);
    }

    Ok(run)
}
