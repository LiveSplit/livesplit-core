//! The Current Pace Component is a component that shows a prediction of the
//! current attempt's final time, if the current attempt's pace matches the
//! chosen comparison for the remainder of the run.

use super::{Json, output_vec};
use crate::{component::OwnedComponent, key_value_component_state::OwnedKeyValueComponentState};
use livesplit_core::{Lang, Timer, component::current_pace::Component as CurrentPaceComponent};

/// type
pub type OwnedCurrentPaceComponent = Box<CurrentPaceComponent>;

/// Creates a new Current Pace Component.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentPaceComponent_new() -> OwnedCurrentPaceComponent {
    Box::new(CurrentPaceComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn CurrentPaceComponent_drop(this: OwnedCurrentPaceComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentPaceComponent_into_generic(
    this: OwnedCurrentPaceComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentPaceComponent_state_as_json(
    this: &mut CurrentPaceComponent,
    timer: &Timer,
    lang: Lang,
) -> Json {
    output_vec(|o| {
        this.state(&timer.snapshot(), lang).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentPaceComponent_state(
    this: &mut CurrentPaceComponent,
    timer: &Timer,
    lang: Lang,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(&timer.snapshot(), lang))
}
