//! The configuration to use for a Hotkey System. It describes with keys to use
//! as hotkeys for the different actions.

use super::{Json, get_file, output_vec, str};
use crate::setting_value::OwnedSettingValue;
use livesplit_core::{HotkeyConfig, Lang};
use std::io::{BufReader, Cursor};

/// type
pub type OwnedHotkeyConfig = Box<HotkeyConfig>;
/// type
pub type NullableOwnedHotkeyConfig = Option<OwnedHotkeyConfig>;

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn HotkeyConfig_drop(this: OwnedHotkeyConfig) {
    drop(this);
}

/// Creates a new Hotkey Configuration with default settings.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeyConfig_new() -> OwnedHotkeyConfig {
    Default::default()
}

/// Encodes generic description of the settings available for the hotkey
/// configuration and their current values as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeyConfig_settings_description_as_json(
    this: &HotkeyConfig,
    lang: Lang,
) -> Json {
    output_vec(|o| {
        serde_json::to_writer(o, &this.settings_description(lang)).unwrap();
    })
}

/// Sets a setting's value by its index to the given value.
///
/// <FALSE> is returned if a hotkey is already in use by a different action.
///
/// This panics if the type of the value to be set is not compatible with the
/// type of the setting's value. A panic can also occur if the index of the
/// setting provided is out of bounds.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeyConfig_set_value(
    this: &mut HotkeyConfig,
    index: usize,
    value: OwnedSettingValue,
) -> bool {
    this.set_value(index, *value).is_ok()
}

/// Encodes the hotkey configuration as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeyConfig_as_json(this: &HotkeyConfig) -> Json {
    output_vec(|o| {
        this.write_json(o).unwrap();
    })
}

/// Parses a hotkey configuration from the given JSON description. <NULL> is
/// returned if it couldn't be parsed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HotkeyConfig_parse_json(settings: Json) -> NullableOwnedHotkeyConfig {
    // SAFETY: The caller guarantees that `settings` is valid JSON.
    let settings = Cursor::new(unsafe { str(settings).as_bytes() });
    HotkeyConfig::from_json(settings).ok().map(Box::new)
}

/// Attempts to parse a hotkey configuration from a given file. <NULL> is
/// returned if it couldn't be parsed. This will not close the file descriptor /
/// handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HotkeyConfig_parse_file_handle(handle: i64) -> NullableOwnedHotkeyConfig {
    // SAFETY: The caller guarantees that `handle` is a valid file handle.
    let file = unsafe { get_file(handle) };

    let reader = BufReader::new(&*file);

    HotkeyConfig::from_json(reader).ok().map(Box::new)
}
