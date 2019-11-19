//! Provides the Delta Component and relevant types for using it. The Delta
//! Component is a component that shows the how far ahead or behind the current
//! attempt is compared to the chosen comparison.

use super::key_value;
use crate::analysis::{delta, state_helper};
use crate::platform::prelude::*;
use crate::settings::{Color, Field, Gradient, SemanticColor, SettingsDescription, Value};
use crate::timing::formatter::{Accuracy, Delta, TimeFormatter};
use crate::{comparison, GeneralLayoutSettings, Timer};
use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// The Delta Component is a component that shows the how far ahead or behind
/// the current attempt is compared to the chosen comparison.
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
    /// Specifies if the decimals should not be shown anymore when the
    /// visualized delta is above one minute.
    pub drop_decimals: bool,
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
            drop_decimals: true,
            accuracy: Accuracy::Tenths,
        }
    }
}

impl Component {
    /// Creates a new Delta Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Delta Component with the given settings.
    pub fn with_settings(settings: Settings) -> Self {
        Self { settings }
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
    pub fn name(&self) -> Cow<'static, str> {
        if let Some(comparison) = &self.settings.comparison_override {
            format!("Delta ({})", comparison).into()
        } else {
            "Delta".into()
        }
    }

    /// Calculates the component's state based on the timer and the layout
    /// settings provided.
    pub fn state(
        &self,
        timer: &Timer,
        layout_settings: &GeneralLayoutSettings,
    ) -> key_value::State {
        let comparison = comparison::resolve(&self.settings.comparison_override, timer);
        let text = comparison.unwrap_or_else(|| timer.current_comparison());
        let comparison = comparison::or_current(comparison, timer);

        let (delta, use_live_delta) = delta::calculate(timer, comparison);

        let mut index = timer.current_split_index();
        if !use_live_delta {
            index = index.and_then(|i| i.checked_sub(1));
        }

        let semantic_color = if let Some(index) = index {
            state_helper::split_color(
                timer,
                delta,
                index,
                true,
                false,
                comparison,
                timer.current_timing_method(),
            )
        } else {
            SemanticColor::Default
        };

        let value_color = Some(semantic_color.visualize(layout_settings));

        let abbreviation = comparison::shorten(text);

        key_value::State {
            background: self.settings.background,
            key_color: self.settings.label_color,
            value_color,
            semantic_color,
            key: text.to_string().into(),
            value: Delta::custom(self.settings.drop_decimals, self.settings.accuracy)
                .format(delta)
                .to_string()
                .into(),
            key_abbreviations: Box::new([abbreviation.into()]) as _,
            display_two_rows: self.settings.display_two_rows,
        }
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new(
                "Comparison".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new(
                "Display 2 Rows".into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new("Label Color".into(), self.settings.label_color.into()),
            Field::new("Drop Decimals".into(), self.settings.drop_decimals.into()),
            Field::new("Accuracy".into(), self.settings.accuracy.into()),
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
            4 => self.settings.drop_decimals = value.into(),
            5 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
