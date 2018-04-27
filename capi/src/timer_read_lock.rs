//! A Timer Read Lock allows temporary read access to a timer. Dispose this to
//! release the read lock.

use super::{acc, own_drop};
use livesplit_core::Timer;
use livesplit_core::parking_lot::RwLockReadGuard;
use std::ops::Deref;

/// type
pub type TimerReadLock = RwLockReadGuard<'static, Timer>;
/// type
pub type OwnedTimerReadLock = *mut TimerReadLock;

/// drop
#[no_mangle]
pub unsafe extern "C" fn TimerReadLock_drop(this: OwnedTimerReadLock) {
    own_drop(this);
}

/// Accesses the timer.
#[no_mangle]
pub unsafe extern "C" fn TimerReadLock_timer(this: *const TimerReadLock) -> *const Timer {
    acc(this).deref()
}
