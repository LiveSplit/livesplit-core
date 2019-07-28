// TODO:

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::pb_chance_component_state::OwnedPbChanceComponentState;
use livesplit_core::component::pb_chance::Component as PbChanceComponent;
use livesplit_core::Timer;

/// type
pub type OwnedPbChanceComponent = Box<PbChanceComponent>;

/// Creates a new Possible Time Save Component.
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
) -> OwnedPbChanceComponentState {
    Box::new(this.state(timer))
}
