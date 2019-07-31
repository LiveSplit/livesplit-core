//! Provides the PB Chance Component and relevant types for using it. The PB
//! Chance Component is a component that shows how likely it is to beat the
//! Personal Best. If there is no active attempt it shows the general chance of
//! beating the Personal Best. During an attempt it actively changes based on
//! how well the attempt is going.

use super::DEFAULT_KEY_VALUE_GRADIENT;
use crate::settings::{Color, Field, Gradient, SettingsDescription, Value};
use crate::{analysis::pb_chance, Timer};
use serde::{Deserialize, Serialize};
use serde_json::{to_writer, Result};
use std::borrow::Cow;
use std::io::Write;

/// The PB Chance Component is a component that shows how likely it is to beat
/// the Personal Best. If there is no active attempt it shows the general chance
/// of beating the Personal Best. During an attempt it actively changes based on
/// how well the attempt is going.
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
            background: DEFAULT_KEY_VALUE_GRADIENT,
            display_two_rows: false,
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
    /// The current PB chance.
    pub pb_chance: String,
    /// Specifies whether to display the name of the component and its value in
    /// two separate rows.
    pub display_two_rows: bool,
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
    /// Creates a new Possible Time Save Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Possible Time Save Component with the given settings.
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
    pub fn name(&self) -> Cow<'_, str> {
        "PB Chance".into()
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, timer: &Timer) -> State {
        let chance = pb_chance::for_timer(timer);

        State {
            background: self.settings.background,
            label_color: self.settings.label_color,
            value_color: self.settings.value_color,
            text: self.name().into_owned(),
            pb_chance: format!("{:.1}%", 100.0 * chance),
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
