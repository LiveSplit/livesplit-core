//! The state object describes the information to visualize for this component.

use livesplit_core::component::blank_space::State as BlankSpaceComponentState;

/// type
pub type OwnedBlankSpaceComponentState = Box<BlankSpaceComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn BlankSpaceComponentState_drop(this: OwnedBlankSpaceComponentState) {
    drop(this);
}

/// The size of the component.
#[no_mangle]
pub extern "C" fn BlankSpaceComponentState_size(this: &BlankSpaceComponentState) -> u32 {
    this.size
}
