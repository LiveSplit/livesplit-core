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
    hotkey::{self, Error, Hotkey, KeyCode, Permission},
    hotkey_system,
};

type HotkeySystem = livesplit_core::HotkeySystem<CommandSink>;

/// type
pub type OwnedHotkeySystem = Box<HotkeySystem>;
/// type
pub type NullableOwnedHotkeySystem = Option<OwnedHotkeySystem>;

// Keep these codes independent of Rust enum discriminants. Positive values
// always mean a missing native grant, including requirements added in a future
// hotkey backend; negative values are failures of the permission operation.
fn permission_status(result: hotkey::Result<()>) -> i32 {
    match result {
        Ok(()) => 0,
        Err(Error::PermissionRequired(permission)) => match permission {
            Permission::InputMonitoring => 1,
            Permission::Accessibility => 2,
            _ => i32::MAX,
        },
        Err(_) => -1,
    }
}

/// Checks the Hotkey System's current native privacy access without prompting.
/// No Hotkey System, timer, hook or registrations are created.
///
/// Returns 0 if access is currently available or the platform has no separate
/// privacy-grant API, 1 if macOS Input Monitoring is required, 2 if macOS
/// Accessibility is required, and -1 if the native permission operation failed.
/// Other positive values identify unknown permission requirements; callers must
/// handle all positive values as missing access and all negative values as
/// failures. The fallback for an unknown requirement is 2147483647.
///
/// This uses the same hook preference as construction. A return value of 0
/// does not guarantee that construction or registration will succeed. After a
/// constructor returns <NULL>, this observes the current grant; it does not
/// recover the original failure or rule out a grant change between calls.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeySystem_check_permission() -> i32 {
    permission_status(hotkey_system::check_permission())
}

/// Requests the Hotkey System's native privacy access after an explicit user
/// action. No Hotkey System, timer, hook or registrations are created.
///
/// Returns the same status codes as HotkeySystem_check_permission: 0 for
/// currently available access, 1 for missing macOS Input Monitoring, 2 for
/// missing macOS Accessibility, and -1 for a native permission-operation failure.
/// Any other positive value is an unknown missing permission (2147483647 is the
/// fallback); callers must handle all positive values as missing access and all
/// negative values as failures.
///
/// Existing access is checked first, and at most one native request is made
/// when access is missing. A positive result can mean approval is still pending
/// even if the native permission UI was opened; it does not establish denial.
/// On a later explicit user action, check again or retry HotkeySystem_new or
/// HotkeySystem_with_config. These operations neither poll nor retry bindings
/// automatically, and permission access alone does not guarantee construction.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeySystem_request_permission() -> i32 {
    permission_status(hotkey_system::request_permission())
}

/// Creates a new Hotkey System for a Timer with the default hotkeys.
///
/// Returns <NULL> on failure without requesting privacy access. Call
/// HotkeySystem_check_permission to inspect the current native grant, then
/// HotkeySystem_request_permission only after an explicit user action. Retry
/// construction explicitly once access is available; other failures may remain.
#[unsafe(no_mangle)]
pub extern "C" fn HotkeySystem_new(command_sink: &CommandSink) -> NullableOwnedHotkeySystem {
    HotkeySystem::new(command_sink.clone()).ok().map(Box::new)
}

/// Creates a new Hotkey System for a Timer with a custom configuration for the
/// hotkeys.
///
/// Returns <NULL> on failure without requesting privacy access. Call
/// HotkeySystem_check_permission to inspect the current native grant, and request
/// missing access only after an explicit user action.
///
/// This consumes the configuration even when construction fails. To retry saved
/// bindings, retain their data in the caller or copy HotkeyConfig_as_json before
/// this call and recreate the configuration with HotkeyConfig_parse_json. Copy
/// the JSON into caller-owned storage before another API call can reuse its
/// temporary output buffer. Retry construction explicitly after granting access.
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
    use crate::hotkey_system::{permission_status, resolve};
    use livesplit_core::{
        Lang,
        hotkey::{Error, Hotkey, KeyCode, Permission, display},
    };
    use std::{ffi::CStr, os::raw::c_char};

    #[test]
    fn permission_status_codes_are_stable() {
        assert_eq!(permission_status(Ok(())), 0);
        assert_eq!(
            permission_status(Err(Error::PermissionRequired(Permission::InputMonitoring))),
            1
        );
        assert_eq!(
            permission_status(Err(Error::PermissionRequired(Permission::Accessibility))),
            2
        );
    }

    #[test]
    fn permission_operation_failures_are_distinct_from_missing_grants() {
        for error in [
            Error::UnmatchedPreference,
            Error::AlreadyRegistered,
            Error::NotRegistered,
        ] {
            assert_eq!(permission_status(Err(error)), -1);
        }
    }

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
