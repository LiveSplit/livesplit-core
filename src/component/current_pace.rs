use {Timer, comparison};
use serde_json::{to_writer, Result};
use std::io::Write;
use analysis::current_pace;
use time_formatter::{Regular, TimeFormatter, Accuracy};

#[derive(Default)]
pub struct Component {
    settings: Settings,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub comparison_override: Option<String>,
    pub accuracy: Accuracy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            comparison_override: None,
            accuracy: Accuracy::Seconds,
        }
    }
}

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

    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn state(&self, timer: &Timer) -> State {
        let comparison = self.settings
            .comparison_override
            .as_ref()
            .and_then(|c| timer.run().comparisons().find(|&rc| c == rc));

        let current_pace = current_pace::calculate(timer, comparison);

        let text = match comparison {
            None |
            Some(comparison::PERSONAL_BEST_COMPARISON_NAME) => "Current Pace".into(),
            Some(comparison::best_segments::NAME) => "Best Possible Time".into(),
            Some(comparison::worst_segments::NAME) => "Worst Possible Time".into(),
            Some(comparison::average_segments::NAME) => "Predicted Time".into(),
            Some(comparison) => format!("Current Pace ({})", comparison),
        };

        State {
            text,
            time: Regular::with_accuracy(self.settings.accuracy)
                .format(current_pace)
                .to_string(),
        }
    }
}
