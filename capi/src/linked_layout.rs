//! A Linked Layout associates a Layout with a Run. If the Run has a Linked
//! Layout, it is supposed to be visualized with the Layout that is linked with
//! it.

use super::{output_str, str};
use livesplit_core::run::LinkedLayout;
use std::os::raw::c_char;

/// type
pub type OwnedLinkedLayout = Box<LinkedLayout>;
/// type
pub type NullableOwnedLinkedLayout = Option<OwnedLinkedLayout>;

/// Creates a new Linked Layout with the path specified. If the path is empty,
/// the default layout is used instead.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkedLayout_new(path: *const c_char) -> OwnedLinkedLayout {
    // SAFETY: The caller guarantees that `path` is valid.
    let path = unsafe { str(path) };
    Box::new(if path.is_empty() {
        LinkedLayout::Default
    } else {
        LinkedLayout::Path(path.to_owned())
    })
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn LinkedLayout_drop(this: OwnedLinkedLayout) {
    drop(this);
}

/// Checks whether the linked layout is the default layout.
#[unsafe(no_mangle)]
pub extern "C" fn LinkedLayout_is_default(this: &LinkedLayout) -> bool {
    matches!(this, LinkedLayout::Default)
}

/// Returns the path of the linked layout, if it's not the default layout.
#[unsafe(no_mangle)]
pub extern "C" fn LinkedLayout_path(this: &LinkedLayout) -> *const c_char {
    output_str(match this {
        LinkedLayout::Path(path) => path,
        _ => "",
    })
}
