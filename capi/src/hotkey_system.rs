//! With a Hotkey System the runner can use hotkeys on their keyboard to control
//! the Timer. The hotkeys are global, so the application doesn't need to be in
//! focus. The behavior of the hotkeys depends on the platform and is stubbed
//! out on platforms that don't support hotkeys. You can turn off a Hotkey
//! System temporarily. By default the Hotkey System is activated.

use std::{os::raw::c_char, str::FromStr};

use crate::{event_sink::EventSink, hotkey_config::OwnedHotkeyConfig, output_str, str};
use livesplit_core::hotkey::KeyCode;

type HotkeySystem = livesplit_core::HotkeySystem<EventSink>;

/// type
pub type OwnedHotkeySystem = Box<HotkeySystem>;
/// type
pub type NullableOwnedHotkeySystem = Option<OwnedHotkeySystem>;

/// Creates a new Hotkey System for a Timer with the default hotkeys.
#[no_mangle]
pub extern "C" fn HotkeySystem_new(event_sink: &EventSink) -> NullableOwnedHotkeySystem {
    HotkeySystem::new(event_sink.clone()).ok().map(Box::new)
}

/// Creates a new Hotkey System for a Timer with a custom configuration for the
/// hotkeys.
#[no_mangle]
pub extern "C" fn HotkeySystem_with_config(
    event_sink: &EventSink,
    config: OwnedHotkeyConfig,
) -> NullableOwnedHotkeySystem {
    HotkeySystem::with_config(event_sink.clone(), *config)
        .ok()
        .map(Box::new)
}

/// drop
#[no_mangle]
pub extern "C" fn HotkeySystem_drop(this: OwnedHotkeySystem) {
    drop(this);
}

/// Deactivates the Hotkey System. No hotkeys will go through until it gets
/// activated again. If it's already deactivated, nothing happens.
#[no_mangle]
pub extern "C" fn HotkeySystem_deactivate(this: &mut HotkeySystem) -> bool {
    this.deactivate().is_ok()
}

/// Activates a previously deactivated Hotkey System. If it's already
/// active, nothing happens.
#[no_mangle]
pub extern "C" fn HotkeySystem_activate(this: &mut HotkeySystem) -> bool {
    this.activate().is_ok()
}

/// Returns the hotkey configuration currently in use by the Hotkey System.
#[no_mangle]
pub extern "C" fn HotkeySystem_config(this: &HotkeySystem) -> OwnedHotkeyConfig {
    Box::new(this.config())
}

/// Applies a new hotkey configuration to the Hotkey System. Each hotkey is
/// changed to the one specified in the configuration. This operation may fail
/// if you provide a hotkey configuration where a hotkey is used for multiple
/// operations. Returns <FALSE> if the operation failed.
#[no_mangle]
pub extern "C" fn HotkeySystem_set_config(
    this: &mut HotkeySystem,
    config: OwnedHotkeyConfig,
) -> bool {
    this.set_config(*config).is_ok()
}

/// Resolves the key according to the current keyboard layout.
#[no_mangle]
pub unsafe extern "C" fn HotkeySystem_resolve(
    this: &HotkeySystem,
    key_code: *const c_char,
) -> *const c_char {
    let name = str(key_code);
    match KeyCode::from_str(name) {
        Ok(key_code) => output_str(this.resolve(key_code)),
        _ => output_str(name),
    }
}
