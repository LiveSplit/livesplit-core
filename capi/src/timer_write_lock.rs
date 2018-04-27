//! A Timer Write Lock allows temporary write access to a timer. Dispose this to
//! release the write lock.

use super::{acc_mut, own_drop};
use livesplit_core::Timer;
use livesplit_core::parking_lot::RwLockWriteGuard;
use std::ops::DerefMut;

/// type
pub type TimerWriteLock = RwLockWriteGuard<'static, Timer>;
/// type
pub type OwnedTimerWriteLock = *mut TimerWriteLock;

/// drop
#[no_mangle]
pub unsafe extern "C" fn TimerWriteLock_drop(this: OwnedTimerWriteLock) {
    own_drop(this);
}

/// Accesses the timer.
#[no_mangle]
pub unsafe extern "C" fn TimerWriteLock_timer(this: *mut TimerWriteLock) -> *mut Timer {
    acc_mut(this).deref_mut()
}
