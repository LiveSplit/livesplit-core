//! The Possible Time Save Component is a component that shows how much time the
//! chosen comparison could've saved for the current segment, based on the Best
//! Segments. This component also allows showing the Total Possible Time Save
//! for the remainder of the current attempt.

use livesplit_core::component::possible_time_save::Component as PossibleTimeSaveComponent;
use livesplit_core::Timer;
use super::{acc, alloc, output_vec, own, own_drop, Json};
use possible_time_save_component_state::OwnedPossibleTimeSaveComponentState;
use component::OwnedComponent;

/// type
pub type OwnedPossibleTimeSaveComponent = *mut PossibleTimeSaveComponent;

/// Creates a new Possible Time Save Component.
#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_new() -> OwnedPossibleTimeSaveComponent {
    alloc(PossibleTimeSaveComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_drop(this: OwnedPossibleTimeSaveComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_into_generic(
    this: OwnedPossibleTimeSaveComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_state_as_json(
    this: *const PossibleTimeSaveComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc(this).state(acc(timer)).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_state(
    this: *const PossibleTimeSaveComponent,
    timer: *const Timer,
) -> OwnedPossibleTimeSaveComponentState {
    alloc(acc(this).state(acc(timer)))
}
