//! The Title Component is a component that shows the name of the game and the
//! category that is being run. Additionally, the game icon, the attempt count,
//! and the total number of successfully finished runs can be shown.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::title_component_state::OwnedTitleComponentState;
use livesplit_core::component::title::Component as TitleComponent;
use livesplit_core::Timer;

/// type
pub type OwnedTitleComponent = Box<TitleComponent>;

/// Creates a new Title Component.
#[no_mangle]
pub extern "C" fn TitleComponent_new() -> OwnedTitleComponent {
    Box::new(TitleComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn TitleComponent_drop(this: OwnedTitleComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn TitleComponent_into_generic(this: OwnedTitleComponent) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn TitleComponent_state_as_json(this: &mut TitleComponent, timer: &Timer) -> Json {
    output_vec(|o| {
        this.state(timer).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[no_mangle]
pub extern "C" fn TitleComponent_state(
    this: &mut TitleComponent,
    timer: &Timer,
) -> OwnedTitleComponentState {
    Box::new(this.state(timer))
}
