use Timer;
use time_formatter::{Regular, TimeFormatter};
use serde_json::{to_writer, Result};
use time_formatter::Accuracy;
use sum_of_segments::calculate_best;
use std::io::Write;

#[derive(new, Default)]
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
    pub fn state(&self, timer: &Timer) -> State {
        let time = calculate_best(timer.run().segments(),
                                  false,
                                  true,
                                  timer.current_timing_method());

        State {
            text: String::from("Sum of Best Segments"),
            time: Regular::with_accuracy(Accuracy::Seconds)
                .format(time)
                .to_string(),
        }
    }
}
