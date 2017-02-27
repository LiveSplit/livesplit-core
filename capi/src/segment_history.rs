use livesplit_core::SegmentHistory;
use super::{alloc, acc};
use segment_history_iter::OwnedSegmentHistoryIter;

pub type OwnedSegmentHistory = *mut SegmentHistory;

#[no_mangle]
pub unsafe extern "C" fn SegmentHistory_iter(this: *const SegmentHistory)
                                             -> OwnedSegmentHistoryIter {
    alloc(acc(this).iter())
}
