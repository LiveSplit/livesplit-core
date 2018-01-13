//! A Layout allows you to combine multiple components together to visualize a
//! variety of information the runner is interested in.

use livesplit_core::{Layout, Timer};
use livesplit_core::layout::LayoutSettings;
use super::{acc, acc_mut, alloc, output_vec, own, own_drop, str, Json};
use component::OwnedComponent;
use std::io::Cursor;
use std::ptr;

/// type
pub type OwnedLayout = *mut Layout;
/// type
pub type NullableOwnedLayout = OwnedLayout;

/// Creates a new empty layout with no components.
#[no_mangle]
pub unsafe extern "C" fn Layout_new() -> OwnedLayout {
    alloc(Layout::new())
}

/// Creates a new default layout that contains a default set of components
/// in order to provide a good default layout for runners. Which components
/// are provided by this and how they are configured may change in the
/// future.
#[no_mangle]
pub unsafe extern "C" fn Layout_default_layout() -> OwnedLayout {
    alloc(Layout::default_layout())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn Layout_drop(this: OwnedLayout) {
    own_drop(this);
}

/// Clones the layout.
#[no_mangle]
pub unsafe extern "C" fn Layout_clone(this: *const Layout) -> OwnedLayout {
    alloc(acc(this).clone())
}

/// Parses a layout from the given JSON description of its settings. <NULL> is
/// returned if it couldn't be parsed.
#[no_mangle]
pub unsafe extern "C" fn Layout_parse_json(settings: Json) -> NullableOwnedLayout {
    let settings = Cursor::new(str(settings).as_bytes());
    if let Ok(settings) = LayoutSettings::from_json(settings) {
        alloc(Layout::from_settings(settings))
    } else {
        ptr::null_mut()
    }
}

/// Calculates the layout's state based on the timer provided and encodes it as
/// JSON. You can use this to visualize all of the components of a layout.
#[no_mangle]
pub unsafe extern "C" fn Layout_state_as_json(this: *mut Layout, timer: *const Timer) -> Json {
    output_vec(|o| {
        acc_mut(this).state(acc(timer)).write_json(o).unwrap();
    })
}

/// Encodes the settings of the layout as JSON.
#[no_mangle]
pub unsafe extern "C" fn Layout_settings_as_json(this: *const Layout) -> Json {
    output_vec(|o| {
        acc(this).settings().write_json(o).unwrap();
    })
}

/// Adds a new component to the end of the layout.
#[no_mangle]
pub unsafe extern "C" fn Layout_push(this: *mut Layout, component: OwnedComponent) {
    acc_mut(this).push(own(component));
}

/// Scrolls up all the components in the layout that can be scrolled up.
#[no_mangle]
pub unsafe extern "C" fn Layout_scroll_up(this: *mut Layout) {
    acc_mut(this).scroll_up();
}

/// Scrolls down all the components in the layout that can be scrolled down.
#[no_mangle]
pub unsafe extern "C" fn Layout_scroll_down(this: *mut Layout) {
    acc_mut(this).scroll_down();
}

/// Remounts all the components as if they were freshly initialized. Some
/// components may only provide some information whenever it changes or when
/// their state is first queried. Remounting returns this information again,
/// whenever the layout's state is queried the next time.
#[no_mangle]
pub unsafe extern "C" fn Layout_remount(this: *mut Layout) {
    acc_mut(this).remount();
}
