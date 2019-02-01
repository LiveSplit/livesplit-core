//! Stores the segment times achieved for a certain segment. Each segment is
//! tagged with an index. Only segment times with an index larger than 0 are
//! considered times actually achieved by the runner, while the others are
//! artifacts of route changes and similar algorithmic changes.

use crate::segment_history_iter::OwnedSegmentHistoryIter;
use livesplit_core::SegmentHistory;

/// type
pub type OwnedSegmentHistory = Box<SegmentHistory>;

/// Iterates over all the segment times and their indices.
#[no_mangle]
pub extern "C" fn SegmentHistory_iter(this: &'static SegmentHistory) -> OwnedSegmentHistoryIter {
    Box::new(this.iter())
}
