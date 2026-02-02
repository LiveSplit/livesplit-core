//! The Possible Time Save Component is a component that shows how much time the
//! chosen comparison could've saved for the current segment, based on the Best
//! Segments. This component also allows showing the Total Possible Time Save
//! for the remainder of the current attempt.

use super::{Json, output_vec};
use crate::{component::OwnedComponent, key_value_component_state::OwnedKeyValueComponentState};
use livesplit_core::{
    Lang, Timer, component::possible_time_save::Component as PossibleTimeSaveComponent,
};

/// type
pub type OwnedPossibleTimeSaveComponent = Box<PossibleTimeSaveComponent>;

/// Creates a new Possible Time Save Component.
#[unsafe(no_mangle)]
pub extern "C" fn PossibleTimeSaveComponent_new() -> OwnedPossibleTimeSaveComponent {
    Box::new(PossibleTimeSaveComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn PossibleTimeSaveComponent_drop(this: OwnedPossibleTimeSaveComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn PossibleTimeSaveComponent_into_generic(
    this: OwnedPossibleTimeSaveComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn PossibleTimeSaveComponent_state_as_json(
    this: &PossibleTimeSaveComponent,
    timer: &Timer,
    lang: Lang,
) -> Json {
    output_vec(|o| {
        this.state(&timer.snapshot(), lang).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[unsafe(no_mangle)]
pub extern "C" fn PossibleTimeSaveComponent_state(
    this: &PossibleTimeSaveComponent,
    timer: &Timer,
    lang: Lang,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(&timer.snapshot(), lang))
}
