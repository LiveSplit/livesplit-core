//! The Possible Time Save Component is a component that shows how much time the
//! chosen comparison could've saved for the current segment, based on the Best
//! Segments. This component also allows showing the Total Possible Time Save
//! for the remainder of the current attempt.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::key_value_component_state::OwnedKeyValueComponentState;
use livesplit_core::component::possible_time_save::Component as PossibleTimeSaveComponent;
use livesplit_core::Timer;

/// type
pub type OwnedPossibleTimeSaveComponent = Box<PossibleTimeSaveComponent>;

/// Creates a new Possible Time Save Component.
#[no_mangle]
pub extern "C" fn PossibleTimeSaveComponent_new() -> OwnedPossibleTimeSaveComponent {
    Box::new(PossibleTimeSaveComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn PossibleTimeSaveComponent_drop(this: OwnedPossibleTimeSaveComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn PossibleTimeSaveComponent_into_generic(
    this: OwnedPossibleTimeSaveComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn PossibleTimeSaveComponent_state_as_json(
    this: &PossibleTimeSaveComponent,
    timer: &Timer,
) -> Json {
    output_vec(|o| {
        this.state(timer).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub extern "C" fn PossibleTimeSaveComponent_state(
    this: &PossibleTimeSaveComponent,
    timer: &Timer,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer))
}
