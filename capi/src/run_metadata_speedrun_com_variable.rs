//! A speedrun.com variable is an arbitrary key value pair storing additional
//! information about the category. An example of this may be whether Amiibos
//! are used in the category.

use super::output_str;
use std::os::raw::c_char;

/// type
pub type RunMetadataSpeedrunComVariable = (*const String, *const String);
/// type
pub type NullableRunMetadataSpeedrunComVariable = RunMetadataSpeedrunComVariable;
/// type
pub type OwnedRunMetadataSpeedrunComVariable = Box<RunMetadataSpeedrunComVariable>;

/// drop
#[no_mangle]
pub extern "C" fn RunMetadataSpeedrunComVariable_drop(this: OwnedRunMetadataSpeedrunComVariable) {
    drop(this);
}

/// Accesses the name of this speedrun.com variable.
#[no_mangle]
pub unsafe extern "C" fn RunMetadataSpeedrunComVariable_name(
    this: &RunMetadataSpeedrunComVariable,
) -> *const c_char {
    output_str(&*this.0)
}

/// Accesses the value of this speedrun.com variable.
#[no_mangle]
pub unsafe extern "C" fn RunMetadataSpeedrunComVariable_value(
    this: &RunMetadataSpeedrunComVariable,
) -> *const c_char {
    output_str(&*this.1)
}
