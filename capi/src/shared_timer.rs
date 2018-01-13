//! A Shared Timer that can be used to share a single timer object with multiple
//! owners.

use livesplit_core::SharedTimer;
use timer::OwnedTimer;
use timer_read_lock::OwnedTimerReadLock;
use timer_write_lock::OwnedTimerWriteLock;
use super::{acc, alloc, own, own_drop};

/// type
pub type OwnedSharedTimer = *mut SharedTimer;

/// Creates a new shared timer handle that shares the same timer. The inner
/// timer object only gets disposed when the final handle gets disposed.
#[no_mangle]
pub unsafe extern "C" fn SharedTimer_share(this: *const SharedTimer) -> OwnedSharedTimer {
    alloc(acc(this).clone())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn SharedTimer_drop(this: OwnedSharedTimer) {
    own_drop(this);
}

/// Requests read access to the timer that is being shared. This blocks the
/// thread as long as there is an active write lock. Dispose the read lock when
/// you are done using the timer.
#[no_mangle]
pub unsafe extern "C" fn SharedTimer_read(this: *const SharedTimer) -> OwnedTimerReadLock {
    alloc(acc(this).read())
}

/// Requests write access to the timer that is being shared. This blocks the
/// thread as long as there are active write or read locks. Dispose the write
/// lock when you are done using the timer.
#[no_mangle]
pub unsafe extern "C" fn SharedTimer_write(this: *const SharedTimer) -> OwnedTimerWriteLock {
    alloc(acc(this).write())
}

/// Replaces the timer that is being shared by the timer provided. This blocks
/// the thread as long as there are active write or read locks. Everyone who is
/// sharing the old timer will share the provided timer after successful
/// completion.
#[no_mangle]
pub unsafe extern "C" fn SharedTimer_replace_inner(this: *const SharedTimer, timer: OwnedTimer) {
    *acc(this).write() = own(timer);
}
