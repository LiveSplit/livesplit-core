use livesplit_core::{Time, TimeSpan, TimingMethod};
use super::{alloc, drop, acc};
use std::ptr;

pub type OwnedTime = *mut Time;

#[no_mangle]
pub unsafe extern "C" fn Time_clone(this: *const Time) -> OwnedTime {
    alloc(*acc(this))
}

#[no_mangle]
pub unsafe extern "C" fn Time_drop(this: OwnedTime) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn Time_real_time(this: *const Time) -> *const TimeSpan {
    acc(this).real_time.as_ref().map(|t| t as *const _).unwrap_or_else(ptr::null)
}

#[no_mangle]
pub unsafe extern "C" fn Time_game_time(this: *const Time) -> *const TimeSpan {
    acc(this).game_time.as_ref().map(|t| t as *const _).unwrap_or_else(ptr::null)
}

#[no_mangle]
pub unsafe extern "C" fn Time_index(this: *const Time,
                                    timing_method: TimingMethod)
                                    -> *const TimeSpan {
    acc(this)[timing_method].as_ref().map(|t| t as *const _).unwrap_or_else(ptr::null)
}
