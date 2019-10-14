//! The Current Comparison Component is a component that shows the name of the
//! comparison that is currently selected to be compared against.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::key_value_component_state::OwnedKeyValueComponentState;
use livesplit_core::component::current_comparison::Component as CurrentComparisonComponent;
use livesplit_core::Timer;

/// type
pub type OwnedCurrentComparisonComponent = Box<CurrentComparisonComponent>;

/// Creates a new Current Comparison Component.
#[no_mangle]
pub extern "C" fn CurrentComparisonComponent_new() -> OwnedCurrentComparisonComponent {
    Box::new(CurrentComparisonComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn CurrentComparisonComponent_drop(this: OwnedCurrentComparisonComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn CurrentComparisonComponent_into_generic(
    this: OwnedCurrentComparisonComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn CurrentComparisonComponent_state_as_json(
    this: &mut CurrentComparisonComponent,
    timer: &Timer,
) -> Json {
    output_vec(|o| {
        this.state(timer).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub extern "C" fn CurrentComparisonComponent_state(
    this: &mut CurrentComparisonComponent,
    timer: &Timer,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer))
}
