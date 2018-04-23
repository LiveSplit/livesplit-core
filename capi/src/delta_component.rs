//! The Delta Component is a component that shows the how far ahead or behind
//! the current attempt is compared to the chosen comparison.

use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use component::OwnedComponent;
use delta_component_state::OwnedDeltaComponentState;
use livesplit_core::component::delta::Component as DeltaComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};

/// type
pub type OwnedDeltaComponent = *mut DeltaComponent;

/// Creates a new Delta Component.
#[no_mangle]
pub unsafe extern "C" fn DeltaComponent_new() -> OwnedDeltaComponent {
    alloc(DeltaComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn DeltaComponent_drop(this: OwnedDeltaComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn DeltaComponent_into_generic(this: OwnedDeltaComponent) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn DeltaComponent_state_as_json(
    this: *mut DeltaComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        acc_mut(this)
            .state(acc(timer), acc(layout_settings))
            .write_json(o)
            .unwrap();
    })
}

/// Calculates the component's state based on the timer and the layout
/// settings provided.
#[no_mangle]
pub unsafe extern "C" fn DeltaComponent_state(
    this: *mut DeltaComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> OwnedDeltaComponentState {
    alloc(acc_mut(this).state(acc(timer), acc(layout_settings)))
}
