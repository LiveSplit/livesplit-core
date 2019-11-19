//! Provides the Current Comparison Component and relevant types for using it.
//! The Current Comparison Component is a component that shows the name of the
//! comparison that is currently selected to be compared against.

use super::key_value;
use crate::platform::prelude::*;
use crate::settings::{Color, Field, Gradient, SettingsDescription, Value};
use crate::Timer;
use serde::{Deserialize, Serialize};

/// The Current Comparison Component is a component that shows the name of the
/// comparison that is currently selected to be compared against.
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
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: key_value::DEFAULT_GRADIENT,
            display_two_rows: false,
            label_color: None,
            value_color: None,
        }
    }
}

impl Component {
    /// Creates a new Current Comparison Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Current Comparison Component with the given settings.
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
    pub fn name(&self) -> &'static str {
        "Current Comparison"
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Timer) -> key_value::State {
        key_value::State {
            background: self.settings.background,
            key_color: self.settings.label_color,
            value_color: self.settings.value_color,
            semantic_color: Default::default(),
            key: "Comparing Against".into(),
            value: timer.current_comparison().into(),
            key_abbreviations: Box::new(["Comparison".into()]) as _,
            display_two_rows: self.settings.display_two_rows,
        }
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
            2 => self.settings.label_color = value.into(),
            3 => self.settings.value_color = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
