//! A Timer Write Lock allows temporary write access to a timer. Dispose this to
//! release the write lock.

use livesplit_core::parking_lot::RwLockWriteGuard;
use livesplit_core::Timer;

/// type
pub type TimerWriteLock = RwLockWriteGuard<'static, Timer>;
/// type
pub type OwnedTimerWriteLock = Box<TimerWriteLock>;

/// drop
#[no_mangle]
pub extern "C" fn TimerWriteLock_drop(this: OwnedTimerWriteLock) {
    drop(this);
}

/// Accesses the timer.
#[no_mangle]
pub extern "C" fn TimerWriteLock_timer(this: &mut TimerWriteLock) -> &mut Timer {
    &mut *this
}
