use livesplit_core::TimeSpan;
use super::{alloc, drop, acc};

pub type OwnedTimeSpan = *mut TimeSpan;

#[no_mangle]
pub unsafe extern "C" fn TimeSpan_clone(this: *const TimeSpan) -> OwnedTimeSpan {
    alloc(*acc(this))
}

#[no_mangle]
pub unsafe extern "C" fn TimeSpan_drop(this: OwnedTimeSpan) {
    drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn TimeSpan_total_seconds(this: *const TimeSpan) -> f64 {
    acc(this).total_seconds()
}
