use livesplit_core::Timer;
use super::{acc_mut, own_drop};
use livesplit_core::parking_lot::RwLockWriteGuard;
use std::ops::DerefMut;

pub type TimerWriteLock = RwLockWriteGuard<'static, Timer>;
pub type OwnedTimerWriteLock = *mut TimerWriteLock;

#[no_mangle]
pub unsafe extern "C" fn TimerWriteLock_drop(this: OwnedTimerWriteLock) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TimerWriteLock_timer(this: *mut TimerWriteLock) -> *mut Timer {
    acc_mut(&this).deref_mut()
}
