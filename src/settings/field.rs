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
    /// An optional hint about how to display the setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<Hint>,
    /// The current value of the setting.
    pub value: Value,
}

/// A hint about how to display the setting.
#[derive(Serialize, Deserialize)]
pub enum Hint {
    /// Display the setting as a selection of a comparison.
    Comparison,
    /// Display the setting as a selection of a custom variable.
    CustomVariable,
}

impl Field {
    /// Creates a new field.
    pub const fn new(text: Cow<'static, str>, tooltip: Cow<'static, str>, value: Value) -> Self {
        Self {
            text,
            tooltip,
            hint: None,
            value,
        }
    }

    /// Sets a hint for the field.
    pub fn with_hint(self, hint: Hint) -> Self {
        Self {
            hint: Some(hint),
            ..self
        }
    }
}
