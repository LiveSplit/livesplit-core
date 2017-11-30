//! Provides the Text Component and relevant types for using it. The Text
//! Component simply visualizes any given text. This can either be a single
//! centered text, or split up into a left and right text, which is suitable for
//! a situation where you have a label and a value.

use std::io::Write;
use serde_json::{to_writer, Result};
use std::borrow::Cow;
use settings::{Field, SettingsDescription, Value};
use std::mem::replace;

/// The Text Component simply visualizes any given text. This can either be a
/// single centered text, or split up into a left and right text, which is
/// suitable for a situation where you have a label and a value.
#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

/// The Settings for this component.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The text to be shown.
    pub text: Text,
}

/// The text that is supposed to be shown.
#[derive(Clone, Serialize, Deserialize)]
pub enum Text {
    /// A single centered text.
    Center(String),
    /// A text that is split up into a left and right part. This is suitable for
    /// a situation where you have a label and a value.
    Split(String, String),
}

impl Text {
    /// Sets the centered text. If the current mode is split, it is switched to
    /// centered mode.
    pub fn set_center<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let Text::Center(ref mut inner) = *self {
            *inner = text;
        } else {
            *self = Text::Center(text);
        }
    }

    /// Sets the left text. If the current mode is centered, it is switched to
    /// split mode, with the right text being empty.
    pub fn set_left<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let Text::Split(ref mut inner, _) = *self {
            *inner = text;
        } else {
            *self = Text::Split(text, String::from(""));
        }
    }

    /// Sets the right text. If the current mode is centered, it is switched to
    /// split mode, with the left text being empty.
    pub fn set_right<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let Text::Split(_, ref mut inner) = *self {
            *inner = text;
        } else {
            *self = Text::Split(String::from(""), text);
        }
    }
}

/// The state object describes the information to visualize for this component.
#[derive(Serialize, Deserialize)]
pub struct State(pub Text);

impl Default for Settings {
    fn default() -> Self {
        Self {
            text: Text::Center(String::from("")),
        }
    }
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
    /// Creates a new Text Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Text Component with the given settings.
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
        let name: Cow<str> = match self.settings.text {
            Text::Center(ref text) => text.as_str().into(),
            Text::Split(ref left, ref right) => {
                let mut name = String::with_capacity(left.len() + right.len() + 1);
                name.push_str(left);
                if !left.is_empty() && !right.is_empty() {
                    name.push_str(" ");
                }
                name.push_str(right);
                name.into()
            }
        };

        if name.trim().is_empty() {
            "Text".into()
        } else {
            name
        }
    }

    /// Calculates the component's state.
    pub fn state(&self) -> State {
        State(self.settings.text.clone())
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        let (first, second) = match self.settings.text {
            Text::Center(ref text) => (Field::new("Text".into(), text.to_string().into()), None),
            Text::Split(ref left, ref right) => (
                Field::new("Left".into(), left.to_string().into()),
                Some(Field::new("Right".into(), right.to_string().into())),
            ),
        };

        let mut fields = vec![Field::new("Split".into(), second.is_some().into()), first];

        if let Some(second) = second {
            fields.push(second);
        }

        SettingsDescription::with_fields(fields)
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
            0 => {
                self.settings.text = match (value.into_bool().unwrap(), &mut self.settings.text) {
                    (true, &mut Text::Center(ref mut center)) => {
                        Text::Split(replace(center, String::new()), String::new())
                    }
                    (false, &mut Text::Split(ref mut left, ref mut right)) => {
                        let mut value = replace(left, String::new());
                        let right = replace(right, String::new());
                        if !value.is_empty() && !right.is_empty() {
                            value.push(' ');
                        }
                        value.push_str(&right);

                        Text::Center(value)
                    }
                    _ => return,
                };
            }
            1 => match self.settings.text {
                Text::Center(ref mut center) => *center = value.into(),
                Text::Split(ref mut left, _) => *left = value.into(),
            },
            2 => match self.settings.text {
                Text::Center(_) => panic!("Set right text when there's only a center text"),
                Text::Split(_, ref mut right) => *right = value.into(),
            },
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
