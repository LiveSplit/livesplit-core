//! The state object describes the information to visualize for this component.

use super::{acc, output_str, output_vec, own_drop};
use livesplit_core::component::delta::State as DeltaComponentState;
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type OwnedDeltaComponentState = *mut DeltaComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_drop(this: OwnedDeltaComponentState) {
    own_drop(this);
}

/// The label's text.
#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_text(
    this: *const DeltaComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

/// The delta.
#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_time(
    this: *const DeltaComponentState,
) -> *const c_char {
    output_str(&acc(this).time)
}

/// The semantic coloring information the delta time carries.
#[no_mangle]
pub unsafe extern "C" fn DeltaComponentState_semantic_color(
    this: *const DeltaComponentState,
) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", acc(this).semantic_color).unwrap())
}
