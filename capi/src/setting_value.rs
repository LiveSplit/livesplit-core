//! Describes a setting's value. Such a value can be of a variety of different
//! types.

use crate::{output_vec, str, Json};
use livesplit_core::{
    component::{
        splits::{ColumnStartWith, ColumnUpdateTrigger, ColumnUpdateWith},
        timer::DeltaGradient,
    },
    layout::LayoutDirection,
    settings::{
        Alignment, BackgroundImage, Color, ColumnKind, Font, FontStretch, FontStyle, FontWeight,
        Gradient, ImageId, LayoutBackground, ListGradient, Value as SettingValue,
    },
    timing::formatter::{Accuracy, DigitsFormat},
    TimingMethod,
};
use std::{os::raw::c_char, str::FromStr};

/// type
pub type OwnedSettingValue = Box<SettingValue>;
/// type
pub type NullableOwnedSettingValue = Option<OwnedSettingValue>;

/// drop
#[no_mangle]
pub extern "C" fn SettingValue_drop(this: OwnedSettingValue) {
    drop(this);
}

/// Encodes this Setting Value's state as JSON.
#[no_mangle]
pub extern "C" fn SettingValue_as_json(this: &SettingValue) -> Json {
    output_vec(|o| {
        serde_json::to_writer(o, this).unwrap();
    })
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
    Box::new(Color::rgba(r, g, b, a).into())
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
    Box::new(Some(Color::rgba(r, g, b, a)).into())
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
    Box::new(Gradient::Vertical(Color::rgba(r1, g1, b1, a1), Color::rgba(r2, g2, b2, a2)).into())
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
    Box::new(Gradient::Horizontal(Color::rgba(r1, g1, b1, a1), Color::rgba(r2, g2, b2, a2)).into())
}

/// Creates a new setting value from the alternating gradient provided as two RGBA colors.
#[no_mangle]
pub extern "C" fn SettingValue_from_alternating_gradient(
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
        ListGradient::Alternating(Color::rgba(r1, g1, b1, a1), Color::rgba(r2, g2, b2, a2)).into(),
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

/// Creates a new setting value from the column kind with the name provided. If
/// it doesn't match a known column kind, <NULL> is returned.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_column_kind(
    value: *const c_char,
) -> NullableOwnedSettingValue {
    let value = str(value);
    let value = match value {
        "Time" => ColumnKind::Time,
        "Variable" => ColumnKind::Variable,
        _ => return None,
    };
    Some(Box::new(value.into()))
}

/// Creates a new setting value from the column start with the name provided. If
/// it doesn't match a known column start with, <NULL> is returned.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_column_start_with(
    value: *const c_char,
) -> NullableOwnedSettingValue {
    let value = str(value);
    let value = match value {
        "Empty" => ColumnStartWith::Empty,
        "ComparisonTime" => ColumnStartWith::ComparisonTime,
        "ComparisonSegmentTime" => ColumnStartWith::ComparisonSegmentTime,
        "PossibleTimeSave" => ColumnStartWith::PossibleTimeSave,
        _ => return None,
    };
    Some(Box::new(value.into()))
}

/// Creates a new setting value from the column update with the name provided.
/// If it doesn't match a known column update with, <NULL> is returned.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_column_update_with(
    value: *const c_char,
) -> NullableOwnedSettingValue {
    let value = str(value);
    let value = match value {
        "DontUpdate" => ColumnUpdateWith::DontUpdate,
        "SplitTime" => ColumnUpdateWith::SplitTime,
        "Delta" => ColumnUpdateWith::Delta,
        "DeltaWithFallback" => ColumnUpdateWith::DeltaWithFallback,
        "SegmentTime" => ColumnUpdateWith::SegmentTime,
        "SegmentDelta" => ColumnUpdateWith::SegmentDelta,
        "SegmentDeltaWithFallback" => ColumnUpdateWith::SegmentDeltaWithFallback,
        _ => return None,
    };
    Some(Box::new(value.into()))
}

/// Creates a new setting value from the column update trigger. If it doesn't
/// match a known column update trigger, <NULL> is returned.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_column_update_trigger(
    value: *const c_char,
) -> NullableOwnedSettingValue {
    let value = str(value);
    let value = match value {
        "OnStartingSegment" => ColumnUpdateTrigger::OnStartingSegment,
        "Contextual" => ColumnUpdateTrigger::Contextual,
        "OnEndingSegment" => ColumnUpdateTrigger::OnEndingSegment,
        _ => return None,
    };
    Some(Box::new(value.into()))
}

/// Creates a new setting value from the layout direction. If it doesn't
/// match a known layout direction, <NULL> is returned.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_layout_direction(
    value: *const c_char,
) -> NullableOwnedSettingValue {
    let value = str(value);
    let value = match value {
        "Vertical" => LayoutDirection::Vertical,
        "Horizontal" => LayoutDirection::Horizontal,
        _ => return None,
    };
    Some(Box::new(value.into()))
}

/// Creates a new setting value with the type `font`.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_font(
    family: *const c_char,
    style: *const c_char,
    weight: *const c_char,
    stretch: *const c_char,
) -> NullableOwnedSettingValue {
    Some(Box::new(
        Some(Font {
            family: str(family).to_owned(),
            style: match str(style) {
                "normal" => FontStyle::Normal,
                "italic" => FontStyle::Italic,
                _ => return None,
            },
            weight: match str(weight) {
                "thin" => FontWeight::Thin,
                "extra-light" => FontWeight::ExtraLight,
                "light" => FontWeight::Light,
                "semi-light" => FontWeight::SemiLight,
                "normal" => FontWeight::Normal,
                "medium" => FontWeight::Medium,
                "semi-bold" => FontWeight::SemiBold,
                "bold" => FontWeight::Bold,
                "extra-bold" => FontWeight::ExtraBold,
                "black" => FontWeight::Black,
                "extra-black" => FontWeight::ExtraBlack,
                _ => return None,
            },
            stretch: match str(stretch) {
                "ultra-condensed" => FontStretch::UltraCondensed,
                "extra-condensed" => FontStretch::ExtraCondensed,
                "condensed" => FontStretch::Condensed,
                "semi-condensed" => FontStretch::SemiCondensed,
                "normal" => FontStretch::Normal,
                "semi-expanded" => FontStretch::SemiExpanded,
                "expanded" => FontStretch::Expanded,
                "extra-expanded" => FontStretch::ExtraExpanded,
                "ultra-expanded" => FontStretch::UltraExpanded,
                _ => return None,
            },
        })
        .into(),
    ))
}

/// Creates a new empty setting value with the type `font`.
#[no_mangle]
pub extern "C" fn SettingValue_from_empty_font() -> OwnedSettingValue {
    Box::new(None::<Font>.into())
}

/// Creates a new setting value from the delta gradient with the name provided.
/// If it doesn't match a known delta gradient, <NULL> is returned.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_delta_gradient(
    value: *const c_char,
) -> NullableOwnedSettingValue {
    let value = str(value);
    let value = match value {
        "DeltaPlain" => DeltaGradient::DeltaPlain,
        "DeltaVertical" => DeltaGradient::DeltaVertical,
        "DeltaHorizontal" => DeltaGradient::DeltaHorizontal,
        _ => return None,
    };
    Some(Box::new(value.into()))
}

/// Creates a new setting value from the background image with the image ID and
/// the brightness, opacity, and blur provided. If the image ID is invalid,
/// <NULL> is returned.
#[no_mangle]
pub unsafe extern "C" fn SettingValue_from_background_image(
    image_id: *const c_char,
    brightness: f32,
    opacity: f32,
    blur: f32,
) -> NullableOwnedSettingValue {
    Some(Box::new(
        LayoutBackground::Image(BackgroundImage {
            image: ImageId::from_str(str(image_id)).ok()?,
            brightness,
            opacity,
            blur,
        })
        .into(),
    ))
}
