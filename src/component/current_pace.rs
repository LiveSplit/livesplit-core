use {comparison, Timer, TimerPhase};
use serde_json::{to_writer, Result};
use std::io::Write;
use analysis::current_pace;
use time::formatter::{Accuracy, Regular, TimeFormatter};
use std::borrow::Cow;
use settings::{Field, Gradient, SettingsDescription, Value};
use super::DEFAULT_INFO_TEXT_GRADIENT;

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub background: Gradient,
    pub comparison_override: Option<String>,
    pub accuracy: Accuracy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: DEFAULT_INFO_TEXT_GRADIENT,
            comparison_override: None,
            accuracy: Accuracy::Seconds,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub background: Gradient,
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
        self.text(
            self.settings
                .comparison_override
                .as_ref()
                .map(String::as_str),
        )
    }

    fn text(&self, comparison: Option<&str>) -> Cow<str> {
        if let Some(comparison) = comparison {
            match comparison {
                comparison::personal_best::NAME => "Current Pace".into(),
                comparison::best_segments::NAME => "Best Possible Time".into(),
                comparison::worst_segments::NAME => "Worst Possible Time".into(),
                comparison::average_segments::NAME => "Predicted Time".into(),
                comparison => format!("Current Pace ({})", comparison::shorten(comparison)).into(),
            }
        } else {
            "Current Pace".into()
        }
    }

    pub fn state(&self, timer: &Timer) -> State {
        let comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(comparison, timer);
        let text = self.text(Some(comparison)).into_owned();

        let current_pace =
            if timer.current_phase() == TimerPhase::NotRunning && text.starts_with("Current Pace") {
                None
            } else {
                current_pace::calculate(timer, comparison)
            };

        State {
            background: self.settings.background,
            text,
            time: Regular::with_accuracy(self.settings.accuracy)
                .format(current_pace)
                .to_string(),
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new(
                "Comparison".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new("Accuracy".into(), self.settings.accuracy.into()),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.background = value.into(),
            1 => self.settings.comparison_override = value.into(),
            2 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
