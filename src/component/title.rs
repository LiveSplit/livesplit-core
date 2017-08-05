use Timer;
use serde_json::{to_writer, Result};
use std::io::Write;
use std::borrow::Cow;
use settings::{Color, Field, Gradient, SettingsDescription, Value};

#[derive(Default, Clone)]
pub struct Component {
    icon_id: usize,
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub background: Gradient,
    pub show_game_name: bool,
    pub show_category_name: bool,
    pub show_finished_runs_count: bool,
    pub show_attempt_count: bool,
    pub center_text: bool,
    pub display_as_single_line: bool,
    pub display_game_icon: bool,
    pub show_region: bool,
    pub show_platform: bool,
    pub show_variables: bool,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub background: Gradient,
    pub icon_change: Option<String>,
    pub line1: String,
    pub line2: Option<String>,
    pub is_centered: bool,
    pub finished_runs: Option<u32>,
    pub attempts: Option<u32>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: Gradient::Vertical(
                Color::hsla(0.0, 0.0, 1.0, 0.13),
                Color::hsla(0.0, 0.0, 1.0, 0.0),
            ),
            show_game_name: true,
            show_category_name: true,
            show_finished_runs_count: false,
            show_attempt_count: true,
            center_text: false,
            display_as_single_line: false,
            display_game_icon: true,
            show_region: false,
            show_platform: false,
            show_variables: true,
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

        let icon_change = if self.settings.display_game_icon {
            run.game_icon()
                .check_for_change(&mut self.icon_id)
                .map(str::to_owned)
        } else if self.icon_id != 0 {
            self.icon_id = 0;
            Some(String::new())
        } else {
            None
        };

        let is_centered = self.settings.center_text || run.game_icon().is_empty() ||
            !self.settings.display_game_icon;

        let game_name = if self.settings.show_game_name {
            run.game_name()
        } else {
            ""
        };

        let category_name = if self.settings.show_category_name {
            run.extended_category_name(
                self.settings.show_region,
                self.settings.show_platform,
                self.settings.show_variables,
            )
        } else {
            "".into()
        };

        let (line1, line2) = if self.settings.display_as_single_line {
            let mut line1 = String::with_capacity(game_name.len() + category_name.len() + 3);
            line1.push_str(game_name);
            if !game_name.is_empty() && !category_name.is_empty() {
                line1.push_str(" - ");
            }
            line1.push_str(&*category_name);
            (line1, None)
        } else {
            match (!game_name.is_empty(), !category_name.is_empty()) {
                (true, true) => (game_name.to_owned(), Some(category_name.into_owned())),
                (true, false) => (game_name.to_owned(), None),
                (false, true) => (category_name.into_owned(), None),
                (false, false) => (String::new(), None),
            }
        };

        State {
            background: self.settings.background,
            icon_change,
            finished_runs,
            attempts,
            is_centered,
            line1,
            line2,
        }
    }

    pub fn remount(&mut self) {
        self.icon_id = 0;
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new("Show Game Name".into(), self.settings.show_game_name.into()),
            Field::new(
                "Show Category Name".into(),
                self.settings.show_category_name.into(),
            ),
            Field::new(
                "Show Finished Runs Count".into(),
                self.settings.show_finished_runs_count.into(),
            ),
            Field::new(
                "Show Attempt Count".into(),
                self.settings.show_attempt_count.into(),
            ),
            Field::new("Center Text".into(), self.settings.center_text.into()),
            Field::new(
                "Display Text as Single Line".into(),
                self.settings.display_as_single_line.into(),
            ),
            Field::new(
                "Display Game Icon".into(),
                self.settings.display_game_icon.into(),
            ),
            Field::new("Show Region".into(), self.settings.show_region.into()),
            Field::new("Show Platform".into(), self.settings.show_platform.into()),
            Field::new("Show Variables".into(), self.settings.show_variables.into()),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.background = value.into(),
            1 => self.settings.show_game_name = value.into(),
            2 => self.settings.show_category_name = value.into(),
            3 => self.settings.show_finished_runs_count = value.into(),
            4 => self.settings.show_attempt_count = value.into(),
            5 => self.settings.center_text = value.into(),
            6 => self.settings.display_as_single_line = value.into(),
            7 => self.settings.display_game_icon = value.into(),
            8 => self.settings.show_region = value.into(),
            9 => self.settings.show_platform = value.into(),
            10 => self.settings.show_variables = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
