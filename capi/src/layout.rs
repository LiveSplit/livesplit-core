use livesplit_core::{Layout, Timer};
use livesplit_core::layout::LayoutSettings;
use super::{Json, alloc, own, own_drop, acc, acc_mut, output_vec, str};
use component::OwnedComponent;
use std::io::Cursor;
use std::ptr;

pub type OwnedLayout = *mut Layout;

#[no_mangle]
pub unsafe extern "C" fn Layout_new() -> OwnedLayout {
    alloc(Layout::new())
}

#[no_mangle]
pub unsafe extern "C" fn Layout_default_layout() -> OwnedLayout {
    alloc(Layout::default_layout())
}

#[no_mangle]
pub unsafe extern "C" fn Layout_drop(this: OwnedLayout) {
    own_drop(this);
}

#[no_mangle]
pub unsafe extern "C" fn Layout_clone(this: *const Layout) -> OwnedLayout {
    alloc(acc(this).clone())
}

#[no_mangle]
pub unsafe extern "C" fn Layout_parse_json(settings: Json) -> OwnedLayout {
    let settings = Cursor::new(str(settings).as_bytes());
    if let Ok(settings) = LayoutSettings::from_json(settings) {
        alloc(Layout::from_settings(settings))
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn Layout_state_as_json(this: *mut Layout, timer: *const Timer) -> Json {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

#[no_mangle]
pub unsafe extern "C" fn Layout_settings_as_json(this: *const Layout) -> Json {
    output_vec(|o| { acc(this).settings().write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn Layout_push(this: *mut Layout, component: OwnedComponent) {
    acc_mut(this).push(own(component));
}

#[no_mangle]
pub unsafe extern "C" fn Layout_scroll_up(this: *mut Layout) {
    acc_mut(this).scroll_up();
}

#[no_mangle]
pub unsafe extern "C" fn Layout_scroll_down(this: *mut Layout) {
    acc_mut(this).scroll_down();
}

#[no_mangle]
pub unsafe extern "C" fn Layout_remount(this: *mut Layout) {
    acc_mut(this).remount();
}
