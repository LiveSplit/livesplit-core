//! The Text Component simply visualizes any given text. This can either be a
//! single centered text, or split up into a left and right text, which is
//! suitable for a situation where you have a label and a value.

use super::{Json, output_vec, str};
use crate::{component::OwnedComponent, text_component_state::OwnedTextComponentState};
use livesplit_core::{
    Timer,
    component::text::{Component as TextComponent, Text},
};
use std::os::raw::c_char;

/// type
pub type OwnedTextComponent = Box<TextComponent>;

/// Creates a new Text Component.
#[unsafe(no_mangle)]
pub extern "C" fn TextComponent_new() -> OwnedTextComponent {
    Box::new(TextComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn TextComponent_drop(this: OwnedTextComponent) {
    drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[unsafe(no_mangle)]
pub extern "C" fn TextComponent_into_generic(this: OwnedTextComponent) -> OwnedComponent {
    Box::new((*this).into())
}

/// Encodes the component's state information as JSON.
#[unsafe(no_mangle)]
pub extern "C" fn TextComponent_state_as_json(this: &TextComponent, timer: &Timer) -> Json {
    output_vec(|o| {
        this.state(timer).write_json(o).unwrap();
    })
}

/// Switches the component to display the specified custom variable instead of a
/// fixed text. The boolean indicates whether the name should also be shown as a
/// key value pair.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TextComponent_use_variable(
    this: &mut TextComponent,
    variable: *const c_char,
    split: bool,
) {
    // SAFETY: The caller guarantees that `variable` is valid.
    this.settings_mut().text = Text::Variable(unsafe { str(variable).into() }, split);
}

/// Sets the centered text. If the current mode is split, it is switched to
/// centered mode.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TextComponent_set_center(this: &mut TextComponent, text: *const c_char) {
    // SAFETY: The caller guarantees that `text` is valid.
    this.settings_mut().text.set_center(unsafe { str(text) });
}

/// Sets the left text. If the current mode is centered, it is switched to
/// split mode, with the right text being empty.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TextComponent_set_left(this: &mut TextComponent, text: *const c_char) {
    // SAFETY: The caller guarantees that `text` is valid.
    this.settings_mut().text.set_left(unsafe { str(text) });
}

/// Sets the right text. If the current mode is centered, it is switched to
/// split mode, with the left text being empty.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TextComponent_set_right(this: &mut TextComponent, text: *const c_char) {
    // SAFETY: The caller guarantees that `text` is valid.
    this.settings_mut().text.set_right(unsafe { str(text) });
}

/// Calculates the component's state.
#[unsafe(no_mangle)]
pub extern "C" fn TextComponent_state(
    this: &TextComponent,
    timer: &Timer,
) -> OwnedTextComponentState {
    Box::new(this.state(timer))
}
