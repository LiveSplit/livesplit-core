//! The Title Component is a component that shows the name of the game and the
//! category that is being run. Additionally, the game icon, the attempt count,
//! and the total number of successfully finished runs can be shown.

use super::{Json, output_vec};
use crate::{component::OwnedComponent, title_component_state::OwnedTitleComponentState};
use livesplit_core::{Timer, component::title::Component as TitleComponent, settings::ImageCache};

/// type
pub type OwnedTitleComponent = Box<TitleComponent>;

/// Creates a new Title Component.
#[unsafe(no_mangle)]
pub extern "C" fn TitleComponent_new() -> OwnedTitleComponent {
    Box::new(TitleComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn TitleComponent_drop(this: OwnedTitleComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn TitleComponent_into_generic(this: OwnedTitleComponent) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn TitleComponent_state_as_json(
    this: &mut TitleComponent,
    image_cache: &mut ImageCache,
    timer: &Timer,
) -> Json {
    output_vec(|o| {
        this.state(image_cache, timer).write_json(o).unwrap();
    })
}

/// Calculates the component's state based on the timer provided.
#[unsafe(no_mangle)]
pub extern "C" fn TitleComponent_state(
    this: &mut TitleComponent,
    image_cache: &mut ImageCache,
    timer: &Timer,
) -> OwnedTitleComponentState {
    Box::new(this.state(image_cache, timer))
}
