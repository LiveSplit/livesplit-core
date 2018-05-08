//! The state object describes the information to visualize for this component.

use super::output_str;
use livesplit_core::component::sum_of_best::State as SumOfBestComponentState;
use std::os::raw::c_char;

/// type
pub type OwnedSumOfBestComponentState = Box<SumOfBestComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn SumOfBestComponentState_drop(this: OwnedSumOfBestComponentState) {
    drop(this);
}

/// The label's text.
#[no_mangle]
pub extern "C" fn SumOfBestComponentState_text(this: &SumOfBestComponentState) -> *const c_char {
    output_str(&this.text)
}

/// The sum of best segments.
#[no_mangle]
pub extern "C" fn SumOfBestComponentState_time(this: &SumOfBestComponentState) -> *const c_char {
    output_str(&this.time)
}
