use livesplit_core::layout::editor::LayoutEditor;
use layout::OwnedLayout;
use super::{Json, alloc, own, output_vec, acc, acc_mut, str};
use component::OwnedComponent;
use livesplit_core::time_formatter::{Accuracy, DigitsFormat};
use libc::c_char;

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
pub unsafe extern "C" fn LayoutEditor_set_component_settings_bool(
    this: *mut LayoutEditor,
    index: usize,
    value: bool,
) {
    acc_mut(this).set_component_settings_value(index, value.into());
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_component_settings_uint(
    this: *mut LayoutEditor,
    index: usize,
    value: u64,
) {
    acc_mut(this).set_component_settings_value(index, value.into());
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_component_settings_int(
    this: *mut LayoutEditor,
    index: usize,
    value: i64,
) {
    acc_mut(this).set_component_settings_value(index, value.into());
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_component_settings_string(
    this: *mut LayoutEditor,
    index: usize,
    value: *const c_char,
) {
    acc_mut(this).set_component_settings_value(index, str(value).to_string().into());
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_component_settings_optional_string(
    this: *mut LayoutEditor,
    index: usize,
    value: *const c_char,
) {
    let value = if value.is_null() {
        None.into()
    } else {
        Some(str(value).to_string()).into()
    };
    acc_mut(this).set_component_settings_value(index, value);
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_component_settings_optional_string_to_empty(
    this: *mut LayoutEditor,
    index: usize,
) {
    acc_mut(this).set_component_settings_value(index, None::<String>.into());
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_component_settings_float(
    this: *mut LayoutEditor,
    index: usize,
    value: f64,
) {
    acc_mut(this).set_component_settings_value(index, value.into());
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_component_settings_accuracy(
    this: *mut LayoutEditor,
    index: usize,
    value: *const c_char,
) {
    let value = str(value);
    let value = match value {
        "Seconds" => Accuracy::Seconds,
        "Tenths" => Accuracy::Tenths,
        "Hundredths" => Accuracy::Hundredths,
        _ => return,
    };

    acc_mut(this).set_component_settings_value(index, value.into());
}

#[no_mangle]
pub unsafe extern "C" fn LayoutEditor_set_component_settings_digits_format(
    this: *mut LayoutEditor,
    index: usize,
    value: *const c_char,
) {
    let value = str(value);
    let value = match value {
        "SingleDigitSeconds" => DigitsFormat::SingleDigitSeconds,
        "DoubleDigitSeconds" => DigitsFormat::DoubleDigitSeconds,
        "SingleDigitMinutes" => DigitsFormat::SingleDigitMinutes,
        "DoubleDigitMinutes" => DigitsFormat::DoubleDigitMinutes,
        "SingleDigitHours" => DigitsFormat::SingleDigitHours,
        "DoubleDigitHours" => DigitsFormat::DoubleDigitHours,
        _ => return,
    };

    acc_mut(this).set_component_settings_value(index, value.into());
}
