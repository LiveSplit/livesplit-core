//! The Total Playtime Component is a component that shows the total amount of
//! time that the current category has been played for.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::key_value_component_state::OwnedKeyValueComponentState;
use livesplit_core::component::total_playtime::Component as TotalPlaytimeComponent;
use livesplit_core::Timer;

/// type
pub type OwnedTotalPlaytimeComponent = Box<TotalPlaytimeComponent>;

/// Creates a new Total Playtime Component.
#[no_mangle]
pub extern "C" fn TotalPlaytimeComponent_new() -> OwnedTotalPlaytimeComponent {
    Box::new(TotalPlaytimeComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn TotalPlaytimeComponent_drop(this: OwnedTotalPlaytimeComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn TotalPlaytimeComponent_into_generic(
    this: OwnedTotalPlaytimeComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn TotalPlaytimeComponent_state_as_json(
    this: &mut TotalPlaytimeComponent,
    timer: &Timer,
) -> Json {
    output_vec(|o| {
        this.state(timer).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub extern "C" fn TotalPlaytimeComponent_state(
    this: &mut TotalPlaytimeComponent,
    timer: &Timer,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer))
}
