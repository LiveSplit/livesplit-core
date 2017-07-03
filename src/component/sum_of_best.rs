use Timer;
use time_formatter::{Regular, TimeFormatter, Accuracy};
use serde_json::{to_writer, Result};
use analysis::sum_of_segments::calculate_best;
use std::io::Write;
use std::borrow::Cow;
use layout::editor::settings_description::{SettingsDescription, Value};

#[derive(Default, Clone)]
pub struct Component;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub text: String,
    pub time: String,
}

impl State {
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn name(&self) -> Cow<str> {
        "Sum of Best".into()
    }

    pub fn state(&self, timer: &Timer) -> State {
        let time = calculate_best(
            timer.run().segments(),
            false,
            true,
            timer.current_timing_method(),
        );

        State {
            text: String::from("Sum of Best Segments"),
            time: Regular::with_accuracy(Accuracy::Seconds)
                .format(time)
                .to_string(),
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::default()
    }

    pub fn set_value(&mut self, _index: usize, _value: Value) {}
}
