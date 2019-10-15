//! The PB Chance Component is a component that shows how likely it is to beat
//! the Personal Best. If there is no active attempt it shows the general chance
//! of beating the Personal Best. During an attempt it actively changes based on
//! how well the attempt is going.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::key_value_component_state::OwnedKeyValueComponentState;
use livesplit_core::component::pb_chance::Component as PbChanceComponent;
use livesplit_core::Timer;

/// type
pub type OwnedPbChanceComponent = Box<PbChanceComponent>;

/// Creates a new PB Chance Component.
#[no_mangle]
pub extern "C" fn PbChanceComponent_new() -> OwnedPbChanceComponent {
    Box::new(PbChanceComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn PbChanceComponent_drop(this: OwnedPbChanceComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn PbChanceComponent_into_generic(this: OwnedPbChanceComponent) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn PbChanceComponent_state_as_json(this: &PbChanceComponent, timer: &Timer) -> Json {
    output_vec(|o| {
        this.state(timer).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub extern "C" fn PbChanceComponent_state(
    this: &PbChanceComponent,
    timer: &Timer,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer))
}
