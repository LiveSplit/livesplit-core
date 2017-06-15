use Timer;
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Default)]
pub struct Component;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub text: String,
    pub comparison: String,
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

    pub fn state(&self, timer: &Timer) -> State {
        State {
            text: String::from("Comparing Against"),
            comparison: timer.current_comparison().to_string(),
        }
    }
}
