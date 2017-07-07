use {Timer, TimerPhase, comparison};
use serde_json::{to_writer, Result};
use std::io::Write;
use analysis::current_pace;
use time_formatter::{Regular, TimeFormatter, Accuracy};
use std::borrow::Cow;
use layout::editor::settings_description::{SettingsDescription, Field, Value};

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
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

    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn name(&self) -> Cow<str> {
        "Current Pace".into()
    }

    pub fn state(&self, timer: &Timer) -> State {
        let comparison = self.settings
            .comparison_override
            .as_ref()
            .and_then(|c| timer.run().comparisons().find(|&rc| c == rc))
            .unwrap_or_else(|| timer.current_comparison());

        let mut current_pace = current_pace::calculate(timer, comparison);

        let text = match comparison {
            comparison::personal_best::NAME => "Current Pace".into(),
            comparison::best_segments::NAME => "Best Possible Time".into(),
            comparison::worst_segments::NAME => "Worst Possible Time".into(),
            comparison::average_segments::NAME => "Predicted Time".into(),
            comparison => format!("Current Pace ({})", comparison),
        };

        if timer.current_phase() == TimerPhase::NotRunning && text.starts_with("Current Pace") {
            current_pace = None;
        }

        State {
            text,
            time: Regular::with_accuracy(self.settings.accuracy)
                .format(current_pace)
                .to_string(),
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Comparison Override".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new(
                "Accuracy".into(),
                self.settings.accuracy.into(),
            ),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.comparison_override = value.into(),
            1 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
