use std::io::Write;
use serde_json::{to_writer, Result};
use std::borrow::Cow;
use settings::{SettingsDescription, Field, Value};
use std::mem::replace;

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub text: Text,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Text {
    Center(String),
    Split(String, String),
}

impl Text {
    pub fn set_center<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let Text::Center(ref mut inner) = *self {
            *inner = text;
        } else {
            *self = Text::Center(text);
        }
    }

    pub fn set_left<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let Text::Split(ref mut inner, _) = *self {
            *inner = text;
        } else {
            *self = Text::Split(text, String::from(""));
        }
    }

    pub fn set_right<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let Text::Split(_, ref mut inner) = *self {
            *inner = text;
        } else {
            *self = Text::Split(String::from(""), text);
        }
    }
}

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
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

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

    pub fn state(&self) -> State {
        State(self.settings.text.clone())
    }

    pub fn settings_description(&self) -> SettingsDescription {
        let (first, second) = match self.settings.text {
            Text::Center(ref text) => (Field::new("Text".into(), text.to_string().into()), None),
            Text::Split(ref left, ref right) => {
                (
                    Field::new("Left".into(), left.to_string().into()),
                    Some(Field::new("Right".into(), right.to_string().into())),
                )
            }
        };

        let mut fields = vec![Field::new("Split".into(), second.is_some().into()), first];

        if let Some(second) = second {
            fields.push(second);
        }

        SettingsDescription::with_fields(fields)
    }

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
            1 => {
                match self.settings.text {
                    Text::Center(ref mut center) => *center = value.into(),
                    Text::Split(ref mut left, _) => *left = value.into(),
                }
            }
            2 => {
                match self.settings.text {
                    Text::Center(_) => panic!("Set right text when there's only a center text"),
                    Text::Split(_, ref mut right) => *right = value.into(),
                }
            }
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
