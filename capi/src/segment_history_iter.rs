//! Iterates over all the segment times of a segment and their indices.

use super::SEGMENT_HISTORY_ELEMENT;
use crate::segment_history_element::{NullableSegmentHistoryElement, SegmentHistoryElement};
use livesplit_core::Time;
use std::{ptr, slice};

/// type
pub type SegmentHistoryIter = slice::Iter<'static, (i32, Time)>;
/// type
pub type OwnedSegmentHistoryIter = Box<SegmentHistoryIter>;

/// drop
#[no_mangle]
pub extern "C" fn SegmentHistoryIter_drop(this: OwnedSegmentHistoryIter) {
    drop(this);
}

/// Accesses the next Segment History element. Returns <NULL> if there are no
/// more elements.
#[no_mangle]
pub extern "C" fn SegmentHistoryIter_next(
    this: &mut SegmentHistoryIter,
) -> *const NullableSegmentHistoryElement {
    if let Some(&element) = this.next() {
        SEGMENT_HISTORY_ELEMENT.with(|output| {
            output.set(element);
            output.as_ptr() as *const SegmentHistoryElement
        })
    } else {
        ptr::null()
    }
}
