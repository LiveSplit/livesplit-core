//! The state object describes the information to visualize for this component.

use super::{acc, output_str, own_drop};
use livesplit_core::component::current_comparison::State as CurrentComparisonComponentState;
use std::os::raw::c_char;

/// type
pub type OwnedCurrentComparisonComponentState = *mut CurrentComparisonComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponentState_drop(
    this: OwnedCurrentComparisonComponentState,
) {
    own_drop(this);
}

/// The label's text.
#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponentState_text(
    this: *const CurrentComparisonComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

/// The name of the comparison that is currently selected to be compared
/// against.
#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponentState_comparison(
    this: *const CurrentComparisonComponentState,
) -> *const c_char {
    output_str(&acc(this).comparison)
}
