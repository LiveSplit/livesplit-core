use super::{acc, output_str, own_drop};
use libc::c_char;

pub type RunMetadataVariable = (*const String, *const String);
pub type NullableRunMetadataVariable = RunMetadataVariable;
pub type OwnedRunMetadataVariable = *mut RunMetadataVariable;

#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariable_drop(this: OwnedRunMetadataVariable) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariable_name(
    this: *const RunMetadataVariable,
) -> *const c_char {
    output_str(acc(&acc(&this).0))
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariable_value(
    this: *const RunMetadataVariable,
) -> *const c_char {
    output_str(acc(&acc(&this).1))
}
