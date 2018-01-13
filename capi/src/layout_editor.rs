//! The Layout Editor allows modifying Layouts while ensuring all the different
//! invariants of the Layout objects are upheld no matter what kind of
//! operations are being applied. It provides the current state of the editor as
//! state objects that can be visualized by any kind of User Interface.

use livesplit_core::{LayoutEditor, Timer};
use layout::OwnedLayout;
use super::{acc, acc_mut, alloc, output_vec, own, Json};
use component::OwnedComponent;
use setting_value::OwnedSettingValue;
use std::ptr;

/// type
pub type OwnedLayoutEditor = *mut LayoutEditor;
/// type
pub type NullableOwnedLayoutEditor = OwnedLayoutEditor;

/// Creates a new Layout Editor that modifies the Layout provided. Creation of
/// the Layout Editor fails when a Layout with no components is provided. In
/// that case <NULL> is returned instead.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_new(layout: OwnedLayout) -> NullableOwnedLayoutEditor {
    LayoutEditor::new(own(layout))
        .ok()
        .map_or_else(ptr::null_mut, alloc)
}

/// Closes the Layout Editor and gives back access to the modified Layout. In
/// case you want to implement a Cancel Button, just dispose the Layout object
/// you get here.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_close(this: OwnedLayoutEditor) -> OwnedLayout {
    alloc(own(this).close())
}

/// Encodes the Layout Editor's state as JSON in order to visualize it.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_state_as_json(this: *const LayoutEditor) -> Json {
    output_vec(|o| {
        acc(this).state().write_json(o).unwrap();
    })
}

/// Encodes the layout's state as JSON based on the timer provided. You can use
/// this to visualize all of the components of a layout, while it is still being
/// edited by the Layout Editor.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_layout_state_as_json(
    this: *mut LayoutEditor,
    timer: *const Timer,
) -> Json {
    output_vec(|o| {
        acc_mut(this)
            .layout_state(acc(timer))
            .write_json(o)
            .unwrap();
    })
}

/// Selects the component with the given index in order to modify its
/// settings. Only a single component is selected at any given time. You may
/// not provide an invalid index.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_select(this: *mut LayoutEditor, index: usize) {
    acc_mut(this).select(index);
}

/// Adds the component provided to the end of the layout. The newly added
/// component becomes the selected component.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_add_component(
    this: *mut LayoutEditor,
    component: OwnedComponent,
) {
    acc_mut(this).add_component(own(component));
}

/// Removes the currently selected component, unless there's only one
/// component in the layout. The next component becomes the selected
/// component. If there's none, the previous component becomes the selected
/// component instead.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_remove_component(this: *mut LayoutEditor) {
    acc_mut(this).remove_component();
}

/// Moves the selected component up, unless the first component is selected.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_move_component_up(this: *mut LayoutEditor) {
    acc_mut(this).move_component_up();
}

/// Moves the selected component down, unless the last component is
/// selected.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_move_component_down(this: *mut LayoutEditor) {
    acc_mut(this).move_component_down();
}

/// Moves the selected component to the index provided. You may not provide
/// an invalid index.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_move_component(this: *mut LayoutEditor, dst_index: usize) {
    acc_mut(this).move_component(dst_index);
}

/// Duplicates the currently selected component. The copy gets placed right
/// after the selected component and becomes the newly selected component.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_duplicate_component(this: *mut LayoutEditor) {
    acc_mut(this).duplicate_component();
}

/// Sets a setting's value of the selected component by its setting index
/// to the given value.
///
/// This panics if the type of the value to be set is not compatible with
/// the type of the setting's value. A panic can also occur if the index of
/// the setting provided is out of bounds.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_component_settings_value(
    this: *mut LayoutEditor,
    index: usize,
    value: OwnedSettingValue,
) {
    acc_mut(this).set_component_settings_value(index, own(value));
}

/// Sets a setting's value of the general settings by its setting index to
/// the given value.
///
/// This panics if the type of the value to be set is not compatible with
/// the type of the setting's value. A panic can also occur if the index of
/// the setting provided is out of bounds.
#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_general_settings_value(
    this: *mut LayoutEditor,
    index: usize,
    value: OwnedSettingValue,
) {
    acc_mut(this).set_general_settings_value(index, own(value));
}
