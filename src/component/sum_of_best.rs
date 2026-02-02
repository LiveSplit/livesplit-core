//! Provides the Sum of Best Segments Component. The Sum of Best Segments
//! Component shows the fastest possible time to complete a run of this
//! category, based on information collected from all the previous attempts.
//! This often matches up with the sum of the best segment times of all the
//! segments, but that may not always be the case, as skipped segments may
//! introduce combined segments that may be faster than the actual sum of their
//! best segment times. The name is therefore a bit misleading, but sticks
//! around for historical reasons.

use super::key_value;
use crate::{
    Timer,
    analysis::sum_of_segments::calculate_best,
    localization::{Lang, Text},
    platform::prelude::*,
    settings::{Color, Field, Gradient, SettingsDescription, Value},
    timing::formatter::{Accuracy, Regular, TimeFormatter},
};
use core::fmt::Write;
use serde_derive::{Deserialize, Serialize};

/// The Sum of Best Segments Component shows the fastest possible time to
/// complete a run of this category, based on information collected from all the
/// previous attempts. This often matches up with the sum of the best segment
/// times of all the segments, but that may not always be the case, as skipped
/// segments may introduce combined segments that may be faster than the actual
/// sum of their best segment times. The name is therefore a bit misleading, but
/// sticks around for historical reasons.
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
    /// Specifies whether to display the name of the component and its value in
    /// two separate rows.
    pub display_two_rows: bool,
    /// The color of the label. If `None` is specified, the color is taken from
    /// the layout.
    pub label_color: Option<Color>,
    /// The color of the value. If `None` is specified, the color is taken from
    /// the layout.
    pub value_color: Option<Color>,
    /// The accuracy of the time shown.
    pub accuracy: Accuracy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: key_value::DEFAULT_GRADIENT,
            display_two_rows: false,
            label_color: None,
            value_color: None,
            accuracy: Accuracy::Seconds,
        }
    }
}

impl Component {
    /// Creates a new Sum of Best Segments Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Sum of Best Segments Component with the given settings.
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
        Text::ComponentSumOfBest.resolve(lang)
    }

    /// Updates the component's state based on the timer provided.
    pub fn update_state(&self, state: &mut key_value::State, timer: &Timer, lang: Lang) {
        let time = calculate_best(
            timer.run().segments(),
            false,
            true,
            timer.current_timing_method(),
        );

        state.background = self.settings.background;
        state.key_color = self.settings.label_color;
        state.value_color = self.settings.value_color;
        state.semantic_color = Default::default();

        state.key.clear();
        state.key.push_str(Text::SumOfBestSegments.resolve(lang));

        state.value.clear();
        let _ = write!(
            state.value,
            "{}",
            Regular::with_accuracy(self.settings.accuracy).format(time, lang),
        );

        state.key_abbreviations.clear();
        state
            .key_abbreviations
            .push(Text::SumOfBestShort.resolve(lang).into());
        state
            .key_abbreviations
            .push(Text::SumOfBestAbbreviation.resolve(lang).into());

        state.display_two_rows = self.settings.display_two_rows;
        state.updates_frequently = false;
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Timer, lang: Lang) -> key_value::State {
        let mut state = Default::default();
        self.update_state(&mut state, timer, lang);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values for the specified language.
    pub fn settings_description(&self, lang: Lang) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                Text::SumOfBestBackground.resolve(lang).into(),
                Text::SumOfBestBackgroundDescription.resolve(lang).into(),
                self.settings.background.into(),
            ),
            Field::new(
                Text::SumOfBestDisplayTwoRows.resolve(lang).into(),
                Text::SumOfBestDisplayTwoRowsDescription
                    .resolve(lang)
                    .into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new(
                Text::SumOfBestLabelColor.resolve(lang).into(),
                Text::SumOfBestLabelColorDescription.resolve(lang).into(),
                self.settings.label_color.into(),
            ),
            Field::new(
                Text::SumOfBestValueColor.resolve(lang).into(),
                Text::SumOfBestValueColorDescription.resolve(lang).into(),
                self.settings.value_color.into(),
            ),
            Field::new(
                Text::SumOfBestAccuracy.resolve(lang).into(),
                Text::SumOfBestAccuracyDescription.resolve(lang).into(),
                self.settings.accuracy.into(),
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
            1 => self.settings.display_two_rows = value.into(),
            2 => self.settings.label_color = value.into(),
            3 => self.settings.value_color = value.into(),
            4 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
