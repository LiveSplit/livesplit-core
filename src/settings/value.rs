use crate::platform::prelude::*;
use crate::{
    component::splits::{ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith},
    hotkey::KeyCode,
    layout::LayoutDirection,
    settings::{Alignment, Color, Gradient, ListGradient},
    timing::formatter::{Accuracy, DigitsFormat},
    TimingMethod,
};
use core::result::Result as StdResult;
use serde::{Deserialize, Serialize};

/// Describes a setting's value. Such a value can be of a variety of different
/// types.
#[derive(derive_more::From, Serialize, Deserialize)]
pub enum Value {
    /// A boolean value.
    Bool(bool),
    /// An unsigned integer.
    UInt(u64),
    /// An integer.
    Int(i64),
    /// A string.
    String(String),
    /// An optional string.
    OptionalString(Option<String>),
    /// A floating point number.
    Float(f64),
    /// An accuracy, describing how many digits to show for the fractional part
    /// of a time.
    Accuracy(Accuracy),
    /// A digits format, describing how many digits to show for the main part of
    /// a time.
    DigitsFormat(DigitsFormat),
    /// An optional timing method.
    OptionalTimingMethod(Option<TimingMethod>),
    /// A color.
    Color(Color),
    /// An optional color.
    OptionalColor(Option<Color>),
    /// A gradient.
    Gradient(Gradient),
    /// A gradient designed for use with lists.
    ListGradient(ListGradient),
    /// An alignment for the Title Component's title.
    Alignment(Alignment),
    /// A value describing what a column of the Splits Component starts out
    /// with.
    ColumnStartWith(ColumnStartWith),
    /// A value describing what a column of the Splits Component gets updated
    /// with.
    ColumnUpdateWith(ColumnUpdateWith),
    /// A value describing when to update a column of the Splits Component.
    ColumnUpdateTrigger(ColumnUpdateTrigger),
    /// A value describing what hotkey to press to trigger a certain action.
    Hotkey(Option<KeyCode>),
    /// A value describing the direction of a layout.
    LayoutDirection(LayoutDirection),
}

/// The Error type for values that couldn't be converted.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// The value couldn't be converted because it had an incompatible type.
    WrongType,
}

/// The Result type for conversions from Values to other types.
pub type Result<T> = StdResult<T, Error>;

impl Value {
    /// Tries to convert the value into a boolean.
    pub fn into_bool(self) -> Result<bool> {
        match self {
            Value::Bool(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into an unsigned integer.
    pub fn into_uint(self) -> Result<u64> {
        match self {
            Value::UInt(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into an integer.
    pub fn into_int(self) -> Result<i64> {
        match self {
            Value::Int(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into a string.
    pub fn into_string(self) -> Result<String> {
        match self {
            Value::String(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into an optional string.
    pub fn into_optional_string(self) -> Result<Option<String>> {
        match self {
            Value::OptionalString(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into a floating point number.
    pub fn into_float(self) -> Result<f64> {
        match self {
            Value::Float(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into an accuracy.
    pub fn into_accuracy(self) -> Result<Accuracy> {
        match self {
            Value::Accuracy(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into a digits format.
    pub fn into_digits_format(self) -> Result<DigitsFormat> {
        match self {
            Value::DigitsFormat(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into an optional timing method.
    pub fn into_optional_timing_method(self) -> Result<Option<TimingMethod>> {
        match self {
            Value::OptionalTimingMethod(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into a color.
    pub fn into_color(self) -> Result<Color> {
        match self {
            Value::Color(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into an optional color.
    pub fn into_optional_color(self) -> Result<Option<Color>> {
        match self {
            Value::OptionalColor(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into a gradient.
    pub fn into_gradient(self) -> Result<Gradient> {
        match self {
            Value::Color(v) => Ok(Gradient::Plain(v)),
            Value::Gradient(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into a list gradient.
    pub fn into_list_gradient(self) -> Result<ListGradient> {
        match self {
            Value::Color(v) => Ok(ListGradient::Same(Gradient::Plain(v))),
            Value::Gradient(v) => Ok(ListGradient::Same(v)),
            Value::ListGradient(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into an alignment.
    pub fn into_alignment(self) -> Result<Alignment> {
        match self {
            Value::Alignment(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into a value describing what a splits
    /// component's column starts out with.
    pub fn into_column_start_with(self) -> Result<ColumnStartWith> {
        match self {
            Value::ColumnStartWith(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into a value describing what a splits
    /// component's column gets updated with.
    pub fn into_column_update_with(self) -> Result<ColumnUpdateWith> {
        match self {
            Value::ColumnUpdateWith(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into a Column Update Trigger.
    pub fn into_column_update_trigger(self) -> Result<ColumnUpdateTrigger> {
        match self {
            Value::ColumnUpdateTrigger(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into a hotkey.
    pub fn into_hotkey(self) -> Result<Option<KeyCode>> {
        match self {
            Value::Hotkey(v) => Ok(v),
            Value::String(v) | Value::OptionalString(Some(v)) => {
                v.parse().map_err(|_| Error::WrongType).map(Some)
            }
            Value::OptionalString(None) => Ok(None),
            _ => Err(Error::WrongType),
        }
    }

    /// Tries to convert the value into a layout direction.
    pub fn into_layout_direction(self) -> Result<LayoutDirection> {
        match self {
            Value::LayoutDirection(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }
}

impl Into<bool> for Value {
    fn into(self) -> bool {
        self.into_bool().unwrap()
    }
}

impl Into<u64> for Value {
    fn into(self) -> u64 {
        self.into_uint().unwrap()
    }
}

impl Into<i64> for Value {
    fn into(self) -> i64 {
        self.into_int().unwrap()
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        self.into_string().unwrap()
    }
}

impl Into<Option<String>> for Value {
    fn into(self) -> Option<String> {
        self.into_optional_string().unwrap()
    }
}

impl Into<f64> for Value {
    fn into(self) -> f64 {
        self.into_float().unwrap()
    }
}

impl Into<Accuracy> for Value {
    fn into(self) -> Accuracy {
        self.into_accuracy().unwrap()
    }
}

impl Into<DigitsFormat> for Value {
    fn into(self) -> DigitsFormat {
        self.into_digits_format().unwrap()
    }
}

impl Into<Option<TimingMethod>> for Value {
    fn into(self) -> Option<TimingMethod> {
        self.into_optional_timing_method().unwrap()
    }
}

impl Into<Color> for Value {
    fn into(self) -> Color {
        self.into_color().unwrap()
    }
}

impl Into<Option<Color>> for Value {
    fn into(self) -> Option<Color> {
        self.into_optional_color().unwrap()
    }
}

impl Into<Gradient> for Value {
    fn into(self) -> Gradient {
        self.into_gradient().unwrap()
    }
}

impl Into<ListGradient> for Value {
    fn into(self) -> ListGradient {
        self.into_list_gradient().unwrap()
    }
}

impl Into<Alignment> for Value {
    fn into(self) -> Alignment {
        self.into_alignment().unwrap()
    }
}

impl Into<ColumnStartWith> for Value {
    fn into(self) -> ColumnStartWith {
        self.into_column_start_with().unwrap()
    }
}

impl Into<ColumnUpdateWith> for Value {
    fn into(self) -> ColumnUpdateWith {
        self.into_column_update_with().unwrap()
    }
}

impl Into<ColumnUpdateTrigger> for Value {
    fn into(self) -> ColumnUpdateTrigger {
        self.into_column_update_trigger().unwrap()
    }
}

impl Into<Option<KeyCode>> for Value {
    fn into(self) -> Option<KeyCode> {
        self.into_hotkey().unwrap()
    }
}

impl Into<LayoutDirection> for Value {
    fn into(self) -> LayoutDirection {
        self.into_layout_direction().unwrap()
    }
}
