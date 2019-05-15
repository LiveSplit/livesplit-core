//! The Text Component simply visualizes any given text. This can either be a
//! single centered text, or split up into a left and right text, which is
//! suitable for a situation where you have a label and a value.

use super::{output_vec, str, Json};
use crate::component::OwnedComponent;
use crate::text_component_state::OwnedTextComponentState;
use livesplit_core::{component::text::Component as TextComponent, Timer};
use std::os::raw::c_char;

/// type
pub type OwnedTextComponent = Box<TextComponent>;

/// Creates a new Text Component.
#[no_mangle]
pub extern "C" fn TextComponent_new() -> OwnedTextComponent {
    Box::new(TextComponent::new())
}

/// drop
#[no_mangle]
pub extern "C" fn TextComponent_drop(this: OwnedTextComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub extern "C" fn TextComponent_into_generic(this: OwnedTextComponent) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub extern "C" fn TextComponent_state_as_json(this: &TextComponent, timer: &Timer) -> Json {
    output_vec(|o| {
        this.state(timer).write_json(o).unwrap();
    })
}

/// Sets the centered text. If the current mode is split, it is switched to
/// centered mode.
#[no_mangle]
pub unsafe extern "C" fn TextComponent_set_center(this: &mut TextComponent, text: *const c_char) {
    this.settings_mut().text.set_center(str(text));
}

/// Sets the left text. If the current mode is centered, it is switched to
/// split mode, with the right text being empty.
#[no_mangle]
pub unsafe extern "C" fn TextComponent_set_left(this: &mut TextComponent, text: *const c_char) {
    this.settings_mut().text.set_left(str(text));
}

/// Sets the right text. If the current mode is centered, it is switched to
/// split mode, with the left text being empty.
#[no_mangle]
pub unsafe extern "C" fn TextComponent_set_right(this: &mut TextComponent, text: *const c_char) {
    this.settings_mut().text.set_right(str(text));
}

/// Calculates the component's state.
#[no_mangle]
pub extern "C" fn TextComponent_state(
    this: &TextComponent,
    timer: &Timer,
) -> OwnedTextComponentState {
    Box::new(this.state(timer))
}
