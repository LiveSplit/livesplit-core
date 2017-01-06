use {state_helper, Timer, TimeSpan, Segment, TimingMethod};
use time_formatter::{Regular, TimeFormatter};
use comparison::best_segments::COMPARISON_NAME;
use time_formatter::Accuracy;
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Default)]
pub struct Component;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub text: String,
    pub time: String,
}

impl State {
    pub fn write_json<W>(&self, mut writer: W) -> Result<()>
        where W: Write
    {
        to_writer(&mut writer, self)
    }
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn state(&self, timer: &Timer) -> State {
        let live = false;

        let segment = timer.current_split();
        let mut time: Option<TimeSpan> = Some(TimeSpan::zero());

        if segment.is_some() {
            time = get_possible_time_save(&timer, &segment.unwrap(), COMPARISON_NAME, live);
        }

        State {
            text: "Possible Time Save".to_string(),
            time: Regular::with_accuracy(Accuracy::Tenths).format(time.unwrap()).to_string(),
        }
    }
}

pub fn get_possible_time_save(timer: &Timer,
                              segment: &Segment,
                              comparison: &str,
                              live: bool)
                              -> Option<TimeSpan> {
    let segments = timer.run().segments();
    let mut split_index = split_index(&timer, &segment);
    let mut prev_time = TimeSpan::zero();
    let mut best_segments = segment.best_segment_time().real_time;
    let pb_split_time = segment.personal_best_split_time().real_time;

    while split_index > 0 && best_segments != None {

        let split_time = segments[split_index - 1].personal_best_split_time().real_time;

        if split_time.is_some() {
            prev_time = split_time.unwrap();
            break;
        } else {
            split_index -= 1;
            if segments[split_index].best_segment_time().real_time.is_some() {
                best_segments = Some(best_segments.unwrap() +
                                     segments[split_index].best_segment_time().real_time.unwrap());
            }
        }
    }

    let mut time = TimeSpan::zero();
    if pb_split_time.is_some() {
        time = pb_split_time.unwrap();
    }

    time = time - prev_time;
    if best_segments.is_some() {
        time = time - best_segments.unwrap();
    }

    if live && split_index == timer.current_split_index() as usize {
        let live_delta = state_helper::live_segment_delta(&timer,
                                                          split_index,
                                                          comparison,
                                                          TimingMethod::RealTime);

        let mut segment_delta = TimeSpan::zero();
        if live_delta.is_some() {
            segment_delta = TimeSpan::zero() - live_delta.unwrap();
        }
        if segment_delta < time {
            time = segment_delta;
        }
    }

    if time < TimeSpan::zero() {
        time = TimeSpan::zero();
    }

    Some(time)
}

fn split_index(timer: &Timer, segment: &Segment) -> usize {
    let segments = timer.run().segments();
    let index = segments.iter().position(|s| s.name() == segment.name()).unwrap();
    index
}