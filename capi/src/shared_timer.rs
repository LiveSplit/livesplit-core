use livesplit_core::SharedTimer;
use super::{alloc, acc, own, own_drop};
use cow_timer::OwnedCowTimer;

pub type OwnedSharedTimer = *mut SharedTimer;

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_share(this: *const SharedTimer) -> OwnedSharedTimer {
    alloc(acc(this).clone())
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_drop(this: OwnedSharedTimer) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_get(this: *const SharedTimer) -> OwnedCowTimer {
    alloc(acc(this).get())
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_commit(this: *const SharedTimer, timer: OwnedCowTimer) {
    acc(this).commit(own(timer));
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_split(this: *const SharedTimer) {
    acc(this).split();
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_skip_split(this: *const SharedTimer) {
    acc(this).skip_split();
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_reset(this: *const SharedTimer, update_splits: bool) {
    acc(this).reset(update_splits);
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_pause(this: *const SharedTimer) {
    acc(this).pause();
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_switch_to_next_comparison(this: *const SharedTimer) {
    acc(this).switch_to_next_comparison();
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_switch_to_previous_comparison(this: *const SharedTimer) {
    acc(this).switch_to_previous_comparison();
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_undo(this: *const SharedTimer) {
    acc(this).undo();
}

#[no_mangle]
pub unsafe extern "C" fn SharedTimer_redo(this: *const SharedTimer) {
    acc(this).redo();
}
