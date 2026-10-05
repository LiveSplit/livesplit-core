//! A Segment Group describes a contiguous range of segments that forms a
//! one-level group.

use super::output_str;
use livesplit_core::run::SegmentGroup;
use std::os::raw::c_char;

/// Accesses the inclusive start index of the segment group.
#[unsafe(no_mangle)]
pub extern "C" fn SegmentGroup_start(this: &SegmentGroup) -> usize {
    this.start()
}

/// Accesses the exclusive end index of the segment group.
#[unsafe(no_mangle)]
pub extern "C" fn SegmentGroup_end(this: &SegmentGroup) -> usize {
    this.end()
}

/// Accesses the explicit name of the segment group. If the group uses the
/// final segment's name instead, an empty string is returned.
#[unsafe(no_mangle)]
pub extern "C" fn SegmentGroup_name(this: &SegmentGroup) -> *const c_char {
    output_str(this.name().unwrap_or_default())
}

/// Accesses the explicit icon's data. If the group uses the final segment's
/// icon instead, an empty buffer is returned.
#[unsafe(no_mangle)]
pub extern "C" fn SegmentGroup_icon_ptr(this: &SegmentGroup) -> *const u8 {
    this.icon().map_or([].as_ptr(), |icon| icon.data().as_ptr())
}

/// Accesses the amount of bytes the explicit icon's data takes up.
#[unsafe(no_mangle)]
pub extern "C" fn SegmentGroup_icon_len(this: &SegmentGroup) -> usize {
    this.icon().map_or(0, |icon| icon.data().len())
}
