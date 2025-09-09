//! Provides the Delta Component and relevant types for using it. The Delta
//! Component is a component that shows how far ahead or behind the current
//! attempt is compared to the chosen comparison.

use super::key_value;
use crate::{
    GeneralLayoutSettings,
    analysis::{delta, state_helper},
    comparison,
    platform::prelude::*,
    settings::{Color, Field, Gradient, SemanticColor, SettingsDescription, Value},
    timing::{
        Snapshot,
        formatter::{Accuracy, Delta, TimeFormatter},
    },
};
use alloc::borrow::Cow;
use core::fmt::Write;
use serde_derive::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// The Delta Component is a component that shows how far ahead or behind the
/// current attempt is compared to the chosen comparison.
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

    /// Accesses the name of the component.
    pub fn name(&self) -> Cow<'static, str> {
        if let Some(comparison) = &self.settings.comparison_override {
            format!("Delta ({comparison})").into()
        } else {
            "Delta".into()
        }
    }

    /// Updates the component's state based on the timer and layout settings
    /// provided.
    pub fn update_state(
        &self,
        state: &mut key_value::State,
        timer: &Snapshot,
        layout_settings: &GeneralLayoutSettings,
    ) {
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

        state.background = self.settings.background;
        state.key_color = self.settings.label_color;
        state.value_color = value_color;

        state.key.clear();
        state.key.push_str(text);

        state.value.clear();
        let _ = write!(
            state.value,
            "{}",
            Delta::custom(self.settings.drop_decimals, self.settings.accuracy).format(delta),
        );

        state.key_abbreviations.clear();
        if let Some(abbreviation) = comparison::try_shorten(text) {
            state.key_abbreviations.push(abbreviation.into());
        }

        state.display_two_rows = self.settings.display_two_rows;
        state.updates_frequently = use_live_delta;
    }

    /// Calculates the component's state based on the timer and the layout
    /// settings provided.
    pub fn state(
        &self,
        timer: &Snapshot,
        layout_settings: &GeneralLayoutSettings,
    ) -> key_value::State {
        let mut state = Default::default();
        self.update_state(&mut state, timer, layout_settings);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new(
                "Background".into(),
                "The background shown behind the component.".into(),
                self.settings.background.into(),
            ),
            Field::new(
                "Comparison".into(),
                "The comparison to use for calculating how far ahead or behind the current attempt is. If not specified, the current comparison is used.".into(),
                self.settings.comparison_override.clone().into(),
            ),
            Field::new(
                "Display 2 Rows".into(),
                "Specifies whether to display the name of the comparison and the delta in two separate rows.".into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new(
                "Label Color".into(),
                "The color of the comparison name. If not specified, the color is taken from the layout.".into(),
                self.settings.label_color.into()
            ),
            Field::new(
                "Drop Decimals".into(),
                "Specifies if the decimals should not be shown anymore when the visualized delta is over a minute.".into(),
                self.settings.drop_decimals.into(),
            ),
            Field::new(
                "Accuracy".into(),
                "The accuracy of the delta shown.".into(),
                self.settings.accuracy.into()
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
            4 => self.settings.drop_decimals = value.into(),
            5 => self.settings.accuracy = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
