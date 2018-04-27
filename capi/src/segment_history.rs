//! Stores the segment times achieved for a certain segment. Each segment is
//! tagged with an index. Only segment times with an index larger than 0 are
//! considered times actually achieved by the runner, while the others are
//! artifacts of route changes and similar algorithmic changes.

use super::{acc, alloc};
use livesplit_core::SegmentHistory;
use segment_history_iter::OwnedSegmentHistoryIter;

/// type
pub type OwnedSegmentHistory = *mut SegmentHistory;

/// Iterates over all the segment times and their indices.
#[no_mangle]
pub unsafe extern "C" fn SegmentHistory_iter(
    this: *const SegmentHistory,
) -> OwnedSegmentHistoryIter {
    alloc(acc(this).iter())
}
