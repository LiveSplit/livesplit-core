//! The state object describes the information to visualize for this component.

use super::output_str;
use livesplit_core::component::pb_chance::State as PbChanceComponentState;
use std::os::raw::c_char;

/// type
pub type OwnedPbChanceComponentState = Box<PbChanceComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn PbChanceComponentState_drop(this: OwnedPbChanceComponentState) {
    drop(this);
}

/// The label's text.
#[no_mangle]
pub extern "C" fn PbChanceComponentState_text(this: &PbChanceComponentState) -> *const c_char {
    output_str(&this.text)
}

/// The current PB Chance.
#[no_mangle]
pub extern "C" fn PbChanceComponentState_pb_chance(this: &PbChanceComponentState) -> *const c_char {
    output_str(&this.pb_chance)
}
