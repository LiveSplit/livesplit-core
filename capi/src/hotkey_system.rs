//! With a Hotkey System the runner can use hotkeys on their keyboard to control
//! the Timer. The hotkeys are global, so the application doesn't need to be in
//! focus. The behavior of the hotkeys depends on the platform and is stubbed
//! out on platforms that don't support hotkeys. You can turn off a Hotkey
//! System temporarily. By default the Hotkey System is activated.

use livesplit_core::HotkeySystem;
use shared_timer::OwnedSharedTimer;
use super::{acc, alloc, own, own_drop};
use std::ptr;

/// type
pub type OwnedHotkeySystem = *mut HotkeySystem;
/// type
pub type NullableOwnedHotkeySystem = OwnedHotkeySystem;

/// Creates a new Hotkey System for a Timer with the default hotkeys.
#[no_mangle]
pub unsafe extern "C" fn HotkeySystem_new(
    shared_timer: OwnedSharedTimer,
) -> NullableOwnedHotkeySystem {
    if let Ok(hotkey_system) = HotkeySystem::new(own(shared_timer)) {
        alloc(hotkey_system)
    } else {
        ptr::null_mut()
    }
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn HotkeySystem_drop(this: OwnedHotkeySystem) {
    own_drop(this);
}

/// Deactivates the Hotkey System. No hotkeys will go through until it gets
/// activated again. If it's already deactivated, nothing happens.
#[no_mangle]
pub unsafe extern "C" fn HotkeySystem_deactivate(this: *const HotkeySystem) {
    acc(this).deactivate();
}

/// Activates a previously deactivated Hotkey System. If it's already
/// active, nothing happens.
#[no_mangle]
pub unsafe extern "C" fn HotkeySystem_activate(this: *const HotkeySystem) {
    acc(this).activate();
}
