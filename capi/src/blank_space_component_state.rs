use livesplit_core::component::blank_space::State as BlankSpaceComponentState;
use super::{acc, own_drop};

pub type OwnedBlankSpaceComponentState = *mut BlankSpaceComponentState;

#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponentState_drop(this: OwnedBlankSpaceComponentState) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponentState_height(
    this: *const BlankSpaceComponentState,
) -> u32 {
    acc(&this).height
}
