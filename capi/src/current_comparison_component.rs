//! The Current Comparison Component is a component that shows the name of the
//! comparison that is currently selected to be compared against.

use livesplit_core::component::current_comparison::Component as CurrentComparisonComponent;
use livesplit_core::Timer;
use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use current_comparison_component_state::OwnedCurrentComparisonComponentState;
use component::OwnedComponent;

/// type
pub type OwnedCurrentComparisonComponent = *mut CurrentComparisonComponent;

/// Creates a new Current Comparison Component.
#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponent_new() -> OwnedCurrentComparisonComponent {
    alloc(CurrentComparisonComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponent_drop(this: OwnedCurrentComparisonComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponent_into_generic(
    this: OwnedCurrentComparisonComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponent_state_as_json(
    this: *mut CurrentComparisonComponent,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub unsafe extern "C" fn CurrentComparisonComponent_state(
    this: *mut CurrentComparisonComponent,
    timer: *const Timer,
) -> OwnedCurrentComparisonComponentState {
    alloc(acc_mut(this).state(acc(timer)))
}
