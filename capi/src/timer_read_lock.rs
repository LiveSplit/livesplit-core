//! A Timer Read Lock allows temporary read access to a timer. Dispose this to
//! release the read lock.

use livesplit_core::parking_lot::RwLockReadGuard;
use livesplit_core::Timer;

/// type
pub type TimerReadLock = RwLockReadGuard<'static, Timer>;
/// type
pub type OwnedTimerReadLock = Box<TimerReadLock>;

/// drop
#[no_mangle]
pub extern "C" fn TimerReadLock_drop(this: OwnedTimerReadLock) {
    drop(this);
}

/// Accesses the timer.
#[no_mangle]
pub extern "C" fn TimerReadLock_timer(this: &TimerReadLock) -> &Timer {
    &*this
}
