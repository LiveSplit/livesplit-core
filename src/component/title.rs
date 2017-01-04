use Timer;
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Default)]
pub struct Component {
    icon_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub icon_change: Option<String>,
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

    pub fn state(&mut self, timer: &Timer) -> State {
        let run = timer.run();
        State {
            icon_change: run.game_icon().check_for_change(&mut self.icon_id).map(str::to_owned),
            game: run.game_name().to_string(),
            category: run.category_name().to_string(),
            attempts: run.attempt_count(),
        }
    }
}
