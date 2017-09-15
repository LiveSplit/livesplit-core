use livesplit_core::component::blank_space::Component as BlankSpaceComponent;
use livesplit_core::Timer;
use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use blank_space_component_state::OwnedBlankSpaceComponentState;
use component::OwnedComponent;

pub type OwnedBlankSpaceComponent = *mut BlankSpaceComponent;

#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponent_new() -> OwnedBlankSpaceComponent {
    alloc(BlankSpaceComponent::new())
}

#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponent_drop(this: OwnedBlankSpaceComponent) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponent_into_generic(
    this: OwnedBlankSpaceComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponent_state_as_json(
    this: *mut BlankSpaceComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(&this).state(acc(&timer)).write_json(o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponent_state(
    this: *mut BlankSpaceComponent,
    timer: *const Timer,
) -> OwnedBlankSpaceComponentState {
    alloc(acc_mut(&this).state(acc(&timer)))
}
