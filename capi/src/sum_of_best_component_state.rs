//! The state object describes the information to visualize for this component.

use livesplit_core::component::sum_of_best::State as SumOfBestComponentState;
use super::{acc, output_str, own_drop};
use libc::c_char;

/// type
pub type OwnedSumOfBestComponentState = *mut SumOfBestComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponentState_drop(this: OwnedSumOfBestComponentState) {
    own_drop(this);
}

/// The label's text.
#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponentState_text(
    this: *const SumOfBestComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

/// The sum of best segments.
#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponentState_time(
    this: *const SumOfBestComponentState,
) -> *const c_char {
    output_str(&acc(this).time)
}
