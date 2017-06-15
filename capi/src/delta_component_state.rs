use livesplit_core::component::delta::State as DeltaComponentState;
use super::{own_drop, acc, output_str, output_str_with};
use std::fmt::Write;
use libc::c_char;

pub type OwnedDeltaComponentState = *mut DeltaComponentState;

#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_drop(this: OwnedDeltaComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_text(
    this: *const DeltaComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_time(
    this: *const DeltaComponentState,
) -> *const c_char {
    output_str(&acc(this).time)
}

#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_color(
    this: *const DeltaComponentState,
) -> *const c_char {
    output_str_with(|f| write!(f, "{:?}", acc(this).color).unwrap())
}
