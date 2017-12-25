use TimingMethod;
use super::{Alignment, Color, Gradient};
use time::formatter::{Accuracy, DigitsFormat};
use std::result::Result as StdResult;

/// Describes a setting's value. Such a value can be of a variety of different
/// types.
#[derive(From, Serialize, Deserialize)]
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
    /// An alignment for the Title Component's title.
    Alignment(Alignment),
}

quick_error! {
    /// The Error type for values that couldn't be converted.
    #[derive(Debug)]
    pub enum Error {
        /// The value couldn't be converted because it had an incompatible type.
        WrongType {}
    }
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

    /// Tries to convert the value into an alignment.
    pub fn into_alignment(self) -> Result<Alignment> {
        match self {
            Value::Alignment(v) => Ok(v),
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

impl Into<Alignment> for Value {
    fn into(self) -> Alignment {
        self.into_alignment().unwrap()
    }
}
