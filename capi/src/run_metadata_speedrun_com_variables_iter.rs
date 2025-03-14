//! An iterator iterating over all the speedrun.com variables and their values
//! that have been specified.

use super::RUN_METADATA_SPEEDRUN_COM_VARIABLE;
use crate::run_metadata_speedrun_com_variable::{
    NullableRunMetadataSpeedrunComVariable, RunMetadataSpeedrunComVariable,
};
use livesplit_core::util::ordered_map;
use std::ptr;

/// type
pub type RunMetadataSpeedrunComVariablesIter = ordered_map::Iter<'static, String>;
/// type
pub type OwnedRunMetadataSpeedrunComVariablesIter = Box<RunMetadataSpeedrunComVariablesIter>;

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn RunMetadataSpeedrunComVariablesIter_drop(
    this: OwnedRunMetadataSpeedrunComVariablesIter,
) {
    drop(this);
}

/// Accesses the next speedrun.com variable. Returns <NULL> if there are no more
/// variables.
#[unsafe(no_mangle)]
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
