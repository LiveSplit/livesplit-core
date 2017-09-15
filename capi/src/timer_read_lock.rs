use livesplit_core::Timer;
use super::{acc, own_drop};
use livesplit_core::parking_lot::RwLockReadGuard;
use std::ops::Deref;

pub type TimerReadLock<'a> = RwLockReadGuard<'a, Timer>;
pub type OwnedTimerReadLock<'a> = *mut TimerReadLock<'a>;

#[no_mangle]
pub unsafe extern "C" fn TimerReadLock_drop(this: OwnedTimerReadLock) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TimerReadLock_timer(this: *const TimerReadLock) -> *const Timer {
    acc(&this).deref()
}
