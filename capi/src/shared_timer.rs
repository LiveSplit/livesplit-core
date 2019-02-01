//! A Shared Timer that can be used to share a single timer object with multiple
//! owners.

use crate::timer::OwnedTimer;
use crate::timer_read_lock::OwnedTimerReadLock;
use crate::timer_write_lock::OwnedTimerWriteLock;
use livesplit_core::SharedTimer;

/// type
pub type OwnedSharedTimer = Box<SharedTimer>;

/// Creates a new shared timer handle that shares the same timer. The inner
/// timer object only gets disposed when the final handle gets disposed.
#[no_mangle]
pub extern "C" fn SharedTimer_share(this: &SharedTimer) -> OwnedSharedTimer {
    Box::new(this.clone())
}

/// drop
#[no_mangle]
pub extern "C" fn SharedTimer_drop(this: OwnedSharedTimer) {
    drop(this);
}

/// Requests read access to the timer that is being shared. This blocks the
/// thread as long as there is an active write lock. Dispose the read lock when
/// you are done using the timer.
#[no_mangle]
pub extern "C" fn SharedTimer_read(this: &'static SharedTimer) -> OwnedTimerReadLock {
    Box::new(this.read())
}

/// Requests write access to the timer that is being shared. This blocks the
/// thread as long as there are active write or read locks. Dispose the write
/// lock when you are done using the timer.
#[no_mangle]
pub extern "C" fn SharedTimer_write(this: &'static SharedTimer) -> OwnedTimerWriteLock {
    Box::new(this.write())
}

/// Replaces the timer that is being shared by the timer provided. This blocks
/// the thread as long as there are active write or read locks. Everyone who is
/// sharing the old timer will share the provided timer after successful
/// completion.
#[no_mangle]
pub extern "C" fn SharedTimer_replace_inner(this: &SharedTimer, timer: OwnedTimer) {
    *this.write() = *timer;
}
