use livesplit_core::HotkeySystem;
use shared_timer::OwnedSharedTimer;
use super::{alloc, own, own_drop};
use std::ptr;

pub type OwnedHotkeySystem = *mut HotkeySystem;

#[no_mangle]
pub unsafe extern "C" fn HotkeySystem_new(shared_timer: OwnedSharedTimer) -> OwnedHotkeySystem {
    if let Ok(hotkey_system) = HotkeySystem::new(own(shared_timer)) {
        alloc(hotkey_system)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn HotkeySystem_drop(this: OwnedHotkeySystem) {
    own_drop(this);
}
