//! A Segment describes a point in a speedrun that is suitable for storing a
//! split time. This stores the name of that segment, an icon, the split times
//! of different comparisons, and a history of segment times.

use livesplit_core::{Segment, SegmentHistory, Time};
use super::{acc, alloc, output_str, output_time, own_drop, str};
use std::os::raw::c_char;

/// type
pub type OwnedSegment = *mut Segment;

/// Creates a new Segment with the name given.
#[no_mangle]
pub unsafe extern "C" fn Segment_new(name: *const c_char) -> OwnedSegment {
    alloc(Segment::new(str(name)))
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn Segment_drop(this: OwnedSegment) {
    own_drop(this);
}

/// Accesses the name of the segment.
#[no_mangle]
pub unsafe extern "C" fn Segment_name(this: *const Segment) -> *const c_char {
    output_str(acc(this).name())
}

/// Accesses the icon of the segment encoded as a Data URL storing the image's
/// data. If the image's data is empty, this returns an empty string instead of
/// a URL.
#[no_mangle]
pub unsafe extern "C" fn Segment_icon(this: *const Segment) -> *const c_char {
    output_str(acc(this).icon().url())
}

/// Accesses the specified comparison's time. If there's none for this
/// comparison, an empty time is being returned (but not stored in the
/// segment).
#[no_mangle]
pub unsafe extern "C" fn Segment_comparison(
    this: *const Segment,
    comparison: *const c_char,
) -> *const Time {
    output_time(acc(this).comparison(str(comparison)))
}

/// Accesses the split time of the Personal Best for this segment. If it
/// doesn't exist, an empty time is returned.
#[no_mangle]
pub unsafe extern "C" fn Segment_personal_best_split_time(this: *const Segment) -> *const Time {
    output_time(acc(this).personal_best_split_time())
}

/// Accesses the Best Segment Time.
#[no_mangle]
pub unsafe extern "C" fn Segment_best_segment_time(this: *const Segment) -> *const Time {
    output_time(acc(this).best_segment_time())
}

/// Accesses the Segment History of this segment.
#[no_mangle]
pub unsafe extern "C" fn Segment_segment_history(this: *const Segment) -> *const SegmentHistory {
    acc(this).segment_history()
}
