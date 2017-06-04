use Timer;
use serde_json::{to_writer, Result};
use std::io::Write;
use analysis::calculate_total_playtime;
use time_formatter::{Days, TimeFormatter};

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
        let total_playtime = calculate_total_playtime(timer);

        State {
            text: String::from("Total Playtime"),
            time: Days::new().format(total_playtime).to_string(),
        }
    }
}
