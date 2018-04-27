//! The Title Component is a component that shows the name of the game and the
//! category that is being run. Additionally, the game icon, the attempt count,
//! and the total number of successfully finished runs can be shown.

use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use component::OwnedComponent;
use livesplit_core::Timer;
use livesplit_core::component::title::Component as TitleComponent;
use title_component_state::OwnedTitleComponentState;

/// type
pub type OwnedTitleComponent = *mut TitleComponent;

/// Creates a new Title Component.
#[no_mangle]
pub unsafe extern "C" fn TitleComponent_new() -> OwnedTitleComponent {
    alloc(TitleComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn TitleComponent_drop(this: OwnedTitleComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn TitleComponent_into_generic(this: OwnedTitleComponent) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn TitleComponent_state_as_json(
    this: *mut TitleComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub unsafe extern "C" fn TitleComponent_state(
    this: *mut TitleComponent,
    timer: *const Timer,
) -> OwnedTitleComponentState {
    alloc(acc_mut(this).state(acc(timer)))
}
