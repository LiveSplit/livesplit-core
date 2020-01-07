//! An iterator iterating over all the speedrun.com variables and their values
//! that have been specified.

use super::RUN_METADATA_SPEEDRUN_COM_VARIABLE;
use crate::run_metadata_speedrun_com_variable::{
    NullableRunMetadataSpeedrunComVariable, RunMetadataSpeedrunComVariable,
};
use livesplit_core::indexmap;
use std::ptr;

/// type
pub type RunMetadataSpeedrunComVariablesIter = indexmap::map::Iter<'static, String, String>;
/// type
pub type OwnedRunMetadataSpeedrunComVariablesIter = Box<RunMetadataSpeedrunComVariablesIter>;

/// drop
#[no_mangle]
pub extern "C" fn RunMetadataSpeedrunComVariablesIter_drop(
    this: OwnedRunMetadataSpeedrunComVariablesIter,
) {
    drop(this);
}

/// Accesses the next speedrun.com variable. Returns <NULL> if there are no more
/// variables.
#[no_mangle]
pub extern "C" fn RunMetadataSpeedrunComVariablesIter_next(
    this: &mut RunMetadataSpeedrunComVariablesIter,
) -> *const NullableRunMetadataSpeedrunComVariable {
    if let Some((name, value)) = this.next() {
        RUN_METADATA_SPEEDRUN_COM_VARIABLE.with(|output| {
            output.set((name, value));
            output.as_ptr() as *const RunMetadataSpeedrunComVariable
        })
    } else {
        ptr::null()
    }
}
