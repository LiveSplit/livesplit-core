use livesplit_core::SegmentHistory;
use super::alloc;
use segment_history_iter::OwnedSegmentHistoryIter;

pub type OwnedSegmentHistory = *mut SegmentHistory;

/// # Safety
/// `this` must outlive `OwnedSegmentHistoryIter`
#[no_mangle]
pub unsafe extern "C" fn SegmentHistory_iter<'a>(
    this: *const SegmentHistory,
) -> OwnedSegmentHistoryIter<'a> {
    alloc((&*this).iter())
}
