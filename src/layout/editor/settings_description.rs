use super::Component;
use TimingMethod;
use time_formatter::{DigitsFormat, Accuracy};
use std::result::Result as StdResult;

#[derive(From, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    UInt(u64),
    Int(i64),
    String(String),
    OptionalString(Option<String>),
    Float(f64),
    Accuracy(Accuracy),
    DigitsFormat(DigitsFormat),
    OptionalTimingMethod(Option<TimingMethod>),
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    pub text: String,
    pub value: Value,
}

#[derive(Default, Serialize, Deserialize)]
pub struct SettingsDescription {
    pub fields: Vec<Field>,
}

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        WrongType
    }
}

pub type Result<T> = StdResult<T, Error>;

impl Value {
    pub fn into_bool(self) -> Result<bool> {
        match self {
            Value::Bool(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_uint(self) -> Result<u64> {
        match self {
            Value::UInt(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_int(self) -> Result<i64> {
        match self {
            Value::Int(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_string(self) -> Result<String> {
        match self {
            Value::String(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_optional_string(self) -> Result<Option<String>> {
        match self {
            Value::OptionalString(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_float(self) -> Result<f64> {
        match self {
            Value::Float(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_accuracy(self) -> Result<Accuracy> {
        match self {
            Value::Accuracy(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_digits_format(self) -> Result<DigitsFormat> {
        match self {
            Value::DigitsFormat(v) => Ok(v),
            _ => Err(Error::WrongType),
        }
    }

    pub fn into_optional_timing_method(self) -> Result<Option<TimingMethod>> {
        match self {
            Value::OptionalTimingMethod(v) => Ok(v),
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

impl Field {
    pub fn new(text: String, value: Value) -> Self {
        Self { text, value }
    }
}

impl SettingsDescription {
    pub fn with_fields(fields: Vec<Field>) -> Self {
        Self { fields }
    }
}

impl Component {
    pub fn settings_description(&self) -> SettingsDescription {
        match *self {
            Component::CurrentComparison(ref component) => component.settings_description(),
            Component::CurrentPace(ref component) => component.settings_description(),
            Component::Delta(ref component) => component.settings_description(),
            Component::DetailedTimer(ref component) => component.settings_description(),
            Component::Graph(ref component) => component.settings_description(),
            Component::PossibleTimeSave(ref component) => component.settings_description(),
            Component::PreviousSegment(ref component) => component.settings_description(),
            Component::Separator(ref component) => component.settings_description(),
            Component::Splits(ref component) => component.settings_description(),
            Component::SumOfBest(ref component) => component.settings_description(),
            Component::Text(ref component) => component.settings_description(),
            Component::Timer(ref component) => component.settings_description(),
            Component::Title(ref component) => component.settings_description(),
            Component::TotalPlaytime(ref component) => component.settings_description(),
        }
    }

    pub fn set_value(&mut self, index: usize, value: Value) {
        match *self {
            Component::CurrentComparison(ref mut component) => component.set_value(index, value),
            Component::CurrentPace(ref mut component) => component.set_value(index, value),
            Component::Delta(ref mut component) => component.set_value(index, value),
            Component::DetailedTimer(ref mut component) => component.set_value(index, value),
            Component::Graph(ref mut component) => component.set_value(index, value),
            Component::PossibleTimeSave(ref mut component) => component.set_value(index, value),
            Component::PreviousSegment(ref mut component) => component.set_value(index, value),
            Component::Separator(ref mut component) => component.set_value(index, value),
            Component::Splits(ref mut component) => component.set_value(index, value),
            Component::SumOfBest(ref mut component) => component.set_value(index, value),
            Component::Text(ref mut component) => component.set_value(index, value),
            Component::Timer(ref mut component) => component.set_value(index, value),
            Component::Title(ref mut component) => component.set_value(index, value),
            Component::TotalPlaytime(ref mut component) => component.set_value(index, value),
        }
    }
}
