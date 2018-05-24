//! Describes a setting's value. Such a value can be of a variety of different
//! types.

use livesplit_core::settings::{Alignment, Color, Gradient, Value as SettingValue};
use livesplit_core::time::formatter::{Accuracy, DigitsFormat};
use livesplit_core::TimingMethod;
use std::os::raw::c_char;
use str;

/// type
pub type OwnedSettingValue = Box<SettingValue>;
/// type
pub type NullableOwnedSettingValue = Option<OwnedSettingValue>;

/// drop
#[no_mangle]
pub extern "C" fn SettingValue_drop(this: OwnedSettingValue) {
    drop(this);
}

/// Creates a new setting value from a boolean value.
#[no_mangle]
pub extern "C" fn SettingValue_from_bool(value: bool) -> OwnedSettingValue {
    Box::new(value.into())
}

/// Creates a new setting value from an unsigned integer.
#[no_mangle]
pub extern "C" fn SettingValue_from_uint(value: u32) -> OwnedSettingValue {
    Box::new((value as u64).into())
}

/// Creates a new setting value from a signed integer.
#[no_mangle]
pub extern "C" fn SettingValue_from_int(value: i32) -> OwnedSettingValue {
    Box::new((value as i64).into())
}

/// Creates a new setting value from a string.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_string(value: *const c_char) -> OwnedSettingValue {
    Box::new(str(value).to_string().into())
}

/// Creates a new setting value from a string that has the type `optional string`.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_optional_string(
    value: *const c_char,
) -> OwnedSettingValue {
    let value = if value.is_null() {
        None::<String>.into()
    } else {
        Some(str(value).to_string()).into()
    };
    Box::new(value)
}

/// Creates a new empty setting value that has the type `optional string`.
#[no_mangle]
pub extern "C" fn SettingValue_from_optional_empty_string() -> OwnedSettingValue {
    Box::new(None::<String>.into())
}

/// Creates a new setting value from a floating point number.
#[no_mangle]
pub extern "C" fn SettingValue_from_float(value: f64) -> OwnedSettingValue {
    Box::new(value.into())
}

/// Creates a new setting value from an accuracy name. If it doesn't match a
/// known accuracy, <NULL> is returned.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_accuracy(
    value: *const c_char,
) -> NullableOwnedSettingValue {
    let value = str(value);
    let value = match value {
        "Seconds" => Accuracy::Seconds,
        "Tenths" => Accuracy::Tenths,
        "Hundredths" => Accuracy::Hundredths,
        "Milliseconds" => Accuracy::Milliseconds,
        _ => return None,
    };
    Some(Box::new(value.into()))
}

/// Creates a new setting value from a digits format name. If it doesn't match a
/// known digits format, <NULL> is returned.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_digits_format(
    value: *const c_char,
) -> NullableOwnedSettingValue {
    let value = str(value);
    let value = match value {
        "SingleDigitSeconds" => DigitsFormat::SingleDigitSeconds,
        "DoubleDigitSeconds" => DigitsFormat::DoubleDigitSeconds,
        "SingleDigitMinutes" => DigitsFormat::SingleDigitMinutes,
        "DoubleDigitMinutes" => DigitsFormat::DoubleDigitMinutes,
        "SingleDigitHours" => DigitsFormat::SingleDigitHours,
        "DoubleDigitHours" => DigitsFormat::DoubleDigitHours,
        _ => return None,
    };
    Some(Box::new(value.into()))
}

/// Creates a new setting value from a timing method name with the type
/// `optional timing method`. If it doesn't match a known timing method, <NULL>
/// is returned.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_optional_timing_method(
    value: *const c_char,
) -> NullableOwnedSettingValue {
    if value.is_null() {
        Some(Box::new(None::<TimingMethod>.into()))
    } else {
        let value = str(value);
        let value = match value {
            "RealTime" => TimingMethod::RealTime,
            "GameTime" => TimingMethod::GameTime,
            _ => return None,
        };
        Some(Box::new(Some(value).into()))
    }
}

/// Creates a new empty setting value with the type `optional timing method`.
#[no_mangle]
pub extern "C" fn SettingValue_from_optional_empty_timing_method() -> OwnedSettingValue {
    Box::new(None::<TimingMethod>.into())
}

/// Creates a new setting value from the color provided as RGBA.
#[no_mangle]
pub extern "C" fn SettingValue_from_color(r: f32, g: f32, b: f32, a: f32) -> OwnedSettingValue {
    Box::new(Color::from((r, g, b, a)).into())
}

/// Creates a new setting value from the color provided as RGBA with the type
/// `optional color`.
#[no_mangle]
pub extern "C" fn SettingValue_from_optional_color(
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) -> OwnedSettingValue {
    Box::new(Some(Color::from((r, g, b, a))).into())
}

/// Creates a new empty setting value with the type `optional color`.
#[no_mangle]
pub extern "C" fn SettingValue_from_optional_empty_color() -> OwnedSettingValue {
    Box::new(None::<Color>.into())
}

/// Creates a new setting value that is a transparent gradient.
#[no_mangle]
pub extern "C" fn SettingValue_from_transparent_gradient() -> OwnedSettingValue {
    Box::new(Gradient::Transparent.into())
}

/// Creates a new setting value from the vertical gradient provided as two RGBA colors.
#[no_mangle]
pub extern "C" fn SettingValue_from_vertical_gradient(
    r1: f32,
    g1: f32,
    b1: f32,
    a1: f32,
    r2: f32,
    g2: f32,
    b2: f32,
    a2: f32,
) -> OwnedSettingValue {
    Box::new(
        Gradient::Vertical(Color::from((r1, g1, b1, a1)), Color::from((r2, g2, b2, a2))).into(),
    )
}

/// Creates a new setting value from the horizontal gradient provided as two RGBA colors.
#[no_mangle]
pub extern "C" fn SettingValue_from_horizontal_gradient(
    r1: f32,
    g1: f32,
    b1: f32,
    a1: f32,
    r2: f32,
    g2: f32,
    b2: f32,
    a2: f32,
) -> OwnedSettingValue {
    Box::new(
        Gradient::Horizontal(Color::from((r1, g1, b1, a1)), Color::from((r2, g2, b2, a2))).into(),
    )
}

/// Creates a new setting value from the alignment name provided. If it doesn't
/// match a known alignment, <NULL> is returned.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_alignment(
    value: *const c_char,
) -> NullableOwnedSettingValue {
    let value = str(value);
    let value = match value {
        "Center" => Alignment::Center,
        "Left" => Alignment::Left,
        "Auto" => Alignment::Auto,
        _ => return None,
    };
    Some(Box::new(value.into()))
}
