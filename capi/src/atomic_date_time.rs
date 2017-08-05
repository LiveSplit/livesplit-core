use livesplit_core::AtomicDateTime;
use {acc, output_str, own_drop};
use libc::c_char;

pub type OwnedAtomicDateTime = *mut AtomicDateTime;
pub type NullableOwnedAtomicDateTime = OwnedAtomicDateTime;

#[no_mangle]
pub unsafe extern "C" fn AtomicDateTime_drop(this: OwnedAtomicDateTime) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn AtomicDateTime_is_synchronized(this: *const AtomicDateTime) -> bool {
    acc(this).synced_with_atomic_clock
}

#[no_mangle]
pub unsafe extern "C" fn AtomicDateTime_to_rfc2822(this: *const AtomicDateTime) -> *const c_char {
    output_str(acc(this).time.to_rfc2822())
}

#[no_mangle]
pub unsafe extern "C" fn AtomicDateTime_to_rfc3339(this: *const AtomicDateTime) -> *const c_char {
    output_str(acc(this).time.to_rfc3339())
}
