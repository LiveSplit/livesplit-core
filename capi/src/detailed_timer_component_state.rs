use livesplit_core::component::detailed_timer::State as DetailedTimerComponentState;
use super::{acc, output_str, output_vec, own_drop, Nullablec_char};
use libc::c_char;
use std::io::Write;
use std::ptr;

pub type OwnedDetailedTimerComponentState = *mut DetailedTimerComponentState;

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_drop(this: OwnedDetailedTimerComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_timer_time(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(&acc(this).timer.time)
}

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_timer_fraction(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(&acc(this).timer.fraction)
}

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_timer_semantic_color(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_vec(|f| {
        write!(f, "{:?}", acc(this).timer.semantic_color).unwrap()
    })
}

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_segment_timer_time(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(&acc(this).segment_timer.time)
}

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_segment_timer_fraction(
    this: *const DetailedTimerComponentState,
) -> *const c_char {
    output_str(&acc(this).segment_timer.fraction)
}

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_comparison1_visible(
    this: *const DetailedTimerComponentState,
) -> bool {
    acc(this).comparison1.is_some()
}

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

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_comparison2_visible(
    this: *const DetailedTimerComponentState,
) -> bool {
    acc(this).comparison2.is_some()
}

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

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_icon_change(
    this: *const DetailedTimerComponentState,
) -> *const Nullablec_char {
    acc(this)
        .icon_change
        .as_ref()
        .map_or_else(ptr::null, output_str)
}

#[no_mangle]
pub unsafe extern "C" fn DetailedTimerComponentState_name(
    this: *const DetailedTimerComponentState,
) -> *const Nullablec_char {
    acc(this)
        .segment_name
        .as_ref()
        .map_or_else(ptr::null, output_str)
}
