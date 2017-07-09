use {Timer, TimerPhase};
use time_formatter::{PossibleTimeSave, TimeFormatter};
use serde_json::{to_writer, Result};
use std::io::Write;
use std::borrow::Cow;
use layout::editor::settings_description::{SettingsDescription, Value, Field};
use analysis::possible_time_save;
use std::cmp::max;

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub total_possible_time_save: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            total_possible_time_save: false,
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
        "Possible Time Save".into()
    }

    pub fn state(&self, timer: &Timer) -> State {
        let segment_index = timer.current_split_index();
        let current_phase = timer.current_phase();
        let comparison = timer.current_comparison();

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

        let text = if self.settings.total_possible_time_save {
            "Total Possible Time Save"
        } else {
            "Possible Time Save"
        };

        State {
            text: text.to_string(),
            time: PossibleTimeSave::new().format(time).to_string(),
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Show Total Possible Time Save".into(),
                self.settings.total_possible_time_save.into(),
            ),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.total_possible_time_save = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
