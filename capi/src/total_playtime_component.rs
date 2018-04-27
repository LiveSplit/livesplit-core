//! The Total Playtime Component is a component that shows the total amount of
//! time that the current category has been played for.

use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use component::OwnedComponent;
use livesplit_core::Timer;
use livesplit_core::component::total_playtime::Component as TotalPlaytimeComponent;
use total_playtime_component_state::OwnedTotalPlaytimeComponentState;

/// type
pub type OwnedTotalPlaytimeComponent = *mut TotalPlaytimeComponent;

/// Creates a new Total Playtime Component.
#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponent_new() -> OwnedTotalPlaytimeComponent {
    alloc(TotalPlaytimeComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponent_drop(this: OwnedTotalPlaytimeComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponent_into_generic(
    this: OwnedTotalPlaytimeComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponent_state_as_json(
    this: *mut TotalPlaytimeComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub unsafe extern "C" fn TotalPlaytimeComponent_state(
    this: *mut TotalPlaytimeComponent,
    timer: *const Timer,
) -> OwnedTotalPlaytimeComponentState {
    alloc(acc_mut(this).state(acc(timer)))
}
