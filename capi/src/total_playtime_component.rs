//! The Total Playtime Component is a component that shows the total amount of
//! time that the current category has been played for.

use super::{Json, output_vec};
use crate::{component::OwnedComponent, key_value_component_state::OwnedKeyValueComponentState};
use livesplit_core::{Lang, Timer, component::total_playtime::Component as TotalPlaytimeComponent};

/// type
pub type OwnedTotalPlaytimeComponent = Box<TotalPlaytimeComponent>;

/// Creates a new Total Playtime Component.
#[unsafe(no_mangle)]
pub extern "C" fn TotalPlaytimeComponent_new() -> OwnedTotalPlaytimeComponent {
    Box::new(TotalPlaytimeComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn TotalPlaytimeComponent_drop(this: OwnedTotalPlaytimeComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn TotalPlaytimeComponent_into_generic(
    this: OwnedTotalPlaytimeComponent,
) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn TotalPlaytimeComponent_state_as_json(
    this: &mut TotalPlaytimeComponent,
    timer: &Timer,
    lang: Lang,
) -> Json {
    output_vec(|o| {
        this.state(timer, lang).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[unsafe(no_mangle)]
pub extern "C" fn TotalPlaytimeComponent_state(
    this: &mut TotalPlaytimeComponent,
    timer: &Timer,
    lang: Lang,
) -> OwnedKeyValueComponentState {
    Box::new(this.state(timer, lang))
}
