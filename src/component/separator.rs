//! Provides the Separator Component and relevant types for using it. The
//! Separator Component is a simple component that only serves to render
//! separators between components.

use crate::settings::{SettingsDescription, Value};
use crate::Timer;
use serde::{Deserialize, Serialize};

/// The Separator Component is a simple component that only serves to render
/// separators between components.
#[derive(Default, Clone)]
pub struct Component;

/// The state object describes the information to visualize for this component.
#[derive(Serialize, Deserialize)]
pub struct State;

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
    /// Creates a new Separator Component.
    pub fn new() -> Self {
        Default::default()
    }

    /// Accesses the name of the component.
    pub fn name(&self) -> &'static str {
        "Separator"
    }

    /// Calculates the component's state based on the timer provided.
    pub fn state(&self, _timer: &Timer) -> State {
        State
    }

    /// Accesses a generic description of the settings available for this
    /// component and their current values.
    pub fn settings_description(&self) -> SettingsDescription {
        SettingsDescription::default()
    }

    /// Sets a setting's value by its index to the given value.
    ///
    /// # Panics
    ///
    /// This panics if the type of the value to be set is not compatible with
    /// the type of the setting's value. A panic can also occur if the index of
    /// the setting provided is out of bounds.
    #[allow(clippy::needless_pass_by_value)]
    pub fn set_value(&mut self, _index: usize, _value: Value) {}
}
