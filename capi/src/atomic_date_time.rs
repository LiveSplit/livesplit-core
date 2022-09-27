//! An Atomic Date Time represents a UTC Date Time that tries to be as close to
//! an atomic clock as possible.

use crate::output_vec;
use livesplit_core::AtomicDateTime;
use std::os::raw::c_char;
use time::format_description::well_known::Rfc3339;

/// type
pub type OwnedAtomicDateTime = Box<AtomicDateTime>;
/// type
pub type NullableOwnedAtomicDateTime = Option<OwnedAtomicDateTime>;

/// drop
#[no_mangle]
pub extern "C" fn AtomicDateTime_drop(this: OwnedAtomicDateTime) {
    drop(this);
}

/// Represents whether the date time is actually properly derived from an
/// atomic clock. If the synchronization with the atomic clock didn't happen
/// yet or failed, this is set to <FALSE>.
#[no_mangle]
pub extern "C" fn AtomicDateTime_is_synchronized(this: &AtomicDateTime) -> bool {
    this.synced_with_atomic_clock
}

/// Converts this atomic date time into a RFC 3339 formatted date time.
#[no_mangle]
pub extern "C" fn AtomicDateTime_to_rfc3339(this: &AtomicDateTime) -> *const c_char {
    output_vec(|o| {
        let _ = this.time.format_into(o, &Rfc3339);
    })
}
