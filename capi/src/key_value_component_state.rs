//! The state object describes the information to visualize for a key value based component.

use super::output_str;
use livesplit_core::component::key_value::State as KeyValueComponentState;
use std::os::raw::c_char;

/// type
pub type OwnedKeyValueComponentState = Box<KeyValueComponentState>;

/// drop
#[no_mangle]
pub extern "C" fn KeyValueComponentState_drop(this: OwnedKeyValueComponentState) {
    drop(this);
}

/// The key to visualize.
#[no_mangle]
pub extern "C" fn KeyValueComponentState_key(this: &KeyValueComponentState) -> *const c_char {
    output_str(&this.key)
}

/// The value to visualize.
#[no_mangle]
pub extern "C" fn KeyValueComponentState_value(this: &KeyValueComponentState) -> *const c_char {
    output_str(&this.value)
}
