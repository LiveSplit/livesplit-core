//! Provides the Text Component and relevant types for using it. The Text
//! Component simply visualizes any given text. This can either be a single
//! centered text, or split up into a left and right text, which is suitable for
//! a situation where you have a label and a value.

use super::key_value;
use crate::platform::prelude::*;
use crate::{
    settings::{Color, Field, Gradient, SettingsDescription, Value},
    timing::formatter,
    Timer,
};
use alloc::borrow::Cow;
use core::mem::replace;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

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
    /// The background shown behind the component.
    pub background: Gradient,
    /// Specifies whether to display the left and right text is supposed to be
    /// displayed as two rows.
    pub display_two_rows: bool,
    /// The color of the left part of the split up text or the whole text if
    /// it's not split up. If `None` is specified, the color is taken from the
    /// layout.
    pub left_center_color: Option<Color>,
    /// The color of the right part of the split up text. This can be ignored if
    /// the text is not split up. If `None` is specified, the color is taken
    /// from the layout.
    pub right_color: Option<Color>,
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
    /// A custom variable with the name specified is supposed to be shown. The
    /// boolean indicates whether the name should also be shown as a key value
    /// pair.
    Variable(String, bool),
}

/// The text that is supposed to be shown.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextState {
    /// A single centered text.
    Center(String),
    /// A text that is split up into a left and right part. This is suitable for
    /// a situation where you have a label and a value.
    Split(String, String),
}

impl Text {
    /// Returns whether the text is split up into a left and right part.
    pub fn is_split(&self) -> bool {
        match *self {
            Text::Split(_, _) => true,
            Text::Center(_) => false,
            Text::Variable(_, is_split) => is_split,
        }
    }

    /// Sets the centered text. If the current mode is split, it is switched to
    /// centered mode.
    pub fn set_center<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let Text::Center(inner) = self {
            *inner = text;
        } else {
            *self = Text::Center(text);
        }
    }

    /// Sets the left text. If the current mode is centered, it is switched to
    /// split mode, with the right text being empty.
    pub fn set_left<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let Text::Split(inner, _) = self {
            *inner = text;
        } else {
            *self = Text::Split(text, String::from(""));
        }
    }

    /// Sets the right text. If the current mode is centered, it is switched to
    /// split mode, with the left text being empty.
    pub fn set_right<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let Text::Split(_, inner) = self {
            *inner = text;
        } else {
            *self = Text::Split(String::from(""), text);
        }
    }
}

/// The state object describes the information to visualize for this component.
#[derive(Serialize, Deserialize)]
pub struct State {
    /// The background shown behind the component.
    pub background: Gradient,
    /// Specifies whether to display the left and right text is supposed to be
    /// displayed as two rows.
    pub display_two_rows: bool,
    /// The color of the left part of the split up text or the whole text if
    /// it's not split up. If `None` is specified, the color is taken from the
    /// layout.
    pub left_center_color: Option<Color>,
    /// The color of the right part of the split up text. This can be ignored if
    /// the text is not split up. If `None` is specified, the color is taken
    /// from the layout.
    pub right_color: Option<Color>,
    /// The text to show for the component.
    pub text: TextState,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: key_value::DEFAULT_GRADIENT,
            display_two_rows: false,
            left_center_color: None,
            right_color: None,
            text: Text::Center(String::from("")),
        }
    }
}

#[cfg(feature = "std")]
impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }
}

impl Component {
    /// Creates a new Text Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Text Component with the given settings.
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
        let name: Cow<'_, str> = match &self.settings.text {
            Text::Center(text) => text.as_str().into(),
            Text::Split(left, right) => {
                let mut name = String::with_capacity(left.len() + right.len() + 1);
                name.push_str(left);
                if !left.is_empty() && !right.is_empty() {
                    name.push_str(" ");
                }
                name.push_str(right);
                name.into()
            }
            Text::Variable(var_name, _) => var_name.as_str().into(),
        };

        if name.trim().is_empty() {
            "Text".into()
        } else {
            name
        }
    }

    /// Calculates the component's state.
    pub fn state(&self, timer: &Timer) -> State {
        State {
            background: self.settings.background,
            display_two_rows: self.settings.text.is_split() && self.settings.display_two_rows,
            left_center_color: self.settings.left_center_color,
            right_color: self.settings.right_color,
            text: match &self.settings.text {
                Text::Center(center) => TextState::Center(center.clone()),
                Text::Split(left, right) => TextState::Split(left.clone(), right.clone()),
                Text::Variable(var_name, is_split) => {
                    let value = timer
                        .run()
                        .metadata()
                        .custom_variable(&var_name)
                        .map(|var| var.value.as_str())
                        .filter(|value| !value.trim_start().is_empty())
                        .unwrap_or(formatter::DASH);

                    if *is_split {
                        TextState::Split(var_name.clone(), value.to_owned())
                    } else {
                        TextState::Center(value.to_owned())
                    }
                }
            },
        }
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        let (first, second, is_variable, is_split, left_color, right_color) =
            match &self.settings.text {
                Text::Center(text) => (
                    Field::new("Text".into(), text.to_string().into()),
                    None,
                    false,
                    false,
                    "Text Color",
                    "",
                ),
                Text::Split(left, right) => (
                    Field::new("Left".into(), left.to_string().into()),
                    Some(Field::new("Right".into(), right.to_string().into())),
                    false,
                    true,
                    "Left Color",
                    "Right Color",
                ),
                Text::Variable(var_name, is_split) => (
                    Field::new("Variable".into(), var_name.to_string().into()),
                    None,
                    true,
                    *is_split,
                    if *is_split {
                        "Name Color"
                    } else {
                        "Value Color"
                    },
                    "Value Color",
                ),
            };

        let mut fields = vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new("Use Variable".into(), is_variable.into()),
            Field::new("Split".into(), is_split.into()),
            first,
            Field::new(left_color.into(), self.settings.left_center_color.into()),
        ];

        if let Some(second) = second {
            fields.push(second);
        }

        if is_split {
            fields.push(Field::new(
                right_color.into(),
                self.settings.right_color.into(),
            ));
            fields.push(Field::new(
                "Display 2 Rows".into(),
                self.settings.display_two_rows.into(),
            ));
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
    pub fn set_value(&mut self, mut index: usize, value: Value) {
        if index >= 5 {
            if let Text::Variable(_, _) = &self.settings.text {
                index += 1;
            }
        }

        match index {
            0 => self.settings.background = value.into(),
            1 => {
                self.settings.text = match (value.into_bool().unwrap(), &mut self.settings.text) {
                    (false, Text::Variable(name, true)) => {
                        Text::Split(replace(name, String::new()), String::new())
                    }
                    (false, Text::Variable(name, false)) => {
                        Text::Center(replace(name, String::new()))
                    }
                    (true, Text::Center(center)) => {
                        Text::Variable(replace(center, String::new()), false)
                    }
                    (true, Text::Split(left, _)) => {
                        Text::Variable(replace(left, String::new()), true)
                    }
                    _ => return,
                };
            }
            2 => {
                self.settings.text = match (value.into_bool().unwrap(), &mut self.settings.text) {
                    (true, Text::Center(center)) => {
                        self.settings.right_color = self.settings.left_center_color;
                        self.settings.display_two_rows = false;

                        Text::Split(replace(center, String::new()), String::new())
                    }
                    (false, Text::Split(left, right)) => {
                        let mut value = replace(left, String::new());
                        let right = replace(right, String::new());
                        if !value.is_empty() && !right.is_empty() {
                            value.push(' ');
                        }
                        value.push_str(&right);

                        Text::Center(value)
                    }
                    (should_be_split, Text::Variable(_, is_split)) => {
                        *is_split = should_be_split;
                        return;
                    }
                    _ => return,
                };
            }
            3 => match &mut self.settings.text {
                Text::Center(center) => *center = value.into(),
                Text::Split(left, _) => *left = value.into(),
                Text::Variable(var_name, _) => *var_name = value.into(),
            },
            4 => self.settings.left_center_color = value.into(),
            5 => match &mut self.settings.text {
                Text::Center(_) => panic!("Can't set right text when there's only a center text"),
                Text::Split(_, right) => *right = value.into(),
                Text::Variable(_, _) => {
                    unreachable!("Shouldn't be able to set value for a variable")
                }
            },
            6 => self.settings.right_color = value.into(),
            7 => self.settings.display_two_rows = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
