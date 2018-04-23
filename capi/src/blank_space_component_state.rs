//! The state object describes the information to visualize for this component.

use super::{acc, own_drop};
use livesplit_core::component::blank_space::State as BlankSpaceComponentState;

/// type
pub type OwnedBlankSpaceComponentState = *mut BlankSpaceComponentState;

/// drop
#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponentState_drop(this: OwnedBlankSpaceComponentState) {
    own_drop(this);
}

/// The height of the component.
#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponentState_height(
    this: *const BlankSpaceComponentState,
) -> u32 {
    acc(this).height
}
