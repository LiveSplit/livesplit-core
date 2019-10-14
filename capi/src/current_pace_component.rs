//! The Current Pace Component is a component that shows a prediction of the
//! current attempt's final time, if the current attempt's pace matches the
//! chosen comparison for the remainder of the run.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::key_value_component_state::OwnedKeyValueComponentState;
use livesplit_core::component::current_pace::Component as CurrentPaceComponent;
use livesplit_core::Timer;

/// type
pub type OwnedCurrentPaceComponent = Box<CurrentPaceComponent>;

/// Creates a new Current Pace Component.
#[no_mangle]
pub extern "C" fn CurrentPaceComponent_new() -> OwnedCurrentPaceComponent {
    Box::new(CurrentPaceComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn CurrentPaceComponent_drop(this: OwnedCurrentPaceComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn CurrentPaceComponent_into_generic(
    this: OwnedCurrentPaceComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn CurrentPaceComponent_state_as_json(
    this: &mut CurrentPaceComponent,
    timer: &Timer,
) -> Json {
    output_vec(|o| {
        this.state(timer).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub extern "C" fn CurrentPaceComponent_state(
    this: &mut CurrentPaceComponent,
    timer: &Timer,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer))
}
