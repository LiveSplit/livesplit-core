use Timer;
use serde_json::{to_writer, Result};
use std::io::Write;
use std::borrow::Cow;
use layout::editor::settings_description::{SettingsDescription, Field, Value};

#[derive(Default, Clone)]
pub struct Component {
    icon_id: usize,
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub show_attempt_count: bool,
    pub show_finished_runs_count: bool,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub icon_change: Option<String>,
    pub game: String,
    pub category: String,
    pub finished_runs: Option<u32>,
    pub attempts: Option<u32>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_finished_runs_count: false,
            show_attempt_count: true,
        }
    }
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
        "Title".into()
    }

    pub fn state(&mut self, timer: &Timer) -> State {
        let run = timer.run();

        let finished_runs = if self.settings.show_finished_runs_count {
            Some(timer
                .run()
                .attempt_history()
                .iter()
                .filter(|a| a.time().real_time.is_some())
                .count() as u32)
        } else {
            None
        };

        let attempts = if self.settings.show_attempt_count {
            Some(run.attempt_count())
        } else {
            None
        };

        State {
            icon_change: run.game_icon()
                .check_for_change(&mut self.icon_id)
                .map(str::to_owned),
            game: run.game_name().to_string(),
            category: run.extended_category_name(false, false, true).into_owned(),
            finished_runs,
            attempts,
        }
    }

    pub fn remount(&mut self) {
        self.icon_id = 0;
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Show Finished Runs Count".into(),
                self.settings.show_finished_runs_count.into(),
            ),
            Field::new(
                "Show Attempt Count".into(),
                self.settings.show_attempt_count.into(),
            ),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.show_finished_runs_count = value.into(),
            1 => self.settings.show_attempt_count = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
