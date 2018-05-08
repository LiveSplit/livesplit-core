//! The state object describes the information to visualize for this component.

use super::{output_str, output_vec};
use livesplit_core::component::previous_segment::State as PreviousSegmentComponentState;
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type OwnedPreviousSegmentComponentState = Box<PreviousSegmentComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn PreviousSegmentComponentState_drop(this: OwnedPreviousSegmentComponentState) {
    drop(this);
}

/// The label's text.
#[no_mangle]
pub extern "C" fn PreviousSegmentComponentState_text(
    this: &PreviousSegmentComponentState,
) -> *const c_char {
    output_str(&this.text)
}

/// The delta (and possibly the possible time save).
#[no_mangle]
pub extern "C" fn PreviousSegmentComponentState_time(
    this: &PreviousSegmentComponentState,
) -> *const c_char {
    output_str(&this.time)
}

/// The semantic coloring information the delta time carries.
#[no_mangle]
pub extern "C" fn PreviousSegmentComponentState_semantic_color(
    this: &PreviousSegmentComponentState,
) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", this.semantic_color).unwrap())
}
