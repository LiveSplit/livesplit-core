//! The Blank Space Component is simply an empty component that doesn't show
//! anything other than a background. It mostly serves as padding between other
//! components.

use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use blank_space_component_state::OwnedBlankSpaceComponentState;
use component::OwnedComponent;
use livesplit_core::Timer;
use livesplit_core::component::blank_space::Component as BlankSpaceComponent;

/// type
pub type OwnedBlankSpaceComponent = *mut BlankSpaceComponent;

/// Creates a new Blank Space Component.
#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponent_new() -> OwnedBlankSpaceComponent {
    alloc(BlankSpaceComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponent_drop(this: OwnedBlankSpaceComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponent_into_generic(
    this: OwnedBlankSpaceComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponent_state_as_json(
    this: *mut BlankSpaceComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub unsafe extern "C" fn BlankSpaceComponent_state(
    this: *mut BlankSpaceComponent,
    timer: *const Timer,
) -> OwnedBlankSpaceComponentState {
    alloc(acc_mut(this).state(acc(timer)))
}
