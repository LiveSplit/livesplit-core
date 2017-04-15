use livesplit_core::clone_on_write::Cow;
use livesplit_core::Timer;
use super::{alloc, acc, acc_mut, own_drop};

pub type CowTimer = Cow<Timer>;
pub type OwnedCowTimer = *mut CowTimer;

#[no_mangle]
pub unsafe extern "C" fn CowTimer_share(this: *const CowTimer) -> OwnedCowTimer {
    alloc(acc(this).clone())
}

#[no_mangle]
pub unsafe extern "C" fn CowTimer_drop(this: OwnedCowTimer) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn CowTimer_read(this: *const CowTimer) -> *const Timer {
    &*acc(this) as &Timer
}

#[no_mangle]
pub unsafe extern "C" fn CowTimer_write(this: *mut CowTimer) -> *mut Timer {
    &mut *acc_mut(this) as &mut Timer
}
