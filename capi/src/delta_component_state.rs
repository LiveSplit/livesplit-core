//! The state object describes the information to visualize for this component.

use super::{output_str, output_vec};
use livesplit_core::component::delta::State as DeltaComponentState;
use std::io::Write;
use std::os::raw::c_char;

/// type
pub type OwnedDeltaComponentState = Box<DeltaComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn DeltaComponentState_drop(this: OwnedDeltaComponentState) {
    drop(this);
}

/// The label's text.
#[no_mangle]
pub extern "C" fn DeltaComponentState_text(this: &DeltaComponentState) -> *const c_char {
    output_str(&this.text)
}

/// The delta.
#[no_mangle]
pub extern "C" fn DeltaComponentState_time(this: &DeltaComponentState) -> *const c_char {
    output_str(&this.time)
}

/// The semantic coloring information the delta time carries.
#[no_mangle]
pub extern "C" fn DeltaComponentState_semantic_color(this: &DeltaComponentState) -> *const c_char {
    output_vec(|f| write!(f, "{:?}", this.semantic_color).unwrap())
}
