use Timer;
use serde_json::{to_writer, Result};
use std::io::Write;
use analysis::total_playtime;
use time_formatter::{Days, TimeFormatter};
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
        "Total Playtime".into()
    }

    pub fn state(&self, timer: &Timer) -> State {
        let total_playtime = total_playtime::calculate(timer);

        State {
            text: String::from("Total Playtime"),
            time: Days::new().format(total_playtime).to_string(),
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::default()
    }

    pub fn set_value(&mut self, _index: usize, _value: Value) {}
}
