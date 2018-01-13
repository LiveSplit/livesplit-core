//! A segment time achieved for a segment. It is tagged with an index. Only
//! segment times with an index larger than 0 are considered times actually
//! achieved by the runner, while the others are artifacts of route changes and
//! similar algorithmic changes.

use livesplit_core::Time;
use super::{acc, output_time};

/// type
pub type SegmentHistoryElement = (i32, Time);
/// type
pub type NullableSegmentHistoryElement = SegmentHistoryElement;
/// type
pub type OwnedSegmentHistoryElement = *mut SegmentHistoryElement;

/// Accesses the index of the segment history element.
#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryElement_index(this: *const SegmentHistoryElement) -> i32 {
    acc(this).0
}

/// Accesses the segment time of the segment history element.
#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryElement_time(
    this: *const SegmentHistoryElement,
) -> *const Time {
    output_time(acc(this).1)
}
