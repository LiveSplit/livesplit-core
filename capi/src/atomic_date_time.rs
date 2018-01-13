//! An Atomic Date Time represents a UTC Date Time that tries to be as close to
//! an atomic clock as possible.

use livesplit_core::AtomicDateTime;
use {acc, output_str, own_drop};
use libc::c_char;

/// type
pub type OwnedAtomicDateTime = *mut AtomicDateTime;
/// type
pub type NullableOwnedAtomicDateTime = OwnedAtomicDateTime;

/// drop
#[no_mangle]
pub unsafe extern "C" fn AtomicDateTime_drop(this: OwnedAtomicDateTime) {
    own_drop(this);
}

/// Represents whether the date time is actually properly derived from an
/// atomic clock. If the synchronization with the atomic clock didn't happen
/// yet or failed, this is set to <FALSE>.
#[no_mangle]
pub unsafe extern "C" fn AtomicDateTime_is_synchronized(this: *const AtomicDateTime) -> bool {
    acc(this).synced_with_atomic_clock
}

/// Converts this atomic date time into a RFC 2822 formatted date time.
#[no_mangle]
pub unsafe extern "C" fn AtomicDateTime_to_rfc2822(this: *const AtomicDateTime) -> *const c_char {
    output_str(acc(this).time.to_rfc2822())
}

/// Converts this atomic date time into a RFC 3339 formatted date time.
#[no_mangle]
pub unsafe extern "C" fn AtomicDateTime_to_rfc3339(this: *const AtomicDateTime) -> *const c_char {
    output_str(acc(this).time.to_rfc3339())
}
