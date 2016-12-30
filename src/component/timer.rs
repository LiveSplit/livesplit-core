use Timer;
use time_formatter::timer as formatter;
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Default)]
pub struct Component;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub time: String,
    pub fraction: String,
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
        let time = timer.current_time();
        let time = time.real_time.unwrap();
        State {
            time: formatter::Time::new(time).to_string(),
            fraction: formatter::Fraction::new(time).to_string(),
        }
    }
}
