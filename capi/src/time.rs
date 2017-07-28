use livesplit_core::{Time, TimingMethod};
use super::{alloc, own_drop, acc};
use std::ptr;
use time_span::NullableTimeSpan;

pub type OwnedTime = *mut Time;

#[no_mangle]
pub unsafe extern "C" fn Time_clone(this: *const Time) -> OwnedTime {
    alloc(*acc(this))
}

#[no_mangle]
pub unsafe extern "C" fn Time_drop(this: OwnedTime) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn Time_real_time(this: *const Time) -> *const NullableTimeSpan {
    acc(this)
        .real_time
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}

#[no_mangle]
pub unsafe extern "C" fn Time_game_time(this: *const Time) -> *const NullableTimeSpan {
    acc(this)
        .game_time
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}

#[no_mangle]
pub unsafe extern "C" fn Time_index(
    this: *const Time,
    timing_method: TimingMethod,
) -> *const NullableTimeSpan {
    acc(this)[timing_method]
        .as_ref()
        .map(|t| t as *const _)
        .unwrap_or_else(ptr::null)
}
