//! Provides the Title Component and relevant types for using it. The Title
//! Component is a component that shows the name of the game and the category
//! that is being run. Additionally, the game icon, the attempt count, and the
//! total number of finished runs can be shown.

use crate::platform::prelude::*;
use crate::settings::{Alignment, Color, Field, Gradient, SettingsDescription, Value};
use crate::{
    settings::{CachedImageId, Image, ImageData},
    Timer, TimerPhase,
};
use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// The Title Component is a component that shows the name of the game and the
/// category that is being run. Additionally, the game icon, the attempt count,
/// and the total number of finished runs can be shown.
#[derive(Default, Clone)]
pub struct Component {
    icon_id: CachedImageId,
    settings: Settings,
}

/// The Settings for this component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The background shown behind the component.
    pub background: Gradient,
    /// The color of the title text. If `None` is specified, the color is taken
    /// from the layout.
    pub text_color: Option<Color>,
    /// Specifies whether the game name should be part of the title that is
    /// being shown.
    pub show_game_name: bool,
    /// Specifies whether the category name should be part of the title that is
    /// being shown.
    pub show_category_name: bool,
    /// Specifies whether the amount of successfully finished attempts should be
    /// shown.
    pub show_finished_runs_count: bool,
    /// Specifies whether the total amount of attempts should be shown.
    pub show_attempt_count: bool,
    /// Specifies the alignment of the title.
    pub text_alignment: Alignment,
    /// Specifies if the title should be shown as a single line, instead of
    /// being separated into one line for the game name and one for the category
    /// name.
    pub display_as_single_line: bool,
    /// Specifies whether the game's icon should be shown, in case there is a
    /// game icon stored in the Run.
    pub display_game_icon: bool,
    /// The category name can be extended by additional information. This
    /// extends it by the game's region, if it is provided by the run's
    /// metadata.
    pub show_region: bool,
    /// The category name can be extended by additional information. This
    /// extends it by the platform the game is being played on, if it is
    /// provided by the run's metadata.
    pub show_platform: bool,
    /// The category name can be extended by additional information. This
    /// extends it by additional variables provided by the run's metadata.
    pub show_variables: bool,
}

/// The state object describes the information to visualize for this component.
#[derive(Serialize, Deserialize)]
pub struct State {
    /// The background shown behind the component.
    pub background: Gradient,
    /// The color of the text. If `None` is specified, the color is taken from
    /// the layout.
    pub text_color: Option<Color>,
    /// The game's icon encoded as the raw file bytes. This value is only
    /// specified whenever the icon changes. If you explicitly want to query
    /// this value, remount the component. The buffer itself may be empty. This
    /// indicates that there is no icon.
    pub icon_change: Option<ImageData>,
    /// The first title line to show. This is either the game's name, or a
    /// combination of the game's name and the category. This is a list of all
    /// the possible abbreviations. It contains at least one element and the
    /// last element is the unabbreviated value.
    pub line1: Box<[Box<str>]>,
    /// By default the category name is shown on the second line. Based on the
    /// settings, it can however instead be shown in a single line together with
    /// the game name. This is a list of all the possible abbreviations. It
    /// contains at least one element and the last element is the unabbreviated
    /// value.
    pub line2: Option<Box<[Box<str>]>>,
    /// Specifies whether the title should centered or aligned to the left
    /// instead.
    pub is_centered: bool,
    /// The amount of successfully finished attempts. If `None` is specified,
    /// the amount of successfully finished attempts isn't supposed to be shown.
    pub finished_runs: Option<u32>,
    /// The amount of total attempts. If `None` is specified, the amount of
    /// total attempts isn't supposed to be shown.
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

#[cfg(feature = "std")]
impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }
}

impl Component {
    /// Creates a new Title Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Title Component with the given settings.
    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    /// Accesses the settings of the component.
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Grants mutable access to the settings of the component.
    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Accesses the name of the component.
    pub fn name(&self) -> &'static str {
        "Title"
    }

    /// Calculates the component's state based on the timer provided.
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

        let game_icon = Some(run.game_icon()).filter(|_| self.settings.display_game_icon);
        let icon_change = self.icon_id.update_with(game_icon).map(Into::into);

        let is_centered = match self.settings.text_alignment {
            Alignment::Center => true,
            Alignment::Left => false,
            Alignment::Auto => game_icon.map_or(true, Image::is_empty),
        };

        let game_abbrevs: Box<[Box<str>]> = if self.settings.show_game_name {
            livesplit_title_abbreviations::abbreviate(run.game_name()).into()
        } else {
            Default::default()
        };

        let full_category_name = run.extended_category_name(
            self.settings.show_region,
            self.settings.show_platform,
            self.settings.show_variables,
        );

        let (line1, line2) = if self.settings.display_as_single_line {
            let mut abbrevs = Vec::new();
            let mut abbrev = String::new();

            if !full_category_name.is_empty() {
                for game_abbrev in game_abbrevs.iter() {
                    abbrev.clear();
                    abbrev.push_str(game_abbrev);
                    if !game_abbrev.is_empty() {
                        abbrev.push_str(" - ");
                    }
                    abbrev.push_str(&full_category_name);
                    abbrevs.push(abbrev.as_str().into());
                }
            }
            // This assumes the last element is the unabbreviated value, which
            // can only be the case if the `game_abbrevs` also has the
            // unabbreviated game name as its last element.
            let swap_index = abbrevs.len().checked_sub(1);

            if let Some(shortest_game_name) = game_abbrevs.iter().min_by_key(|a| a.len()) {
                abbrev.clear();
                abbrev.push_str(shortest_game_name);
                let game_len = abbrev.len();
                for category_abbrev in abbreviate_category(full_category_name).iter() {
                    if !shortest_game_name.is_empty() && !category_abbrev.is_empty() {
                        abbrev.push_str(" - ");
                    }
                    abbrev.push_str(&*category_abbrev);
                    abbrevs.push(abbrev.as_str().into());
                    abbrev.drain(game_len..);
                }
            }

            // We want to ensure the "unabbreviated value" is at the end. This
            // is something we guarantee at least at the moment.
            if let Some(swap_index) = swap_index {
                let last_element_idx = abbrevs.len() - 1;
                abbrevs.swap(swap_index, last_element_idx);
            }

            (abbrevs.into(), None)
        } else {
            let category_abbrevs: Box<[_]> = if self.settings.show_category_name {
                abbreviate_category(full_category_name).into()
            } else {
                Default::default()
            };

            match (
                game_abbrevs.last().map_or(false, |g| !g.is_empty()),
                category_abbrevs.last().map_or(false, |c| !c.is_empty()),
            ) {
                (true, true) => (game_abbrevs, Some(category_abbrevs)),
                (true, false) => (game_abbrevs, None),
                (false, true) => (category_abbrevs, None),
                (false, false) => (vec!["Untitled".into()].into(), None),
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

    /// Remounts the component as if it was freshly initialized. The game icon
    /// shown by this component is only provided in the state objects whenever
    /// the icon changes or whenever the component's state is first queried.
    /// Remounting returns the game icon again, whenever its state is queried
    /// the next time.
    pub fn remount(&mut self) {
        self.icon_id.reset();
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
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

    /// Sets a setting's value by its index to the given value.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
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

// FIXME: Turn this into a generator once that works on stable Rust.
fn abbreviate_category(category: Cow<'_, str>) -> Vec<Box<str>> {
    let mut abbrevs = Vec::new();

    let mut splits = category.splitn(2, '(');
    let before = splits.next().unwrap().trim();

    if let Some(rest) = splits.next() {
        splits = rest.splitn(2, ')');
        let inside = splits.next().unwrap();
        if let Some(after) = splits.next() {
            let after = after.trim_end();

            let mut buf = String::with_capacity(category.len());
            buf.push_str(before);
            buf.push_str(" (");

            let mut splits = inside.split(',');
            let mut variable = splits.next().unwrap();
            for next_variable in splits {
                buf.push_str(variable);
                let old_len = buf.len();

                buf.push_str(")");
                buf.push_str(after);
                abbrevs.push(buf.as_str().into());

                buf.drain(old_len..);
                buf.push_str(",");
                variable = next_variable;
            }

            if after.trim().is_empty() {
                buf.drain(before.len()..);
            } else {
                buf.drain(before.len() + 1..);
                buf.push_str(after);
            }

            abbrevs.push(buf.into());
        }
    }

    abbrevs.push(category.into_owned().into());

    abbrevs
}
