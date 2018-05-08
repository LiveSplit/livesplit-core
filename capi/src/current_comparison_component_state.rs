//! The state object describes the information to visualize for this component.

use super::output_str;
use livesplit_core::component::current_comparison::State as CurrentComparisonComponentState;
use std::os::raw::c_char;

/// type
pub type OwnedCurrentComparisonComponentState = Box<CurrentComparisonComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn CurrentComparisonComponentState_drop(this: OwnedCurrentComparisonComponentState) {
    drop(this);
}

/// The label's text.
#[no_mangle]
pub extern "C" fn CurrentComparisonComponentState_text(
    this: &CurrentComparisonComponentState,
) -> *const c_char {
    output_str(&this.text)
}

/// The name of the comparison that is currently selected to be compared
/// against.
#[no_mangle]
pub extern "C" fn CurrentComparisonComponentState_comparison(
    this: &CurrentComparisonComponentState,
) -> *const c_char {
    output_str(&this.comparison)
}
