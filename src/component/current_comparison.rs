use Timer;
use serde_json::{to_writer, Result};
use std::io::Write;
use std::borrow::Cow;
use settings::{SettingsDescription, Field, Value, Gradient};
use super::DEFAULT_INFO_TEXT_GRADIENT;

#[derive(Default, Clone)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub background: Gradient,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: DEFAULT_INFO_TEXT_GRADIENT,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub background: Gradient,
    pub text: String,
    pub comparison: String,
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
        "Current Comparison".into()
    }

    pub fn state(&self, timer: &Timer) -> State {
        State {
            background: self.settings.background,
            text: String::from("Comparing Against"),
            comparison: timer.current_comparison().to_string(),
        }
    }

    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
        ])
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match index {
            0 => self.settings.background = value.into(),
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
