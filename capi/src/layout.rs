//! A Layout allows you to combine multiple components together to visualize a
//! variety of information the runner is interested in.

use super::{get_file, output_vec, release_file, str, Json};
use crate::component::OwnedComponent;
use crate::layout_state::OwnedLayoutState;
use livesplit_core::layout::{parser, LayoutSettings};
use livesplit_core::{Layout, Timer};
use std::io::{BufReader, Cursor};
use std::slice;

/// type
pub type OwnedLayout = Box<Layout>;
/// type
pub type NullableOwnedLayout = Option<OwnedLayout>;

/// Creates a new empty layout with no components.
#[no_mangle]
pub extern "C" fn Layout_new() -> OwnedLayout {
    Box::new(Layout::new())
}

/// Creates a new default layout that contains a default set of components
/// in order to provide a good default layout for runners. Which components
/// are provided by this and how they are configured may change in the
/// future.
#[no_mangle]
pub extern "C" fn Layout_default_layout() -> OwnedLayout {
    Box::new(Layout::default_layout())
}

/// drop
#[no_mangle]
pub extern "C" fn Layout_drop(this: OwnedLayout) {
    drop(this);
}

/// Clones the layout.
#[no_mangle]
pub extern "C" fn Layout_clone(this: &Layout) -> OwnedLayout {
    Box::new(this.clone())
}

/// Parses a layout from the given JSON description of its settings. <NULL> is
/// returned if it couldn't be parsed.
#[no_mangle]
pub unsafe extern "C" fn Layout_parse_json(settings: Json) -> NullableOwnedLayout {
    let settings = Cursor::new(str(settings).as_bytes());
    if let Ok(settings) = LayoutSettings::from_json(settings) {
        Some(Box::new(Layout::from_settings(settings)))
    } else {
        None
    }
}

/// Attempts to parse a layout from a given file. <NULL> is returned it couldn't
/// be parsed. This will not close the file descriptor / handle.
#[no_mangle]
pub unsafe extern "C" fn Layout_parse_file_handle(handle: i64) -> NullableOwnedLayout {
    let file = get_file(handle);

    let reader = BufReader::new(&file);

    let layout = if let Ok(settings) = LayoutSettings::from_json(reader) {
        Some(Box::new(Layout::from_settings(settings)))
    } else {
        None
    };

    release_file(file);

    layout
}

/// Parses a layout saved by the original LiveSplit. This is lossy, as not
/// everything can be converted completely. <NULL> is returned if it couldn't be
/// parsed at all.
#[no_mangle]
pub unsafe extern "C" fn Layout_parse_original_livesplit(
    data: *const u8,
    length: usize,
) -> NullableOwnedLayout {
    if let Ok(parsed) = parser::parse(Cursor::new(slice::from_raw_parts(data, length))) {
        Some(Box::new(parsed))
    } else {
        None
    }
}

/// Calculates and returns the layout's state based on the timer provided.
#[no_mangle]
pub extern "C" fn Layout_state(this: &mut Layout, timer: &Timer) -> OwnedLayoutState {
    Box::new(this.state(timer))
}

/// Calculates the layout's state based on the timer provided and encodes it as
/// JSON. You can use this to visualize all of the components of a layout.
#[no_mangle]
pub extern "C" fn Layout_state_as_json(this: &mut Layout, timer: &Timer) -> Json {
    output_vec(|o| {
        this.state(timer).write_json(o).unwrap();
    })
}

/// Encodes the settings of the layout as JSON.
#[no_mangle]
pub extern "C" fn Layout_settings_as_json(this: &Layout) -> Json {
    output_vec(|o| {
        this.settings().write_json(o).unwrap();
    })
}

/// Adds a new component to the end of the layout.
#[no_mangle]
pub extern "C" fn Layout_push(this: &mut Layout, component: OwnedComponent) {
    this.push(*component);
}

/// Scrolls up all the components in the layout that can be scrolled up.
#[no_mangle]
pub extern "C" fn Layout_scroll_up(this: &mut Layout) {
    this.scroll_up();
}

/// Scrolls down all the components in the layout that can be scrolled down.
#[no_mangle]
pub extern "C" fn Layout_scroll_down(this: &mut Layout) {
    this.scroll_down();
}

/// Remounts all the components as if they were freshly initialized. Some
/// components may only provide some information whenever it changes or when
/// their state is first queried. Remounting returns this information again,
/// whenever the layout's state is queried the next time.
#[no_mangle]
pub extern "C" fn Layout_remount(this: &mut Layout) {
    this.remount();
}
