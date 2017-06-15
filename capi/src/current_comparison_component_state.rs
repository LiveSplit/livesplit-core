use livesplit_core::component::current_comparison::State as CurrentComparisonComponentState;
use super::{own_drop, acc, output_str};
use libc::c_char;

pub type OwnedCurrentComparisonComponentState = *mut CurrentComparisonComponentState;

#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponentState_drop(
    this: OwnedCurrentComparisonComponentState,
) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponentState_text(
    this: *const CurrentComparisonComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponentState_comparison(
    this: *const CurrentComparisonComponentState,
) -> *const c_char {
    output_str(&acc(this).comparison)
}
