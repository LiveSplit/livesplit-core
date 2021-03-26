use crate::{
    component::splits::{ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith},
    hotkey::KeyCode,
    layout::LayoutDirection,
    platform::prelude::*,
    settings::{Alignment, Color, Font, Gradient, ListGradient},
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
    /// A value describing a font to use. `None` if a default font should be
    /// used.
    Font(Option<Font>),
}

/// The Error type for values that couldn't be converted.
#[derive(Debug, snafu::Snafu)]
pub enum Error {
    /// The value couldn't be converted because it had an incompatible type.
    WrongType,
}

/// The Result type for conversions from Values to other types.
pub type Result<T> = StdResult<T, Error>;

#[allow(clippy::missing_const_for_fn)] // FIXME: Drop is unsupported.
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

    /// Tries to convert the value into a font.
    pub fn into_font(self) -> Result<Option<Font>> {
        match self {
            Value::Font(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        value.into_bool().unwrap()
    }
}

impl From<Value> for u64 {
    fn from(value: Value) -> Self {
        value.into_uint().unwrap()
    }
}

impl From<Value> for i64 {
    fn from(value: Value) -> Self {
        value.into_int().unwrap()
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        value.into_string().unwrap()
    }
}

impl From<Value> for Option<String> {
    fn from(value: Value) -> Self {
        value.into_optional_string().unwrap()
    }
}

impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        value.into_float().unwrap()
    }
}

impl From<Value> for Accuracy {
    fn from(value: Value) -> Self {
        value.into_accuracy().unwrap()
    }
}

impl From<Value> for DigitsFormat {
    fn from(value: Value) -> Self {
        value.into_digits_format().unwrap()
    }
}

impl From<Value> for Option<TimingMethod> {
    fn from(value: Value) -> Self {
        value.into_optional_timing_method().unwrap()
    }
}

impl From<Value> for Color {
    fn from(value: Value) -> Self {
        value.into_color().unwrap()
    }
}

impl From<Value> for Option<Color> {
    fn from(value: Value) -> Self {
        value.into_optional_color().unwrap()
    }
}

impl From<Value> for Gradient {
    fn from(value: Value) -> Self {
        value.into_gradient().unwrap()
    }
}

impl From<Value> for ListGradient {
    fn from(value: Value) -> Self {
        value.into_list_gradient().unwrap()
    }
}

impl From<Value> for Alignment {
    fn from(value: Value) -> Self {
        value.into_alignment().unwrap()
    }
}

impl From<Value> for ColumnStartWith {
    fn from(value: Value) -> Self {
        value.into_column_start_with().unwrap()
    }
}

impl From<Value> for ColumnUpdateWith {
    fn from(value: Value) -> Self {
        value.into_column_update_with().unwrap()
    }
}

impl From<Value> for ColumnUpdateTrigger {
    fn from(value: Value) -> Self {
        value.into_column_update_trigger().unwrap()
    }
}

impl From<Value> for Option<KeyCode> {
    fn from(value: Value) -> Self {
        value.into_hotkey().unwrap()
    }
}

impl From<Value> for LayoutDirection {
    fn from(value: Value) -> Self {
        value.into_layout_direction().unwrap()
    }
}

impl From<Value> for Option<Font> {
    fn from(value: Value) -> Self {
        value.into_font().unwrap()
    }
}
