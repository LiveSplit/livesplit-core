use livesplit_core::settings::{Value as SettingValue, Color};
use livesplit_core::time_formatter::{Accuracy, DigitsFormat};
use livesplit_core::TimingMethod;
use {alloc, str, own_drop};
use libc::c_char;
use std::ptr;

pub type OwnedSettingValue = *mut SettingValue;

#[no_mangle]
pub unsafe extern "C" fn SettingValue_drop(this: OwnedSettingValue) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_bool(value: bool) -> OwnedSettingValue {
    alloc(value.into())
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_uint(value: u64) -> OwnedSettingValue {
    alloc(value.into())
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_int(value: i64) -> OwnedSettingValue {
    alloc(value.into())
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_string(value: *const c_char) -> OwnedSettingValue {
    alloc(str(value).to_string().into())
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_optional_string(
    value: *const c_char,
) -> OwnedSettingValue {
    let value = if value.is_null() {
        None::<String>.into()
    } else {
        Some(str(value).to_string()).into()
    };
    alloc(value)
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_optional_empty_string() -> OwnedSettingValue {
    alloc(None::<String>.into())
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_float(value: f64) -> OwnedSettingValue {
    alloc(value.into())
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_accuracy(value: *const c_char) -> OwnedSettingValue {
    let value = str(value);
    let value = match value {
        "Seconds" => Accuracy::Seconds,
        "Tenths" => Accuracy::Tenths,
        "Hundredths" => Accuracy::Hundredths,
        _ => return ptr::null_mut(),
    };
    alloc(value.into())
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_digits_format(
    value: *const c_char,
) -> OwnedSettingValue {
    let value = str(value);
    let value = match value {
        "SingleDigitSeconds" => DigitsFormat::SingleDigitSeconds,
        "DoubleDigitSeconds" => DigitsFormat::DoubleDigitSeconds,
        "SingleDigitMinutes" => DigitsFormat::SingleDigitMinutes,
        "DoubleDigitMinutes" => DigitsFormat::DoubleDigitMinutes,
        "SingleDigitHours" => DigitsFormat::SingleDigitHours,
        "DoubleDigitHours" => DigitsFormat::DoubleDigitHours,
        _ => return ptr::null_mut(),
    };
    alloc(value.into())
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_optional_timing_method(
    value: *const c_char,
) -> OwnedSettingValue {
    if value.is_null() {
        alloc(None::<TimingMethod>.into())
    } else {
        let value = str(value);
        let value = match value {
            "RealTime" => TimingMethod::RealTime,
            "GameTime" => TimingMethod::GameTime,
            _ => return ptr::null_mut(),
        };
        alloc(Some(value).into())
    }
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_optional_empty_timing_method() -> OwnedSettingValue {
    alloc(None::<TimingMethod>.into())
}

#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_color(
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) -> OwnedSettingValue {
    alloc(Color::from((r, g, b, a)).into())
}
