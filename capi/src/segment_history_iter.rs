//! Iterates over all the segment times of a segment and their indices.

use super::{acc_mut, own_drop, SEGMENT_HISTORY_ELEMENT};
use livesplit_core::Time;
use segment_history_element::{NullableSegmentHistoryElement, SegmentHistoryElement};
use std::{ptr, slice};

/// type
pub type SegmentHistoryIter = slice::Iter<'static, (i32, Time)>;
/// type
pub type OwnedSegmentHistoryIter = *mut SegmentHistoryIter;

/// drop
#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryIter_drop(this: OwnedSegmentHistoryIter) {
    own_drop(this);
}

/// Accesses the next Segment History element. Returns <NULL> if there are no
/// more elements.
#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryIter_next(
    this: *mut SegmentHistoryIter,
) -> *const NullableSegmentHistoryElement {
    if let Some(&element) = acc_mut(this).next() {
        SEGMENT_HISTORY_ELEMENT.with(|output| {
            output.set(element);
            output.as_ptr() as *const SegmentHistoryElement
        })
    } else {
        ptr::null()
    }
}
