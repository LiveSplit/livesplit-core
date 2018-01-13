//! The state object describes the information to visualize for this component.

use livesplit_core::component::detailed_timer::State as DetailedTimerComponentState;
use super::{acc, output_str, output_vec, own_drop, Nullablec_char};
use libc::c_char;
use std::io::Write;
use std::ptr;

/// type
pub type OwnedDetailedTimerComponentState = *mut DetailedTimerComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_drop(this: OwnedDetailedTimerComponentState) {
    own_drop(this);
}

/// The time shown by the component's main timer without the fractional part.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_timer_time(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(&acc(this).timer.time)
}

/// The fractional part of the time shown by the main timer (including the dot).
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_timer_fraction(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(&acc(this).timer.fraction)
}

/// The semantic coloring information the main timer's time carries.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_timer_semantic_color(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", acc(this).timer.semantic_color).unwrap())
}

/// The time shown by the component's segment timer without the fractional part.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_segment_timer_time(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(&acc(this).segment_timer.time)
}

/// The fractional part of the time shown by the segment timer (including the
/// dot).
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_segment_timer_fraction(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(&acc(this).segment_timer.fraction)
}

/// Returns whether the first comparison is visible.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_comparison1_visible(
    this: *const DetailedTimerComponentState,
) -> bool {
    acc(this).comparison1.is_some()
}

/// Returns the name of the first comparison. You may not call this if the first
/// comparison is not visible.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_comparison1_name(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(
        &acc(this)
            .comparison1
            .as_ref()
            .expect("Comparison 1 is not visible")
            .name,
    )
}

/// Returns the time of the first comparison. You may not call this if the first
/// comparison is not visible.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_comparison1_time(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(
        &acc(this)
            .comparison1
            .as_ref()
            .expect("Comparison 1 is not visible")
            .time,
    )
}

/// Returns whether the second comparison is visible.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_comparison2_visible(
    this: *const DetailedTimerComponentState,
) -> bool {
    acc(this).comparison2.is_some()
}

/// Returns the name of the second comparison. You may not call this if the
/// second comparison is not visible.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_comparison2_name(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(
        &acc(this)
            .comparison2
            .as_ref()
            .expect("Comparison 2 is not visible")
            .name,
    )
}

/// Returns the time of the second comparison. You may not call this if the
/// second comparison is not visible.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_comparison2_time(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(
        &acc(this)
            .comparison2
            .as_ref()
            .expect("Comparison 2 is not visible")
            .time,
    )
}

/// The segment's icon encoded as a Data URL. This value is only specified
/// whenever the icon changes. If you explicitly want to query this value,
/// remount the component. The String itself may be empty. This indicates
/// that there is no icon.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_icon_change(
    this: *const DetailedTimerComponentState,
) -> *const Nullablec_char {
    acc(this)
        .icon_change
        .as_ref()
        .map_or_else(ptr::null, output_str)
}

/// The name of the segment. This may be <NULL> if it's not supposed to be
/// visualized.
#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_segment_name(
    this: *const DetailedTimerComponentState,
) -> *const Nullablec_char {
    acc(this)
        .segment_name
        .as_ref()
        .map_or_else(ptr::null, output_str)
}
