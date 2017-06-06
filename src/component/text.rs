use std::io::Write;
use serde_json::{to_writer, Result};

#[derive(Default)]
pub struct Component {
    settings: Settings,
}

#[derive(Serialize, Deserialize)]
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
        if let &mut Text::Center(ref mut inner) = self {
            *inner = text;
        } else {
            *self = Text::Center(text);
        }
    }

    pub fn set_left<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let &mut Text::Split(ref mut inner, _) = self {
            *inner = text;
        } else {
            *self = Text::Split(text, String::from(""));
        }
    }

    pub fn set_right<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        if let &mut Text::Split(_, ref mut inner) = self {
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
        Self { text: Text::Center(String::from("")) }
    }
}

impl State {
    pub fn write_json<W>(&self, mut writer: W) -> Result<()>
        where W: Write
    {
        to_writer(&mut writer, self)
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

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn state(&self) -> State {
        State(self.settings.text.clone())
    }
}
