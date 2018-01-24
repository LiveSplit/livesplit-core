//! The state object describes the information to visualize for this component.

use livesplit_core::component::previous_segment::State as PreviousSegmentComponentState;
use super::{acc, output_str, output_vec, own_drop};
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type OwnedPreviousSegmentComponentState = *mut PreviousSegmentComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponentState_drop(
    this: OwnedPreviousSegmentComponentState,
) {
    own_drop(this);
}

/// The label's text.
#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponentState_text(
    this: *const PreviousSegmentComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

/// The delta (and possibly the possible time save).
#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponentState_time(
    this: *const PreviousSegmentComponentState,
) -> *const c_char {
    output_str(&acc(this).time)
}

/// The semantic coloring information the delta time carries.
#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponentState_semantic_color(
    this: *const PreviousSegmentComponentState,
) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", acc(this).semantic_color).unwrap())
}
