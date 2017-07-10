use {Timer, TimerPhase, comparison};
use analysis::possible_time_save;
use layout::editor::settings_description::{SettingsDescription, Value, Field};
use serde_json::{to_writer, Result};
use std::borrow::Cow;
use std::cmp::max;
use std::fmt::Write as FmtWrite;
use std::io::Write;
use time_formatter::{PossibleTimeSave, TimeFormatter, Accuracy};

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub comparison_override: Option<String>,
    pub total_possible_time_save: bool,
    pub accuracy: Accuracy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            comparison_override: None,
            total_possible_time_save: false,
            accuracy: Accuracy::Hundredths,
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
        self.text(
            self.settings
                .comparison_override
                .as_ref()
                .map(String::as_ref),
        )
    }

    fn text(&self, comparison: Option<&str>) -> Cow<str> {
        let text = if self.settings.total_possible_time_save {
            "Total Possible Time Save"
        } else {
            "Possible Time Save"
        };
        let mut text = Cow::from(text);
        if let Some(comparison) = comparison {
            write!(text.to_mut(), " ({})", comparison::shorten(comparison)).unwrap();
        }
        text
    }

    pub fn state(&self, timer: &Timer) -> State {
        let segment_index = timer.current_split_index();
        let current_phase = timer.current_phase();
        let comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let text = self.text(comparison);
        let comparison = comparison::or_current(comparison, timer);

        let time = if self.settings.total_possible_time_save {
            Some(possible_time_save::calculate_total(
                timer,
                max(0, segment_index) as usize,
                comparison,
            ))
        } else if current_phase == TimerPhase::Running || current_phase == TimerPhase::Paused {
            possible_time_save::calculate(timer, segment_index as usize, comparison, false)
        } else {
            None
        };

        State {
            text: text.into_owned(),
            time: PossibleTimeSave::with_accuracy(self.settings.accuracy)
                .format(time)
                .to_string(),
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Comparison".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new(
                "Show Total Possible Time Save".into(),
                self.settings.total_possible_time_save.into(),
            ),
            Field::new("Accuracy".into(), self.settings.accuracy.into()),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.comparison_override = value.into(),
            1 => self.settings.total_possible_time_save = value.into(),
            2 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
