use livesplit_core::LayoutEditor;
use layout::OwnedLayout;
use super::{Json, alloc, own, output_vec, acc, acc_mut};
use component::OwnedComponent;
use setting_value::OwnedSettingValue;

pub type OwnedLayoutEditor = *mut LayoutEditor;

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_new(layout: OwnedLayout) -> OwnedLayoutEditor {
    alloc(LayoutEditor::new(own(layout)))
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_close(this: OwnedLayoutEditor) -> OwnedLayout {
    alloc(own(this).close())
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_state_as_json(this: *const LayoutEditor) -> Json {
    output_vec(|o| { acc(this).state().write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_select(this: *mut LayoutEditor, index: usize) {
    acc_mut(this).select(index);
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_add_component(
    this: *mut LayoutEditor,
    component: OwnedComponent,
) {
    acc_mut(this).add_component(own(component));
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_remove_component(this: *mut LayoutEditor) {
    acc_mut(this).remove_component();
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_move_component_up(this: *mut LayoutEditor) {
    acc_mut(this).move_component_up();
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_move_component_down(this: *mut LayoutEditor) {
    acc_mut(this).move_component_down();
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_move_component(this: *mut LayoutEditor, dst_index: usize) {
    acc_mut(this).move_component(dst_index);
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_component_settings_value(
    this: *mut LayoutEditor,
    index: usize,
    value: OwnedSettingValue,
) {
    acc_mut(this).set_component_settings_value(index, own(value));
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_general_settings_value(
    this: *mut LayoutEditor,
    index: usize,
    value: OwnedSettingValue,
) {
    acc_mut(this).set_general_settings_value(index, own(value));
}
