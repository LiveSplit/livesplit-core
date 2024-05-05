use super::Value;
use alloc::borrow::Cow;
use serde_derive::{Deserialize, Serialize};

/// A Field describes a single setting by its name and its current value.
#[derive(Serialize, Deserialize)]
pub struct Field {
    /// The name of the setting.
    pub text: Cow<'static, str>,
    /// The tooltip to show for the setting.
    pub tooltip: Cow<'static, str>,
    /// The current value of the setting.
    pub value: Value,
}

impl Field {
    /// Creates a new field.
    pub const fn new(text: Cow<'static, str>, tooltip: Cow<'static, str>, value: Value) -> Self {
        Self {
            text,
            tooltip,
            value,
        }
    }
}
