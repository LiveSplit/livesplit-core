//! The state object describes the information to visualize for this component.

use livesplit_core::component::total_playtime::State as TotalPlaytimeComponentState;
use super::{acc, output_str, own_drop};
use std::os::raw::c_char;

/// type
pub type OwnedTotalPlaytimeComponentState = *mut TotalPlaytimeComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponentState_drop(this: OwnedTotalPlaytimeComponentState) {
    own_drop(this);
}

/// The label's text.
#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponentState_text(
    this: *const TotalPlaytimeComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

/// The total playtime.
#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponentState_time(
    this: *const TotalPlaytimeComponentState,
) -> *const c_char {
    output_str(&acc(this).time)
}
