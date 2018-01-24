//! A Run Metadata variable is an arbitrary key value pair storing additional
//! information about the category. An example of this may be whether Amiibos
//! are used in the category.

use super::{acc, output_str, own_drop};
use std::os::raw::c_char;

/// type
pub type RunMetadataVariable = (*const String, *const String);
/// type
pub type NullableRunMetadataVariable = RunMetadataVariable;
/// type
pub type OwnedRunMetadataVariable = *mut RunMetadataVariable;

/// drop
#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariable_drop(this: OwnedRunMetadataVariable) {
    own_drop(this);
}

/// Accesses the name of this Run Metadata variable.
#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariable_name(
    this: *const RunMetadataVariable,
) -> *const c_char {
    output_str(acc(acc(this).0))
}

/// Accesses the value of this Run Metadata variable.
#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariable_value(
    this: *const RunMetadataVariable,
) -> *const c_char {
    output_str(acc(acc(this).1))
}
