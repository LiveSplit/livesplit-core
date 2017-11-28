use {Timer, TimerPhase};
use serde_json::{to_writer, Result};
use std::io::Write;
use std::borrow::Cow;
use settings::{Alignment, Color, Field, Gradient, SettingsDescription, Value};

#[cfg(test)]
mod tests;

#[derive(Default, Clone)]
pub struct Component {
    icon_id: usize,
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub background: Gradient,
    pub text_color: Option<Color>,
    pub show_game_name: bool,
    pub show_category_name: bool,
    pub show_finished_runs_count: bool,
    pub show_attempt_count: bool,
    pub text_alignment: Alignment,
    pub display_as_single_line: bool,
    pub display_game_icon: bool,
    pub show_region: bool,
    pub show_platform: bool,
    pub show_variables: bool,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub background: Gradient,
    pub text_color: Option<Color>,
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
            text_color: None,
            show_game_name: true,
            show_category_name: true,
            show_finished_runs_count: false,
            show_attempt_count: true,
            text_alignment: Alignment::Auto,
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
            let mut count = timer
                .run()
                .attempt_history()
                .iter()
                .filter(|a| a.time().real_time.is_some())
                .count() as u32;

            if timer.current_phase() == TimerPhase::Ended {
                count += 1;
            }

            Some(count)
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

        let is_centered = match self.settings.text_alignment {
            Alignment::Center => true,
            Alignment::Left => false,
            Alignment::Auto => run.game_icon().is_empty() || !self.settings.display_game_icon,
        };

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
            text_color: self.settings.text_color,
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
            Field::new("Text Color".into(), self.settings.text_color.into()),
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
            Field::new("Text Alignment".into(), self.settings.text_alignment.into()),
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
            1 => self.settings.text_color = value.into(),
            2 => self.settings.show_game_name = value.into(),
            3 => self.settings.show_category_name = value.into(),
            4 => self.settings.show_finished_runs_count = value.into(),
            5 => self.settings.show_attempt_count = value.into(),
            6 => self.settings.text_alignment = value.into(),
            7 => self.settings.display_as_single_line = value.into(),
            8 => self.settings.display_game_icon = value.into(),
            9 => self.settings.show_region = value.into(),
            10 => self.settings.show_platform = value.into(),
            11 => self.settings.show_variables = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
