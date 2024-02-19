//! Provides the Separator Component and relevant types for using it. The
//! Separator Component is a simple component that only serves to render
//! separators between components.

use serde_derive::{Deserialize, Serialize};

use crate::settings::{SettingsDescription, Value};

/// The Separator Component is a simple component that only serves to render
/// separators between components.
#[derive(Default, Clone)]
pub struct Component;

/// The state object describes the information to visualize for this component.
#[derive(Default, Serialize, Deserialize)]
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
    pub const fn name(&self) -> &'static str {
        "Separator"
    }

    /// Updates the component's state.
    pub fn update_state(&self, _state: &mut State) {}

    /// Calculates the component's state.
    pub const fn state(&self) -> State {
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
