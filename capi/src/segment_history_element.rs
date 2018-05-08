//! A segment time achieved for a segment. It is tagged with an index. Only
//! segment times with an index larger than 0 are considered times actually
//! achieved by the runner, while the others are artifacts of route changes and
//! similar algorithmic changes.

use super::output_time;
use livesplit_core::Time;

/// type
pub type SegmentHistoryElement = (i32, Time);
/// type
pub type NullableSegmentHistoryElement = SegmentHistoryElement;
/// type
pub type OwnedSegmentHistoryElement = Box<SegmentHistoryElement>;

/// Accesses the index of the segment history element.
#[no_mangle]
pub extern "C" fn SegmentHistoryElement_index(this: &SegmentHistoryElement) -> i32 {
    this.0
}

/// Accesses the segment time of the segment history element.
#[no_mangle]
pub extern "C" fn SegmentHistoryElement_time(this: &SegmentHistoryElement) -> *const Time {
    output_time(this.1)
}
