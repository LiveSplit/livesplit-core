//! Provides the Alternate Timing Method Component. The Alternate Timing Method
//! Component shows the current time in the timing method that is not currently
//! being used. So if the timer is comparing against Game Time, it shows the
//! current Real Time and vice versa.

use super::key_value;
use crate::{
    Timer, TimingMethod,
    localization::{Lang, Text},
    platform::prelude::*,
    settings::{Color, Field, Gradient, SettingsDescription, Value},
    timing::formatter::{Accuracy, DigitsFormat, TimeFormatter, timer as formatter},
};
use core::fmt::Write;
use serde_derive::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// The Alternate Timing Method Component shows the current time in the timing
/// method that is not currently being used. So if the timer is comparing
/// against Game Time, it shows the current Real Time and vice versa.
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
    /// The color of the label. If [`None`] is specified, the color is taken from
    /// the layout.
    pub label_color: Option<Color>,
    /// The color of the value. If [`None`] is specified, the color is taken from
    /// the layout.
    pub value_color: Option<Color>,
    /// Determines how many digits are to always be shown. If the duration is
    /// lower than the digits to be shown, they are filled up with zeros.
    pub digits_format: DigitsFormat,
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
            digits_format: DigitsFormat::SingleDigitSeconds,
            accuracy: Accuracy::Hundredths,
        }
    }
}

impl Component {
    /// Creates a new Alternate Timing Method Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Alternate Timing Method Component with the given settings.
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
        Text::ComponentAlternateTimingMethod.resolve(lang)
    }

    /// Updates the component's state based on the timer provided.
    pub fn update_state(&self, state: &mut key_value::State, timer: &Timer, lang: Lang) {
        let timing_method = match timer.current_timing_method() {
            TimingMethod::RealTime => TimingMethod::GameTime,
            TimingMethod::GameTime => TimingMethod::RealTime,
        };

        let time = timer.snapshot().current_time()[timing_method];

        state.background = self.settings.background;
        state.key_color = self.settings.label_color;
        state.value_color = self.settings.value_color;
        state.semantic_color = Default::default();

        state.key.clear();
        state.key.push_str(match timing_method {
            TimingMethod::RealTime => Text::RealTime.resolve(lang),
            TimingMethod::GameTime => Text::GameTime.resolve(lang),
        });

        state.value.clear();
        let _ = write!(
            state.value,
            "{}{}",
            formatter::Time::with_digits_format(self.settings.digits_format).format(time, lang),
            formatter::Fraction::with_accuracy(self.settings.accuracy).format(time, lang),
        );

        state.key_abbreviations.clear();
        state.display_two_rows = self.settings.display_two_rows;
        state.updates_frequently =
            timer.current_phase().updates_frequently(timing_method) && time.is_some();
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
                Text::AlternateTimingMethodBackground.resolve(lang).into(),
                Text::AlternateTimingMethodBackgroundDescription
                    .resolve(lang)
                    .into(),
                self.settings.background.into(),
            ),
            Field::new(
                Text::AlternateTimingMethodDisplayTwoRows
                    .resolve(lang)
                    .into(),
                Text::AlternateTimingMethodDisplayTwoRowsDescription
                    .resolve(lang)
                    .into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new(
                Text::AlternateTimingMethodLabelColor.resolve(lang).into(),
                Text::AlternateTimingMethodLabelColorDescription
                    .resolve(lang)
                    .into(),
                self.settings.label_color.into(),
            ),
            Field::new(
                Text::AlternateTimingMethodValueColor.resolve(lang).into(),
                Text::AlternateTimingMethodValueColorDescription
                    .resolve(lang)
                    .into(),
                self.settings.value_color.into(),
            ),
            Field::new(
                Text::AlternateTimingMethodDigitsFormat.resolve(lang).into(),
                Text::AlternateTimingMethodDigitsFormatDescription
                    .resolve(lang)
                    .into(),
                self.settings.digits_format.into(),
            ),
            Field::new(
                Text::AlternateTimingMethodAccuracy.resolve(lang).into(),
                Text::AlternateTimingMethodAccuracyDescription
                    .resolve(lang)
                    .into(),
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
            4 => self.settings.digits_format = value.into(),
            5 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
