//! The state object describes the information to visualize for this component.

use super::{output_str, output_vec, Nullablec_char};
use livesplit_core::component::detailed_timer::State as DetailedTimerComponentState;
use std::{io::Write, os::raw::c_char, ptr};

/// type
pub type OwnedDetailedTimerComponentState = Box<DetailedTimerComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_drop(this: OwnedDetailedTimerComponentState) {
    drop(this);
}

/// The time shown by the component's main timer without the fractional part.
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_timer_time(
    this: &DetailedTimerComponentState,
) -> *const c_char {
    output_str(&this.timer.time)
}

/// The fractional part of the time shown by the main timer (including the dot).
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_timer_fraction(
    this: &DetailedTimerComponentState,
) -> *const c_char {
    output_str(&this.timer.fraction)
}

/// The semantic coloring information the main timer's time carries.
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_timer_semantic_color(
    this: &DetailedTimerComponentState,
) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", this.timer.semantic_color).unwrap())
}

/// The time shown by the component's segment timer without the fractional part.
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_segment_timer_time(
    this: &DetailedTimerComponentState,
) -> *const c_char {
    output_str(&this.segment_timer.time)
}

/// The fractional part of the time shown by the segment timer (including the
/// dot).
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_segment_timer_fraction(
    this: &DetailedTimerComponentState,
) -> *const c_char {
    output_str(&this.segment_timer.fraction)
}

/// Returns whether the first comparison is visible.
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_comparison1_visible(
    this: &DetailedTimerComponentState,
) -> bool {
    this.comparison1.is_some()
}

/// Returns the name of the first comparison. You may not call this if the first
/// comparison is not visible.
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_comparison1_name(
    this: &DetailedTimerComponentState,
) -> *const c_char {
    output_str(
        &this
            .comparison1
            .as_ref()
            .expect("Comparison 1 is not visible")
            .name,
    )
}

/// Returns the time of the first comparison. You may not call this if the first
/// comparison is not visible.
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_comparison1_time(
    this: &DetailedTimerComponentState,
) -> *const c_char {
    output_str(
        &this
            .comparison1
            .as_ref()
            .expect("Comparison 1 is not visible")
            .time,
    )
}

/// Returns whether the second comparison is visible.
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_comparison2_visible(
    this: &DetailedTimerComponentState,
) -> bool {
    this.comparison2.is_some()
}

/// Returns the name of the second comparison. You may not call this if the
/// second comparison is not visible.
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_comparison2_name(
    this: &DetailedTimerComponentState,
) -> *const c_char {
    output_str(
        &this
            .comparison2
            .as_ref()
            .expect("Comparison 2 is not visible")
            .name,
    )
}

/// Returns the time of the second comparison. You may not call this if the
/// second comparison is not visible.
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_comparison2_time(
    this: &DetailedTimerComponentState,
) -> *const c_char {
    output_str(
        &this
            .comparison2
            .as_ref()
            .expect("Comparison 2 is not visible")
            .time,
    )
}

/// The icon of the segment. The associated image can be looked up in the image
/// cache. The image may be the empty image. This indicates that there is no
/// icon.
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_icon(
    this: &DetailedTimerComponentState,
) -> *const c_char {
    output_str(this.icon.format_str(&mut [0; 64]))
}

/// The name of the segment. This may be <NULL> if it's not supposed to be
/// visualized.
#[no_mangle]
pub extern "C" fn DetailedTimerComponentState_segment_name(
    this: &DetailedTimerComponentState,
) -> *const Nullablec_char {
    this.segment_name
        .as_ref()
        .map_or_else(ptr::null, output_str)
}
