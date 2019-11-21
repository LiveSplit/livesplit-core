use super::Value;
use crate::platform::prelude::*;
use serde::{Deserialize, Serialize};

/// A Field describes a single setting by its name and its current value.
#[derive(Serialize, Deserialize)]
pub struct Field {
    /// The name of the setting.
    pub text: String,
    /// The current value of the setting.
    pub value: Value,
}

impl Field {
    /// Creates a new field.
    pub fn new(text: String, value: Value) -> Self {
        Self { text, value }
    }
}
