//! A custom variable is a key value pair storing additional information about a
//! run. Unlike the speedrun.com variables, these can be fully custom and don't
//! need to correspond to anything on speedrun.com. Permanent custom variables
//! can be specified by the runner. Additionally auto splitters or other sources
//! may provide temporary custom variables that are not stored in the splits
//! files.

use super::output_str;
use livesplit_core::run::CustomVariable;
use std::os::raw::c_char;

/// type
pub type RunMetadataCustomVariable = (*const String, *const CustomVariable);
/// type
pub type NullableRunMetadataCustomVariable = RunMetadataCustomVariable;
/// type
pub type OwnedRunMetadataCustomVariable = Box<RunMetadataCustomVariable>;

/// drop
#[no_mangle]
pub extern "C" fn RunMetadataCustomVariable_drop(this: OwnedRunMetadataCustomVariable) {
    drop(this);
}

/// Accesses the name of this custom variable.
#[no_mangle]
pub unsafe extern "C" fn RunMetadataCustomVariable_name(
    this: &RunMetadataCustomVariable,
) -> *const c_char {
    output_str(&*this.0)
}

/// Accesses the value of this custom variable.
#[no_mangle]
pub unsafe extern "C" fn RunMetadataCustomVariable_value(
    this: &RunMetadataCustomVariable,
) -> *const c_char {
    output_str(&(*this.1).value)
}

/// Returns <TRUE> if the custom variable is permanent. Permanent variables get
/// stored in the splits file and are visible in the run editor. Temporary
/// variables are not.
#[no_mangle]
pub unsafe extern "C" fn RunMetadataCustomVariable_is_permanent(
    this: &RunMetadataCustomVariable,
) -> bool {
    (*this.1).is_permanent
}
