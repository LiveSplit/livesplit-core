use livesplit_core::HotkeyTimer;
use timer::OwnedTimer;
use timer_read_lock::OwnedTimerReadLock;
use timer_write_lock::OwnedTimerWriteLock;
use super::{alloc, own, acc, drop};
use std::ptr;

pub type OwnedHotkeyTimer = *mut HotkeyTimer;

#[no_mangle]
pub unsafe extern "C" fn HotkeyTimer_new(timer: OwnedTimer) -> OwnedHotkeyTimer {
    if let Ok(timer) = HotkeyTimer::new(own(timer)) {
        alloc(timer)
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn HotkeyTimer_drop(this: OwnedHotkeyTimer) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn HotkeyTimer_read(this: *const HotkeyTimer) -> OwnedTimerReadLock {
    alloc(acc(this).read())
}

#[no_mangle]
pub unsafe extern "C" fn HotkeyTimer_write(this: *const HotkeyTimer) -> OwnedTimerWriteLock {
    alloc(acc(this).write())
}
