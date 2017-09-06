use livesplit_core::{Segment, SegmentHistory, Time};
use super::{acc, alloc, output_str, output_time, own_drop, str};
use libc::c_char;

pub type OwnedSegment = *mut Segment;

#[no_mangle]
pub unsafe extern "C" fn Segment_new(name: *const c_char) -> OwnedSegment {
    alloc(Segment::new(str(&name)))
}

#[no_mangle]
pub unsafe extern "C" fn Segment_drop(this: OwnedSegment) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn Segment_name(this: *const Segment) -> *const c_char {
    output_str(acc(&this).name())
}

#[no_mangle]
pub unsafe extern "C" fn Segment_icon(this: *const Segment) -> *const c_char {
    output_str(acc(&this).icon().url())
}

#[no_mangle]
pub unsafe extern "C" fn Segment_comparison(
    this: *const Segment,
    comparison: *const c_char,
) -> *const Time {
    output_time(acc(&this).comparison(str(&comparison)))
}

#[no_mangle]
pub unsafe extern "C" fn Segment_personal_best_split_time(this: *const Segment) -> *const Time {
    output_time(acc(&this).personal_best_split_time())
}

#[no_mangle]
pub unsafe extern "C" fn Segment_best_segment_time(this: *const Segment) -> *const Time {
    output_time(acc(&this).best_segment_time())
}

#[no_mangle]
pub unsafe extern "C" fn Segment_segment_history(this: *const Segment) -> *const SegmentHistory {
    acc(&this).segment_history()
}
