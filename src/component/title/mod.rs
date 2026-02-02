//! Provides the Title Component and relevant types for using it. The Title
//! Component is a component that shows the name of the game and the category
//! that is being run. Additionally, the game icon, the attempt count, and the
//! total number of finished runs can be shown.

use crate::{
    Timer, TimerPhase,
    localization::{Lang, Text},
    platform::prelude::*,
    settings::{
        Alignment, Color, Field, Gradient, Image, ImageCache, ImageId, SettingsDescription, Value,
    },
};
use core::fmt::Write;
use livesplit_title_abbreviations::{abbreviate as abbreviate_title, abbreviate_category};
use serde_derive::{Deserialize, Serialize};
use smallstr::SmallString;

#[cfg(test)]
mod tests;

/// The Title Component is a component that shows the name of the game and the
/// category that is being run. Additionally, the game icon, the attempt count,
/// and the total number of finished runs can be shown.
#[derive(Default, Clone)]
pub struct Component {
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
#[derive(Default, Serialize, Deserialize)]
pub struct State {
    /// The background shown behind the component.
    pub background: Gradient,
    /// The color of the text. If `None` is specified, the color is taken from
    /// the layout.
    pub text_color: Option<Color>,
    /// The game icon to show. The associated image can be looked up in the
    /// image cache. The image may be the empty image. This indicates that there
    /// is no icon.
    pub icon: ImageId,
    /// The first title line to show. This is either the game's name, or a
    /// combination of the game's name and the category. This is a list of all
    /// the possible abbreviations. It contains at least one element and the
    /// last element is the unabbreviated value.
    pub line1: Vec<Box<str>>,
    /// By default the category name is shown on the second line. Based on the
    /// settings, it can however instead be shown in a single line together with
    /// the game name. This is a list of all the possible abbreviations. If this
    /// is empty, only a single line is supposed to be shown. If it contains at
    /// least one element, the last element is the unabbreviated value.
    pub line2: Vec<Box<str>>,
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
    pub const fn with_settings(settings: Settings) -> Self {
        Self { settings }
    }

    /// Accesses the settings of the component.
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Grants mutable access to the settings of the component.
    pub const fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Accesses the name of the component for the specified language.
    pub const fn name(&self, lang: Lang) -> &'static str {
        Text::ComponentTitle.resolve(lang)
    }

    /// Updates the component's state based on the timer provided. The
    /// [`ImageCache`] is updated with all the images that are part of the
    /// state. The images are marked as visited in the [`ImageCache`]. You still
    /// need to manually run [`ImageCache::collect`] to ensure unused images are
    /// removed from the cache.
    pub fn update_state(&self, state: &mut State, image_cache: &mut ImageCache, timer: &Timer) {
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

        let icon = if self.settings.display_game_icon {
            run.game_icon()
        } else {
            Image::EMPTY
        };
        state.icon = *image_cache.cache(icon.id(), || icon.clone()).id();

        let is_centered = match self.settings.text_alignment {
            Alignment::Center => true,
            Alignment::Left => false,
            Alignment::Auto => state.icon.is_empty(),
        };

        let game_name = if self.settings.show_game_name {
            run.game_name()
        } else {
            ""
        };

        let mut full_category_name = SmallString::<[u8; 1024]>::new();
        let _ = write!(
            full_category_name,
            "{}",
            run.extended_category_name(
                self.settings.show_region,
                self.settings.show_platform,
                self.settings.show_variables,
            ),
        );

        let full_category_name = if self.settings.show_category_name {
            &full_category_name
        } else {
            ""
        };

        match (!game_name.is_empty(), !full_category_name.is_empty()) {
            (true, true) => {
                if self.settings.display_as_single_line {
                    let unchanged = catch! {
                        let mut rem = &**state.line1.last()?;

                        rem = rem.strip_prefix(game_name)?;

                        if !game_name.is_empty() && !full_category_name.is_empty() {
                            rem = rem.strip_prefix(" - ")?;
                        }

                        if rem != full_category_name {
                            return None;
                        }
                    };
                    if unchanged.is_none() {
                        let abbrevs = &mut state.line1;
                        abbrevs.clear();

                        let mut abbrev = String::new();
                        let game_abbrevs = abbreviate_title(game_name);
                        let category_abbrevs = abbreviate_category(full_category_name);

                        if !full_category_name.is_empty() {
                            for game_abbrev in game_abbrevs.iter() {
                                abbrev.clear();
                                abbrev.push_str(game_abbrev);
                                if !game_abbrev.is_empty() {
                                    abbrev.push_str(" - ");
                                }
                                abbrev.push_str(full_category_name);
                                abbrevs.push(abbrev.as_str().into());
                            }
                        }
                        // This assumes the last element is the unabbreviated value, which
                        // can only be the case if the `game_abbrevs` also has the
                        // unabbreviated game name as its last element.
                        let swap_index = abbrevs.len().checked_sub(1);

                        if let Some(shortest_game_name) =
                            game_abbrevs.iter().min_by_key(|a| a.len())
                        {
                            abbrev.clear();
                            abbrev.push_str(shortest_game_name);
                            let game_len = abbrev.len();
                            for category_abbrev in category_abbrevs.iter() {
                                if !shortest_game_name.is_empty() && !category_abbrev.is_empty() {
                                    abbrev.push_str(" - ");
                                }
                                abbrev.push_str(category_abbrev);
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
                    }
                    state.line2.clear();
                } else {
                    if state.line1.last().is_none_or(|g| game_name != &**g) {
                        state.line1.clear();
                        state.line1.extend(abbreviate_title(game_name));
                    }
                    if state
                        .line2
                        .last()
                        .is_none_or(|c| full_category_name != &**c)
                    {
                        state.line2.clear();
                        state.line2.extend(abbreviate_category(full_category_name));
                    }
                }
            }
            (true, false) => {
                if state.line1.last().is_none_or(|g| game_name != &**g) {
                    state.line1.clear();
                    state.line1.extend(abbreviate_title(game_name));
                }
                state.line2.clear();
            }
            (false, true) => {
                if state
                    .line1
                    .last()
                    .is_none_or(|c| full_category_name != &**c)
                {
                    state.line1.clear();
                    state.line1.extend(abbreviate_category(full_category_name));
                }
                state.line2.clear();
            }
            (false, false) => {
                state.line1.clear();
                state.line1.push("Untitled".into());
                state.line2.clear();
            }
        }

        state.background = self.settings.background;
        state.text_color = self.settings.text_color;
        state.finished_runs = finished_runs;
        state.attempts = attempts;
        state.is_centered = is_centered;
    }

    /// Calculates the component's state based on the timer provided. The
    /// [`ImageCache`] is updated with all the images that are part of the
    /// state. The images are marked as visited in the [`ImageCache`]. You still
    /// need to manually run [`ImageCache::collect`] to ensure unused images are
    /// removed from the cache.
    pub fn state(&self, image_cache: &mut ImageCache, timer: &Timer) -> State {
        let mut state = Default::default();
        self.update_state(&mut state, image_cache, timer);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values for the specified language.
    pub fn settings_description(&self, lang: Lang) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                Text::TitleBackground.resolve(lang).into(),
                Text::TitleBackgroundDescription.resolve(lang).into(),
                self.settings.background.into(),
            ),
            Field::new(
                Text::TitleTextColor.resolve(lang).into(),
                Text::TitleTextColorDescription.resolve(lang).into(),
                self.settings.text_color.into(),
            ),
            Field::new(
                Text::ShowGameName.resolve(lang).into(),
                Text::ShowGameNameDescription.resolve(lang).into(),
                self.settings.show_game_name.into(),
            ),
            Field::new(
                Text::ShowCategoryName.resolve(lang).into(),
                Text::ShowCategoryNameDescription.resolve(lang).into(),
                self.settings.show_category_name.into(),
            ),
            Field::new(
                Text::ShowFinishedRunsCount.resolve(lang).into(),
                Text::ShowFinishedRunsCountDescription.resolve(lang).into(),
                self.settings.show_finished_runs_count.into(),
            ),
            Field::new(
                Text::ShowAttemptCount.resolve(lang).into(),
                Text::ShowAttemptCountDescription.resolve(lang).into(),
                self.settings.show_attempt_count.into(),
            ),
            Field::new(
                Text::TextAlignment.resolve(lang).into(),
                Text::TextAlignmentDescription.resolve(lang).into(),
                self.settings.text_alignment.into(),
            ),
            Field::new(
                Text::DisplayTextAsSingleLine.resolve(lang).into(),
                Text::DisplayTextAsSingleLineDescription
                    .resolve(lang)
                    .into(),
                self.settings.display_as_single_line.into(),
            ),
            Field::new(
                Text::DisplayGameIcon.resolve(lang).into(),
                Text::DisplayGameIconDescription.resolve(lang).into(),
                self.settings.display_game_icon.into(),
            ),
            Field::new(
                Text::ShowRegion.resolve(lang).into(),
                Text::ShowRegionDescription.resolve(lang).into(),
                self.settings.show_region.into(),
            ),
            Field::new(
                Text::ShowPlatform.resolve(lang).into(),
                Text::ShowPlatformDescription.resolve(lang).into(),
                self.settings.show_platform.into(),
            ),
            Field::new(
                Text::ShowVariables.resolve(lang).into(),
                Text::ShowVariablesDescription.resolve(lang).into(),
                self.settings.show_variables.into(),
            ),
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
