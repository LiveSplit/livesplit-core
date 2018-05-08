//! A Run Metadata variable is an arbitrary key value pair storing additional
//! information about the category. An example of this may be whether Amiibos
//! are used in the category.

use super::output_str;
use std::os::raw::c_char;

/// type
pub type RunMetadataVariable = (*const String, *const String);
/// type
pub type NullableRunMetadataVariable = RunMetadataVariable;
/// type
pub type OwnedRunMetadataVariable = Box<RunMetadataVariable>;

/// drop
#[no_mangle]
pub extern "C" fn RunMetadataVariable_drop(this: OwnedRunMetadataVariable) {
    drop(this);
}

/// Accesses the name of this Run Metadata variable.
#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariable_name(this: &RunMetadataVariable) -> *const c_char {
    output_str(&*this.0)
}

/// Accesses the value of this Run Metadata variable.
#[no_mangle]
pub unsafe extern "C" fn RunMetadataVariable_value(this: &RunMetadataVariable) -> *const c_char {
    output_str(&*this.1)
}
