//! An iterator iterating over all the custom variables and their values
//! that have been specified.

use super::RUN_METADATA_CUSTOM_VARIABLE;
use crate::run_metadata_custom_variable::{
    NullableRunMetadataCustomVariable, RunMetadataCustomVariable,
};
use livesplit_core::{run::CustomVariable, util::ordered_map};
use std::ptr;

/// type
pub type RunMetadataCustomVariablesIter = ordered_map::Iter<'static, CustomVariable>;
/// type
pub type OwnedRunMetadataCustomVariablesIter = Box<RunMetadataCustomVariablesIter>;

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn RunMetadataCustomVariablesIter_drop(this: OwnedRunMetadataCustomVariablesIter) {
    drop(this);
}

/// Accesses the next custom variable. Returns <NULL> if there are no more
/// variables.
#[unsafe(no_mangle)]
pub extern "C" fn RunMetadataCustomVariablesIter_next(
    this: &mut RunMetadataCustomVariablesIter,
) -> *const NullableRunMetadataCustomVariable {
    if let Some((name, value)) = this.next() {
        RUN_METADATA_CUSTOM_VARIABLE.with(|output| {
            output.set((name, value));
            output.as_ptr() as *const RunMetadataCustomVariable
        })
    } else {
        ptr::null()
    }
}
