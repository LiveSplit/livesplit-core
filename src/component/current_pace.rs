//! Provides the Current Pace Component and relevant types for using it. The
//! Current Pace Component is a component that shows a prediction of the current
//! attempt's final time, if the current attempt's pace matches the chosen
//! comparison for the remainder of the run.

use super::key_value;
use crate::{
    TimerPhase,
    analysis::current_pace,
    comparison,
    localization::{Lang, Text},
    platform::prelude::*,
    settings::{Color, Field, FieldHint, Gradient, SettingsDescription, Value},
    timing::{
        Snapshot,
        formatter::{Accuracy, Regular, TimeFormatter},
    },
};
use alloc::borrow::Cow;
use core::fmt::Write;
use serde_derive::{Deserialize, Serialize};

/// The Current Pace Component is a component that shows a prediction of the
/// current attempt's final time, if the current attempt's pace matches the
/// chosen comparison for the remainder of the run.
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
    /// The comparison chosen. Uses the Timer's current comparison if set to
    /// `None`.
    pub comparison_override: Option<String>,
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
            comparison_override: None,
            display_two_rows: false,
            label_color: None,
            value_color: None,
            accuracy: Accuracy::Seconds,
        }
    }
}

impl Component {
    /// Creates a new Current Pace Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Current Pace Component with the given settings.
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
    pub fn name(&self, lang: Lang) -> Cow<'static, str> {
        self.localized_text(lang, self.settings.comparison_override.as_deref())
    }

    fn localized_text(&self, lang: Lang, comparison: Option<&str>) -> Cow<'static, str> {
        if let Some(comparison) = comparison {
            match comparison {
                comparison::personal_best::NAME => Text::ComponentCurrentPace.resolve(lang).into(),
                comparison::best_segments::NAME => Text::ComponentCurrentPaceBestPossibleTime
                    .resolve(lang)
                    .into(),
                comparison::worst_segments::NAME => Text::ComponentCurrentPaceWorstPossibleTime
                    .resolve(lang)
                    .into(),
                comparison::average_segments::NAME => {
                    Text::ComponentCurrentPacePredictedTime.resolve(lang).into()
                }
                comparison => format!(
                    "{} ({})",
                    Text::ComponentCurrentPace.resolve(lang),
                    comparison::shorten(comparison)
                )
                .into(),
            }
        } else {
            Text::ComponentCurrentPace.resolve(lang).into()
        }
    }

    /// Updates the component's state based on the timer provided.
    pub fn update_state(&self, state: &mut key_value::State, timer: &Snapshot, lang: Lang) {
        let comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let comparison = comparison::or_current(comparison, timer);
        let key = self.localized_text(lang, Some(comparison));

        let (current_pace, updates_frequently) = if timer.current_phase() == TimerPhase::NotRunning
            && key.starts_with(Text::ComponentCurrentPace.resolve(lang))
        {
            (None, false)
        } else {
            current_pace::calculate(timer, comparison)
        };

        state.background = self.settings.background;
        state.key_color = self.settings.label_color;
        state.value_color = self.settings.value_color;
        state.semantic_color = Default::default();

        state.key.clear();
        state.key.push_str(&key); // FIXME: Uncow this

        state.value.clear();
        let _ = write!(
            state.value,
            "{}",
            Regular::with_accuracy(self.settings.accuracy).format(current_pace, lang)
        );

        state.key_abbreviations.clear();
        match comparison {
            comparison::best_segments::NAME => {
                state
                    .key_abbreviations
                    .push(Text::CurrentPaceBestPossibleTimeShort.resolve(lang).into());
                state
                    .key_abbreviations
                    .push(Text::CurrentPaceBestTimeShort.resolve(lang).into());
                state.key_abbreviations.push(
                    Text::CurrentPaceBestPossibleTimeAbbreviation
                        .resolve(lang)
                        .into(),
                );
            }
            comparison::worst_segments::NAME => {
                state
                    .key_abbreviations
                    .push(Text::CurrentPaceWorstPossibleTimeShort.resolve(lang).into());
                state
                    .key_abbreviations
                    .push(Text::CurrentPaceWorstTimeShort.resolve(lang).into());
            }
            comparison::average_segments::NAME => {
                state
                    .key_abbreviations
                    .push(Text::CurrentPacePredictedTimeShort.resolve(lang).into());
            }
            _ => {
                state
                    .key_abbreviations
                    .push(Text::ComponentCurrentPace.resolve(lang).into());
                state
                    .key_abbreviations
                    .push(Text::CurrentPaceShort.resolve(lang).into());
                state
                    .key_abbreviations
                    .push(Text::CurrentPaceAbbreviation.resolve(lang).into());
            }
        }

        state.display_two_rows = self.settings.display_two_rows;
        state.updates_frequently = updates_frequently;
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Snapshot, lang: Lang) -> key_value::State {
        let mut state = Default::default();
        self.update_state(&mut state, timer, lang);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values for the specified language.
    pub fn settings_description(&self, lang: Lang) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                Text::CurrentPaceBackground.resolve(lang).into(),
                Text::CurrentPaceBackgroundDescription.resolve(lang).into(),
                self.settings.background.into(),
            ),
            Field::new(
                Text::CurrentPaceComparison.resolve(lang).into(),
                Text::CurrentPaceComparisonDescription.resolve(lang).into(),
                self.settings.comparison_override.clone().into(),
            )
            .with_hint(FieldHint::Comparison),
            Field::new(
                Text::CurrentPaceDisplayTwoRows.resolve(lang).into(),
                Text::CurrentPaceDisplayTwoRowsDescription
                    .resolve(lang)
                    .into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new(
                Text::CurrentPaceLabelColor.resolve(lang).into(),
                Text::CurrentPaceLabelColorDescription.resolve(lang).into(),
                self.settings.label_color.into(),
            ),
            Field::new(
                Text::CurrentPaceValueColor.resolve(lang).into(),
                Text::CurrentPaceValueColorDescription.resolve(lang).into(),
                self.settings.value_color.into(),
            ),
            Field::new(
                Text::CurrentPaceAccuracy.resolve(lang).into(),
                Text::CurrentPaceAccuracyDescription.resolve(lang).into(),
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
            1 => self.settings.comparison_override = value.into(),
            2 => self.settings.display_two_rows = value.into(),
            3 => self.settings.label_color = value.into(),
            4 => self.settings.value_color = value.into(),
            5 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
