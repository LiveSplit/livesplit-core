//! The Current Comparison Component is a component that shows the name of the
//! comparison that is currently selected to be compared against.

use super::{Json, output_vec};
use crate::{component::OwnedComponent, key_value_component_state::OwnedKeyValueComponentState};
use livesplit_core::{
    Lang, Timer, component::current_comparison::Component as CurrentComparisonComponent,
};

/// type
pub type OwnedCurrentComparisonComponent = Box<CurrentComparisonComponent>;

/// Creates a new Current Comparison Component.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentComparisonComponent_new() -> OwnedCurrentComparisonComponent {
    Box::new(CurrentComparisonComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn CurrentComparisonComponent_drop(this: OwnedCurrentComparisonComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentComparisonComponent_into_generic(
    this: OwnedCurrentComparisonComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentComparisonComponent_state_as_json(
    this: &mut CurrentComparisonComponent,
    timer: &Timer,
    lang: Lang,
) -> Json {
    output_vec(|o| {
        this.state(timer, lang).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[unsafe(no_mangle)]
pub extern "C" fn CurrentComparisonComponent_state(
    this: &mut CurrentComparisonComponent,
    timer: &Timer,
    lang: Lang,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer, lang))
}
