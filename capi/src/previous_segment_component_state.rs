use livesplit_core::component::previous_segment::State as PreviousSegmentComponentState;
use super::{own_drop, acc, output_str, output_str_with};
use std::fmt::Write;
use libc::c_char;

pub type OwnedPreviousSegmentComponentState = *mut PreviousSegmentComponentState;

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponentState_drop(
    this: OwnedPreviousSegmentComponentState,
) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponentState_text(
    this: *const PreviousSegmentComponentState,
) -> *const c_char {
    output_str(&acc(this).text)
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponentState_time(
    this: *const PreviousSegmentComponentState,
) -> *const c_char {
    output_str(&acc(this).time)
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponentState_semantic_color(
    this: *const PreviousSegmentComponentState,
) -> *const c_char {
    output_str_with(|f| write!(f, "{:?}", acc(this).semantic_color).unwrap())
}
