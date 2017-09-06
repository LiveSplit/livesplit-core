use livesplit_core::Time;
use super::{acc_mut, own_drop, SEGMENT_HISTORY_ELEMENT};
use std::{ptr, slice};
use segment_history_element::{NullableSegmentHistoryElement, SegmentHistoryElement};

pub type SegmentHistoryIter<'a> = slice::Iter<'a, (i32, Time)>;
pub type OwnedSegmentHistoryIter<'a> = *mut SegmentHistoryIter<'a>;

#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryIter_drop(this: OwnedSegmentHistoryIter) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryIter_next(
    this: *mut SegmentHistoryIter,
) -> *const NullableSegmentHistoryElement {
    if let Some(&element) = acc_mut(&this).next() {
        SEGMENT_HISTORY_ELEMENT.with(|output| {
            output.set(element);
            output.as_ptr() as *const SegmentHistoryElement
        })
    } else {
        ptr::null()
    }
}
