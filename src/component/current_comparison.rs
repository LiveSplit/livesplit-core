//! Provides the Current Comparison Component and relevant types for using it.
//! The Current Comparison Component is a component that shows the name of the
//! comparison that is currently selected to be compared against.

use super::DEFAULT_INFO_TEXT_GRADIENT;
use Timer;
use serde_json::{to_writer, Result};
use settings::{Color, Field, Gradient, SettingsDescription, Value};
use std::borrow::Cow;
use std::io::Write;

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
            background: DEFAULT_INFO_TEXT_GRADIENT,
            label_color: None,
            value_color: None,
        }
    }
}

/// The state object describes the information to visualize for this component.
#[derive(Serialize, Deserialize)]
pub struct State {
    /// The background shown behind the component.
    pub background: Gradient,
    /// The color of the label. If `None` is specified, the color is taken from
    /// the layout.
    pub label_color: Option<Color>,
    /// The color of the value. If `None` is specified, the color is taken from
    /// the layout.
    pub value_color: Option<Color>,
    /// The label's text.
    pub text: String,
    /// The name of the comparison that is currently selected to be compared
    /// against.
    pub comparison: String,
}

impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

impl Component {
    /// Creates a new Current Comparison Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Current Comparison Component with the given settings.
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
    pub fn name(&self) -> Cow<str> {
        "Current Comparison".into()
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Timer) -> State {
        State {
            background: self.settings.background,
            label_color: self.settings.label_color,
            value_color: self.settings.value_color,
            text: String::from("Comparing Against"),
            comparison: timer.current_comparison().to_string(),
        }
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
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
            1 => self.settings.label_color = value.into(),
            2 => self.settings.value_color = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
