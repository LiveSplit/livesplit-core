use {Timer, Time, TimeSpan, TimingMethod};
use time_formatter::{Regular, TimeFormatter};
use serde_json::{to_writer, Result};
use time_formatter::Accuracy;
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
        let run = timer.run();
        let mut time: Time = Time::zero();
        let mut best_time = TimeSpan::zero();

        for split in run.segments() {
            if split.best_segment_time().game_time != None {
                time.game_time = Some(time.game_time.unwrap() +
                                      split.best_segment_time().game_time.unwrap());
            }
            if split.best_segment_time().real_time != None {
                time.real_time = Some(time.real_time.unwrap() +
                                      split.best_segment_time().real_time.unwrap());
            }
        }

        if timer.current_timing_method() == TimingMethod::GameTime {
            if time.game_time != None {
                best_time = time.game_time.unwrap();
            }
        } else {
            if time.real_time != None {
                best_time = time.real_time.unwrap();
            }
        }

        State {
            text: String::from("Sum of Best"),
            time: Regular::with_accuracy(Accuracy::Tenths).format(best_time).to_string(),
        }
    }
}