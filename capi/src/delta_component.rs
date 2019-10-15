//! The Delta Component is a component that shows the how far ahead or behind
//! the current attempt is compared to the chosen comparison.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::key_value_component_state::OwnedKeyValueComponentState;
use livesplit_core::component::delta::Component as DeltaComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};

/// type
pub type OwnedDeltaComponent = Box<DeltaComponent>;

/// Creates a new Delta Component.
#[no_mangle]
pub extern "C" fn DeltaComponent_new() -> OwnedDeltaComponent {
    Box::new(DeltaComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn DeltaComponent_drop(this: OwnedDeltaComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn DeltaComponent_into_generic(this: OwnedDeltaComponent) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn DeltaComponent_state_as_json(
    this: &mut DeltaComponent,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        this.state(timer, layout_settings).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer and the layout
/// settings provided.
#[no_mangle]
pub extern "C" fn DeltaComponent_state(
    this: &mut DeltaComponent,
    timer: &Timer,
    layout_settings: &GeneralLayoutSettings,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer, layout_settings))
}
