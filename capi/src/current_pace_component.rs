//! The Current Pace Component is a component that shows a prediction of the
//! current attempt's final time, if the current attempt's pace matches the
//! chosen comparison for the remainder of the run.

use livesplit_core::component::current_pace::Component as CurrentPaceComponent;
use livesplit_core::Timer;
use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use current_pace_component_state::OwnedCurrentPaceComponentState;
use component::OwnedComponent;

/// type
pub type OwnedCurrentPaceComponent = *mut CurrentPaceComponent;

/// Creates a new Current Pace Component.
#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponent_new() -> OwnedCurrentPaceComponent {
    alloc(CurrentPaceComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponent_drop(this: OwnedCurrentPaceComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponent_into_generic(
    this: OwnedCurrentPaceComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponent_state_as_json(
    this: *mut CurrentPaceComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub unsafe extern "C" fn CurrentPaceComponent_state(
    this: *mut CurrentPaceComponent,
    timer: *const Timer,
) -> OwnedCurrentPaceComponentState {
    alloc(acc_mut(this).state(acc(timer)))
}
