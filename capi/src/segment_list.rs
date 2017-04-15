use livesplit_core::Segment;
use super::{alloc, own_drop, acc_mut, own};
use segment::OwnedSegment;

pub type SegmentList = Vec<Segment>;
pub type OwnedSegmentList = *mut SegmentList;

#[no_mangle]
pub unsafe extern "C" fn SegmentList_new() -> OwnedSegmentList {
    alloc(Vec::new())
}

#[no_mangle]
pub unsafe extern "C" fn SegmentList_push(this: *mut SegmentList, segment: OwnedSegment) {
    acc_mut(this).push(own(segment));
}

#[no_mangle]
pub unsafe extern "C" fn SegmentList_drop(this: OwnedSegmentList) {
    own_drop(this);
}
