//! The Layout Editor allows modifying Layouts while ensuring all the different
//! invariants of the Layout objects are upheld no matter what kind of
//! operations are being applied. It provides the current state of the editor as
//! state objects that can be visualized by any kind of User Interface.

use super::{output_vec, Json};
use crate::component::OwnedComponent;
use crate::layout::OwnedLayout;
use crate::setting_value::OwnedSettingValue;
use livesplit_core::{LayoutEditor, Timer};

/// type
pub type OwnedLayoutEditor = Box<LayoutEditor>;
/// type
pub type NullableOwnedLayoutEditor = Option<OwnedLayoutEditor>;

/// Creates a new Layout Editor that modifies the Layout provided. Creation of
/// the Layout Editor fails when a Layout with no components is provided. In
/// that case <NULL> is returned instead.
#[no_mangle]
pub extern "C" fn LayoutEditor_new(layout: OwnedLayout) -> NullableOwnedLayoutEditor {
    LayoutEditor::new(*layout).ok().map(Box::new)
}

/// Closes the Layout Editor and gives back access to the modified Layout. In
/// case you want to implement a Cancel Button, just dispose the Layout object
/// you get here.
#[no_mangle]
pub extern "C" fn LayoutEditor_close(this: OwnedLayoutEditor) -> OwnedLayout {
    Box::new((*this).close())
}

/// Encodes the Layout Editor's state as JSON in order to visualize it.
#[no_mangle]
pub extern "C" fn LayoutEditor_state_as_json(this: &LayoutEditor) -> Json {
    output_vec(|o| {
        this.state().write_json(o).unwrap();
    })
}

/// Encodes the layout's state as JSON based on the timer provided. You can use
/// this to visualize all of the components of a layout, while it is still being
/// edited by the Layout Editor.
#[no_mangle]
pub extern "C" fn LayoutEditor_layout_state_as_json(
    this: &mut LayoutEditor,
    timer: &Timer,
) -> Json {
    output_vec(|o| {
        this.layout_state(timer).write_json(o).unwrap();
    })
}

/// Selects the component with the given index in order to modify its
/// settings. Only a single component is selected at any given time. You may
/// not provide an invalid index.
#[no_mangle]
pub extern "C" fn LayoutEditor_select(this: &mut LayoutEditor, index: usize) {
    this.select(index);
}

/// Adds the component provided to the end of the layout. The newly added
/// component becomes the selected component.
#[no_mangle]
pub extern "C" fn LayoutEditor_add_component(this: &mut LayoutEditor, component: OwnedComponent) {
    this.add_component(*component);
}

/// Removes the currently selected component, unless there's only one
/// component in the layout. The next component becomes the selected
/// component. If there's none, the previous component becomes the selected
/// component instead.
#[no_mangle]
pub extern "C" fn LayoutEditor_remove_component(this: &mut LayoutEditor) {
    this.remove_component();
}

/// Moves the selected component up, unless the first component is selected.
#[no_mangle]
pub extern "C" fn LayoutEditor_move_component_up(this: &mut LayoutEditor) {
    this.move_component_up();
}

/// Moves the selected component down, unless the last component is
/// selected.
#[no_mangle]
pub extern "C" fn LayoutEditor_move_component_down(this: &mut LayoutEditor) {
    this.move_component_down();
}

/// Moves the selected component to the index provided. You may not provide
/// an invalid index.
#[no_mangle]
pub extern "C" fn LayoutEditor_move_component(this: &mut LayoutEditor, dst_index: usize) {
    this.move_component(dst_index);
}

/// Duplicates the currently selected component. The copy gets placed right
/// after the selected component and becomes the newly selected component.
#[no_mangle]
pub extern "C" fn LayoutEditor_duplicate_component(this: &mut LayoutEditor) {
    this.duplicate_component();
}

/// Sets a setting's value of the selected component by its setting index
/// to the given value.
///
/// This panics if the type of the value to be set is not compatible with
/// the type of the setting's value. A panic can also occur if the index of
/// the setting provided is out of bounds.
#[no_mangle]
pub extern "C" fn LayoutEditor_set_component_settings_value(
    this: &mut LayoutEditor,
    index: usize,
    value: OwnedSettingValue,
) {
    this.set_component_settings_value(index, *value);
}

/// Sets a setting's value of the general settings by its setting index to
/// the given value.
///
/// This panics if the type of the value to be set is not compatible with
/// the type of the setting's value. A panic can also occur if the index of
/// the setting provided is out of bounds.
#[no_mangle]
pub extern "C" fn LayoutEditor_set_general_settings_value(
    this: &mut LayoutEditor,
    index: usize,
    value: OwnedSettingValue,
) {
    this.set_general_settings_value(index, *value);
}
