use livesplit_core::Timer;
use super::{drop, acc};
use std::sync::RwLockReadGuard;
use std::ops::Deref;

pub type TimerReadLock = RwLockReadGuard<'static, Timer>;
pub type OwnedTimerReadLock = *mut TimerReadLock;

#[no_mangle]
pub unsafe extern "C" fn TimerReadLock_drop(this: OwnedTimerReadLock) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TimerReadLock_timer(this: *const TimerReadLock) -> *const Timer {
    acc(this).deref()
}
