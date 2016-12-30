use Timer;
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Default)]
pub struct Component;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub game: String,
    pub category: String,
    pub attempts: u32,
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
        State {
            game: run.game_name().to_string(),
            category: run.category_name().to_string(),
            attempts: run.attempt_count(),
        }
    }
}
