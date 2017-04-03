use livesplit_core::Timer;
use super::{drop, acc_mut};
use std::sync::RwLockWriteGuard;
use std::ops::DerefMut;

pub type TimerWriteLock = RwLockWriteGuard<'static, Timer>;
pub type OwnedTimerWriteLock = *mut TimerWriteLock;

#[no_mangle]
pub unsafe extern "C" fn TimerWriteLock_drop(this: OwnedTimerWriteLock) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TimerWriteLock_timer(this: *mut TimerWriteLock) -> *mut Timer {
    acc_mut(this).deref_mut()
}
