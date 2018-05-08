//! With a Hotkey System the runner can use hotkeys on their keyboard to control
//! the Timer. The hotkeys are global, so the application doesn't need to be in
//! focus. The behavior of the hotkeys depends on the platform and is stubbed
//! out on platforms that don't support hotkeys. You can turn off a Hotkey
//! System temporarily. By default the Hotkey System is activated.

use livesplit_core::HotkeySystem;
use shared_timer::OwnedSharedTimer;

/// type
pub type OwnedHotkeySystem = Box<HotkeySystem>;
/// type
pub type NullableOwnedHotkeySystem = Option<OwnedHotkeySystem>;

/// Creates a new Hotkey System for a Timer with the default hotkeys.
#[no_mangle]
pub extern "C" fn HotkeySystem_new(shared_timer: OwnedSharedTimer) -> NullableOwnedHotkeySystem {
    if let Ok(hotkey_system) = HotkeySystem::new(*shared_timer) {
        Some(Box::new(hotkey_system))
    } else {
        None
    }
}

/// drop
#[no_mangle]
pub extern "C" fn HotkeySystem_drop(this: OwnedHotkeySystem) {
    drop(this);
}

/// Deactivates the Hotkey System. No hotkeys will go through until it gets
/// activated again. If it's already deactivated, nothing happens.
#[no_mangle]
pub extern "C" fn HotkeySystem_deactivate(this: &HotkeySystem) {
    this.deactivate();
}

/// Activates a previously deactivated Hotkey System. If it's already
/// active, nothing happens.
#[no_mangle]
pub extern "C" fn HotkeySystem_activate(this: &HotkeySystem) {
    this.activate();
}
