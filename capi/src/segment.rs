//! A Segment describes a point in a speedrun that is suitable for storing a
//! split time. This stores the name of that segment, an icon, the split times
//! of different comparisons, and a history of segment times.

use super::{output_str, output_time, str};
use livesplit_core::{Segment, SegmentHistory, Time};
use std::os::raw::c_char;

/// type
pub type OwnedSegment = Box<Segment>;

/// Creates a new Segment with the name given.
#[no_mangle]
pub unsafe extern "C" fn Segment_new(name: &c_char) -> OwnedSegment {
    Box::new(Segment::new(str(name)))
}

/// drop
#[no_mangle]
pub extern "C" fn Segment_drop(this: OwnedSegment) {
    drop(this);
}

/// Accesses the name of the segment.
#[no_mangle]
pub extern "C" fn Segment_name(this: &Segment) -> *const c_char {
    output_str(this.name())
}

/// Accesses the icon of the segment encoded as a Data URL storing the image's
/// data. If the image's data is empty, this returns an empty string instead of
/// a URL.
#[no_mangle]
pub extern "C" fn Segment_icon(this: &Segment) -> *const c_char {
    output_str(this.icon().url())
}

/// Accesses the specified comparison's time. If there's none for this
/// comparison, an empty time is being returned (but not stored in the
/// segment).
#[no_mangle]
pub unsafe extern "C" fn Segment_comparison(
    this: &Segment,
    comparison: *const c_char,
) -> *const Time {
    output_time(this.comparison(str(comparison)))
}

/// Accesses the split time of the Personal Best for this segment. If it
/// doesn't exist, an empty time is returned.
#[no_mangle]
pub extern "C" fn Segment_personal_best_split_time(this: &Segment) -> *const Time {
    output_time(this.personal_best_split_time())
}

/// Accesses the Best Segment Time.
#[no_mangle]
pub extern "C" fn Segment_best_segment_time(this: &Segment) -> *const Time {
    output_time(this.best_segment_time())
}

/// Accesses the Segment History of this segment.
#[no_mangle]
pub extern "C" fn Segment_segment_history(this: &Segment) -> &SegmentHistory {
    this.segment_history()
}
