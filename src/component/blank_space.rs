//! Provides the Blank Space Component and relevant types for using it. The
//! Blank Space Component is simply an empty component that doesn't show
//! anything other than a background. It mostly serves as padding between other
//! components.

use crate::settings::{Field, Gradient, SettingsDescription, Value};
use crate::Timer;
use serde_json::{to_writer, Result};
use std::borrow::Cow;
use std::io::Write;

/// The Blank Space Component is simply an empty component that doesn't show
/// anything other than a background. It mostly serves as padding between other
/// components.
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
    /// The height of the component.
    pub height: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            background: Gradient::Transparent,
            height: 24,
        }
    }
}

/// The state object describes the information to visualize for this component.
#[derive(Serialize, Deserialize)]
pub struct State {
    /// The background shown behind the component.
    pub background: Gradient,
    /// The height of the component.
    pub height: u32,
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
    /// Creates a new Blank Space Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Blank Space Component with the given settings.
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
        "Blank Space".into()
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, _timer: &Timer) -> State {
        State {
            background: self.settings.background,
            height: self.settings.height,
        }
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::with_fields(vec![
            Field::new("Background".into(), self.settings.background.into()),
            Field::new("Height".into(), u64::from(self.settings.height).into()),
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
            1 => self.settings.height = value.into_uint().unwrap() as _,
            _ => panic!("Unsupported Setting Index"),
        }
    }
}
