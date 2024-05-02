//! Provides the Total Playtime Component and relevant types for using it. The
//! Total Playtime is a component that shows the total amount of time that the
//! current category has been played for.

use super::key_value;
use crate::{
    analysis::total_playtime,
    platform::prelude::*,
    settings::{Color, Field, Gradient, SettingsDescription, Value},
    timing::formatter::{Days, Regular, TimeFormatter},
    Timer, TimingMethod,
};
use core::fmt::Write;
use serde_derive::{Deserialize, Serialize};

/// The Total Playtime Component is a component that shows the total amount of
/// time that the current category has been played for.
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
    /// Specifies whether the component should show the amount of days, when the
    /// total duration reaches 24 hours or more.
    pub show_days: bool,
    /// The color of the label. If `None` is specified, the color is taken from
    /// the layout.
    pub label_color: Option<Color>,
    /// The color of the value. If `None` is specified, the color is taken from
    /// the layout.
    pub value_color: Option<Color>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: key_value::DEFAULT_GRADIENT,
            display_two_rows: false,
            show_days: true,
            label_color: None,
            value_color: None,
        }
    }
}

impl Component {
    /// Creates a new Total Playtime Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Total Playtime Component with the given settings.
    pub const fn with_settings(settings: Settings) -> Self {
        Self { settings }
    }

    /// Accesses the settings of the component.
    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Grants mutable access to the settings of the component.
    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    /// Accesses the name of the component.
    pub const fn name(&self) -> &'static str {
        "Total Playtime"
    }

    /// Updates the component's state based on the timer provided.
    pub fn update_state(&self, state: &mut key_value::State, timer: &Timer) {
        let total_playtime = total_playtime::calculate(timer);

        state.background = self.settings.background;
        state.key_color = self.settings.label_color;
        state.value_color = self.settings.value_color;
        state.semantic_color = Default::default();

        state.key.clear();
        state.key.push_str("Total Playtime");

        state.value.clear();
        if self.settings.show_days {
            let _ = write!(state.value, "{}", Days::new().format(total_playtime));
        } else {
            let _ = write!(state.value, "{}", Regular::new().format(total_playtime));
        }

        state.key_abbreviations.clear();
        state.key_abbreviations.push("Playtime".into());

        state.display_two_rows = self.settings.display_two_rows;
        state.updates_frequently = timer
            .current_phase()
            .updates_frequently(TimingMethod::RealTime);
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Timer) -> key_value::State {
        let mut state = Default::default();
        self.update_state(&mut state, timer);
        state
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new(
                "Display 2 Rows".into(),
                self.settings.display_two_rows.into(),
            ),
            Field::new("Show Days (>24h)".into(), self.settings.show_days.into()),
            Field::new("Label Color".into(), self.settings.label_color.into()),
            Field::new("Value Color".into(), self.settings.value_color.into()),
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
            2 => self.settings.show_days = value.into(),
            3 => self.settings.label_color = value.into(),
            4 => self.settings.value_color = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
