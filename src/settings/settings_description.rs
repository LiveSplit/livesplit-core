use super::Field;
use serde::{Deserialize, Serialize};
use crate::platform::prelude::*;

/// A generic description of the settings available and their current values.
#[derive(Default, Serialize, Deserialize)]
pub struct SettingsDescription {
    /// All of the different settings that are available and their current
    /// values.
    pub fields: Vec<Field>,
}

impl SettingsDescription {
    /// Creates a new Settings Description with the settings provided.
    pub fn with_fields(fields: Vec<Field>) -> Self {
        Self { fields }
    }
}
