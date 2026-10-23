//! With a Hotkey System the runner can use hotkeys on their keyboard to control
//! the Timer. The hotkeys are global, so the application doesn't need to be in
//! focus. The behavior of the hotkeys depends on the platform and is stubbed
//! out on platforms that don't support hotkeys. You can turn off a Hotkey
//! System temporarily. By default the Hotkey System is activated.

use std::{fmt::Display, io::Write, os::raw::c_char, str::FromStr};

use crate::{
    command_sink::CommandSink, hotkey_config::OwnedHotkeyConfig, output_str, output_vec, str,
};
use livesplit_core::{
    Lang,
    hotkey::{Hotkey, KeyCode},
};

type HotkeySystem = livesplit_core::HotkeySystem<CommandSink>;

/// type
pub type OwnedHotkeySystem = Box<HotkeySystem>;
/// type
pub type NullableOwnedHotkeySystem = Option<OwnedHotkeySystem>;

/// Creates a new Hotkey System for a Timer with the default hotkeys.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeySystem_new(command_sink: &CommandSink) -> NullableOwnedHotkeySystem {
    HotkeySystem::new(command_sink.clone()).ok().map(Box::new)
}

/// Creates a new Hotkey System for a Timer with a custom configuration for the
/// hotkeys.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeySystem_with_config(
    command_sink: &CommandSink,
    config: OwnedHotkeyConfig,
) -> NullableOwnedHotkeySystem {
    HotkeySystem::with_config(command_sink.clone(), *config)
        .ok()
        .map(Box::new)
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn HotkeySystem_drop(this: OwnedHotkeySystem) {
    drop(this);
}

/// Deactivates the Hotkey System. No hotkeys will go through until it gets
/// activated again. If it's already deactivated, nothing happens.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeySystem_deactivate(this: &mut HotkeySystem) -> bool {
    this.deactivate().is_ok()
}

/// Activates a previously deactivated Hotkey System. If it's already
/// active, nothing happens.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeySystem_activate(this: &mut HotkeySystem) -> bool {
    this.activate().is_ok()
}

/// Returns the hotkey configuration currently in use by the Hotkey System.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeySystem_config(this: &HotkeySystem) -> OwnedHotkeyConfig {
    Box::new(this.config())
}

/// Applies a new hotkey configuration to the Hotkey System. Each hotkey is
/// changed to the one specified in the configuration. This operation may fail
/// if you provide a hotkey configuration where a hotkey is used for multiple
/// operations. Returns <FALSE> if the operation failed.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeySystem_set_config(
    this: &mut HotkeySystem,
    config: OwnedHotkeyConfig,
) -> bool {
    this.set_config(*config).is_ok()
}

/// Resolves a key's stable identifier into a display label using the supplied
/// language, native platform conventions and current keyboard layout. Unknown
/// identifiers are returned unchanged. The result is display text only and must
/// not be stored in a hotkey configuration.
///
/// # Safety
/// `key_code` must point to a valid, NUL-terminated UTF-8 string, or be <NULL>.
/// The returned string uses the C API's temporary output buffer and must be
/// copied before another API call reuses that buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HotkeySystem_resolve_key(
    this: &HotkeySystem,
    key_code: *const c_char,
    lang: Lang,
) -> *const c_char {
    // SAFETY: The caller guarantees that `key_code` is valid or null.
    resolve(unsafe { str(key_code) }, |key: KeyCode| {
        this.resolve_key(key, lang)
    })
}

/// Resolves a complete hotkey's stable identifier, including its modifiers,
/// into display text using the supplied language, native platform conventions
/// and current keyboard layout. Unknown identifiers are returned unchanged.
/// The result must not be parsed or stored in a hotkey configuration.
///
/// # Safety
/// `hotkey` must point to a valid, NUL-terminated UTF-8 string, or be <NULL>.
/// The returned string uses the C API's temporary output buffer and must be
/// copied before another API call reuses that buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HotkeySystem_resolve_hotkey(
    this: &HotkeySystem,
    hotkey: *const c_char,
    lang: Lang,
) -> *const c_char {
    // SAFETY: The caller guarantees that `hotkey` is valid or null.
    resolve(unsafe { str(hotkey) }, |hotkey: Hotkey| {
        this.resolve_hotkey(hotkey, lang)
    })
}

// Keep parsing at the foreign-language boundary: Rust callers already have
// typed keys and bindings. Unknown identifiers may come from newer clients,
// so preserve them verbatim rather than inventing an English display fallback.
fn resolve<T: FromStr, D: Display>(name: &str, display: impl FnOnce(T) -> D) -> *const c_char {
    match name.parse() {
        Ok(value) => output_vec(|output| {
            // Reuse the C API's output allocation. The built-in display values
            // only forward formatter errors, and writing to a Vec cannot fail.
            write!(output, "{}", display(value))
                .expect("formatting a hotkey into the output Vec cannot fail");
        }),
        Err(_) => output_str(name),
    }
}

#[cfg(all(target_family = "wasm", feature = "wasm-web"))]
use wasm_bindgen::prelude::*;

/// On the web you can use this to listen to keyboard events on an additional
/// child window as well.
#[cfg(all(target_family = "wasm", feature = "wasm-web"))]
#[wasm_bindgen]
pub unsafe fn HotkeySystem_add_window(
    hotkey_system: *const HotkeySystem,
    window: web_sys::Window,
) -> bool {
    // SAFETY: The caller guarantees that `hotkey_system` is valid.
    unsafe { (*hotkey_system).add_window(window).is_ok() }
}

#[cfg(test)]
mod tests {
    use crate::hotkey_system::resolve;
    use livesplit_core::{
        Lang,
        hotkey::{Hotkey, KeyCode, display},
    };
    use std::{ffi::CStr, os::raw::c_char};

    #[track_caller]
    fn read_output(output: *const c_char) -> String {
        // SAFETY: Each test passes the output buffer directly from `resolve`
        // and copies its contents before calling another C API function.
        unsafe { CStr::from_ptr(output) }
            .to_str()
            .unwrap()
            .to_owned()
    }

    #[test]
    fn key_and_binding_output_use_the_requested_language() {
        for locale in [
            "en", "nl", "fr", "de", "it", "pt", "pl", "ru", "es", "pt-BR", "zh-Hans", "zh-Hant",
            "ja", "ko",
        ] {
            let lang = Lang::parse_locale(locale);
            let key = read_output(resolve("Numpad1", |key: KeyCode| {
                display::Key::new(key, lang)
            }));
            let binding = read_output(resolve("Ctrl + Numpad1", |hotkey: Hotkey| {
                display::Binding::new(hotkey, lang)
            }));
            assert_eq!(key, display::Key::new(KeyCode::Numpad1, lang).to_string());
            assert!(binding.contains(&key));
        }

        #[cfg(feature = "localization")]
        {
            let english = read_output(resolve("Numpad1", |key: KeyCode| {
                display::Key::new(key, Lang::English)
            }));
            let german = read_output(resolve("Numpad1", |key: KeyCode| {
                display::Key::new(key, Lang::German)
            }));
            assert_eq!(english, "Numpad 1");
            assert_eq!(german, "Num 1");
        }
    }

    #[test]
    fn unknown_and_empty_identifiers_are_preserved_without_resolving() {
        for name in ["", "FutureKey", "Ctrl + FutureKey", "Contrôle + KeyA"] {
            let key = read_output(resolve(name, |_: KeyCode| -> &'static str {
                panic!("unknown keys must not reach the native resolver");
            }));
            let binding = read_output(resolve(name, |_: Hotkey| -> &'static str {
                panic!("unknown hotkeys must not reach the native resolver");
            }));
            assert_eq!(key, name);
            assert_eq!(binding, name);
        }
    }
}
