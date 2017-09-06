use livesplit_core::SharedTimer;
use timer::OwnedTimer;
use timer_read_lock::OwnedTimerReadLock;
use timer_write_lock::OwnedTimerWriteLock;
use super::{acc, alloc, own, own_drop};

pub type OwnedSharedTimer = *mut SharedTimer;

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_share(this: *const SharedTimer) -> OwnedSharedTimer {
    alloc(acc(&this).clone())
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_drop(this: OwnedSharedTimer) {
    own_drop(this);
}

/// # Safety
/// `this` must outlive `OwnedTimerReadLock`
#[no_mangle]
pub unsafe extern "C" fn SharedTimer_read<'a>(this: *const SharedTimer) -> OwnedTimerReadLock<'a> {
    alloc((&*this).read())
}

/// # Safety
/// `this` must outlive `OwnedTimerWriteLock`
#[no_mangle]
pub unsafe extern "C" fn SharedTimer_write<'a>(this: *const SharedTimer) -> OwnedTimerWriteLock<'a> {
    alloc((&*this).write())
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_replace_inner(this: *const SharedTimer, timer: OwnedTimer) {
    *acc(&this).write() = own(timer);
}
