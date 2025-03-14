//! Represents the current state of the Layout Editor in order to visualize it properly.
use crate::{c_char, output_str};
use livesplit_core::{layout::editor::State as LayoutEditorState, settings::Value as SettingValue};

/// type
pub type OwnedLayoutEditorState = Box<LayoutEditorState>;
/// type
pub type NullableOwnedLayoutEditorState = Option<OwnedLayoutEditorState>;

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn LayoutEditorState_drop(this: OwnedLayoutEditorState) {
    drop(this);
}

/// Returns the number of components in the layout.
#[unsafe(no_mangle)]
pub extern "C" fn LayoutEditorState_component_len(this: &LayoutEditorState) -> usize {
    this.components.len()
}

/// Returns the name of the component at the specified index.
#[unsafe(no_mangle)]
pub extern "C" fn LayoutEditorState_component_text(
    this: &LayoutEditorState,
    index: usize,
) -> *const c_char {
    output_str(&this.components[index])
}

/// Returns a bitfield corresponding to which buttons are active.
///
/// The bits are as follows:
///
/// * `0x04` - Can remove the current component
/// * `0x02` - Can move the current component up
/// * `0x01` - Can move the current component down
#[unsafe(no_mangle)]
pub extern "C" fn LayoutEditorState_buttons(this: &LayoutEditorState) -> u8 {
    ((this.buttons.can_remove as u8) << 2)
        | ((this.buttons.can_move_up as u8) << 1)
        | this.buttons.can_move_down as u8
}

/// Returns the index of the currently selected component.
#[unsafe(no_mangle)]
pub extern "C" fn LayoutEditorState_selected_component(this: &LayoutEditorState) -> u32 {
    this.selected_component
}

/// Returns the number of fields in the layout's settings.
///
/// Set `component_settings` to true to use the selected component's settings instead.
#[unsafe(no_mangle)]
pub extern "C" fn LayoutEditorState_field_len(
    this: &LayoutEditorState,
    component_settings: bool,
) -> usize {
    if component_settings {
        this.component_settings.fields.len()
    } else {
        this.general_settings.fields.len()
    }
}

/// Returns the name of the layout's setting at the specified index.
///
/// Set `component_settings` to true to use the selected component's settings instead.
#[unsafe(no_mangle)]
pub extern "C" fn LayoutEditorState_field_text(
    this: &LayoutEditorState,
    component_settings: bool,
    index: usize,
) -> *const c_char {
    if component_settings {
        output_str(&this.component_settings.fields[index].text)
    } else {
        output_str(&this.general_settings.fields[index].text)
    }
}

/// Returns the value of the layout's setting at the specified index.
///
/// Set `component_settings` to true to use the selected component's settings instead.
#[unsafe(no_mangle)]
pub extern "C" fn LayoutEditorState_field_value(
    this: &LayoutEditorState,
    component_settings: bool,
    index: usize,
) -> &SettingValue {
    if component_settings {
        &this.component_settings.fields[index].value
    } else {
        &this.general_settings.fields[index].value
    }
}
