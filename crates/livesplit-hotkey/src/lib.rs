#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    missing_docs
)]
#![forbid(clippy::incompatible_msrv)]
#![cfg_attr(not(feature = "std"), no_std)]

//! Listen to global hotkeys on Windows and macOS, including while the application
//! is not focused. On Linux, global capture requires an X11 session or access to
//! input devices. With `std` and `wasm-web`, web keyboard hooks listen within
//! browser windows instead of across other applications. Without `std`, native
//! hooks are stubs that receive no hotkeys; unsupported platforms also use stubs.
//!
//! [`display`] presents localized key and modifier names using native keyboard
//! conventions. Callers provide translations through [`display::Localize`], so
//! display names never affect the stable identifiers used for saved hotkeys.

extern crate alloc;

cfg_select! {
    not(feature = "std") => {
        mod other;
        use self::other as platform;
    }
    windows => {
        mod windows;
        use self::windows as platform;
    }
    target_os = "linux" => {
        mod linux;
        use self::linux as platform;
    }
    target_os = "macos" => {
        mod macos;
        use self::macos as platform;
    }
    all(target_family = "wasm", target_os = "unknown", feature = "wasm-web") => {
        mod wasm_web;
        use self::wasm_web as platform;
    }
    _ => {
        mod other;
        use self::other as platform;
    }
}

pub mod display;
mod hotkey;
mod key_code;
mod modifiers;
mod permission;
use core::fmt;

pub use self::{hotkey::*, key_code::*, modifiers::*, permission::Permission};

/// A hook allows you to listen to hotkeys.
#[repr(transparent)]
pub struct Hook(platform::Hook);

/// The preference of whether the hotkeys should be consumed or not. Consuming a
/// hotkey means that the hotkey won't be passed on to the application that is
/// currently in focus.
///
/// Requirements select a supported consumption mode when creating the hook.
/// Delivery and consumption can still be interrupted by runtime failures or
/// overload.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConsumePreference {
    /// There is no preference, the crate chooses the most suitable implementation.
    NoPreference,
    /// Prefers the hotkeys to be consumed, but does not require it.
    PreferConsume,
    /// Prefers the hotkeys to not be consumed, but does not require it.
    PreferNoConsume,
    /// Requires a consuming implementation; [`Hook`] creation fails otherwise.
    MustConsume,
    /// Requires a non-consuming implementation; [`Hook`] creation fails otherwise.
    MustNotConsume,
}

impl Hook {
    /// Creates a new hook without any preference of whether the hotkeys should
    /// be consumed or not.
    ///
    /// This never requests privacy permissions. Missing access returns
    /// [`Error::PermissionRequired`]; callers may use [`Self::request_permission`]
    /// after an explicit user action.
    pub fn new() -> Result<Self> {
        Self::with_consume_preference(ConsumePreference::NoPreference)
    }

    /// Creates a new hook with a specific preference of whether the hotkeys
    /// should be consumed or not.
    ///
    /// This never requests privacy permissions. Missing access returns
    /// [`Error::PermissionRequired`]; callers may use [`Self::request_permission`]
    /// after an explicit user action.
    pub fn with_consume_preference(consume: ConsumePreference) -> Result<Self> {
        Self::check_permission(consume)?;
        Ok(Self(platform::Hook::new(consume)?))
    }

    /// Checks the native privacy grant for the selected hook without prompting.
    ///
    /// On macOS, checks the keyboard privacy permission required by the selected
    /// hook. On Windows, Linux and the web, this performs no separate privacy
    /// check and returns success. Hook creation and registration may still fail for
    /// other reasons, and a successful check does not guarantee future delivery.
    pub fn check_permission(consume: ConsumePreference) -> Result<()> {
        permission::check(consume)
    }

    /// Requests the selected hook's native privacy grant after a user action.
    ///
    /// Already granted access does not prompt. A native request may return before
    /// approval takes effect. [`Error::PermissionRequired`] means access is still
    /// missing at that moment; it does not distinguish a denial from pending
    /// approval, even when the native permission UI has been opened.
    ///
    /// Check again or retry construction after the user has granted access.
    /// Platforms without a separate privacy grant return success without
    /// prompting, as described by [`Self::check_permission`].
    ///
    /// ```no_run
    /// # fn enable_from_user_action() -> livesplit_hotkey::Result<Option<livesplit_hotkey::Hook>> {
    /// use livesplit_hotkey::{ConsumePreference, Error, Hook};
    /// let preference = ConsumePreference::PreferNoConsume;
    /// match Hook::request_permission(preference) {
    ///     Ok(()) => Hook::with_consume_preference(preference).map(Some),
    ///     Err(Error::PermissionRequired(permission)) => {
    ///         eprintln!("{permission} access is not available yet.");
    ///         // Explain how to grant access, then retry construction on a later
    ///         // user action. Returning None does not schedule a background retry.
    ///         Ok(None)
    ///     }
    ///     Err(error) => Err(error),
    /// }
    /// # }
    /// ```
    pub fn request_permission(consume: ConsumePreference) -> Result<()> {
        permission::request(consume)
    }

    /// Registers a callback for a hotkey observed by the backend.
    ///
    /// On Windows, macOS and Linux, callbacks run on a background thread; on the
    /// web, they run on the browser's event loop. Keep callbacks short and send
    /// UI work to the application's event loop when required by its UI framework.
    /// Defer registration changes and keyboard-layout queries on the same hook
    /// until after the callback returns.
    pub fn register<F>(&self, hotkey: Hotkey, callback: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        self.0.register(hotkey, callback)
    }

    /// Unregisters a previously registered hotkey.
    pub fn unregister(&self, hotkey: Hotkey) -> Result<()> {
        self.0.unregister(hotkey)
    }

    /// On the web you can use this to listen to keyboard events on an
    /// additional child window as well.
    #[cfg(all(target_family = "wasm", feature = "wasm-web"))]
    pub fn add_window(&self, window: web_sys::Window) -> Result<()> {
        self.0.add_window(window)
    }
}

/// The result type for this crate.
pub type Result<T> = core::result::Result<T, Error>;

/// The error type for this crate.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// The consume preference could not be met on the current platform.
    UnmatchedPreference,
    /// The hotkey was already registered.
    AlreadyRegistered,
    /// The hotkey to unregister was not registered.
    NotRegistered,
    /// A native privacy grant is missing; explicitly request it after a user action.
    PermissionRequired(Permission),
    /// A platform specific error occurred.
    Platform(platform::Error),
}

// FIXME: Impl core::error::Error once it's stable.
#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::UnmatchedPreference => {
                "The consume preference could not be met on the current platform."
            }
            Self::AlreadyRegistered => "The hotkey was already registered.",
            Self::NotRegistered => "The hotkey to unregister was not registered.",
            Self::PermissionRequired(permission) => {
                return write!(f, "Global hotkeys require {permission} permission.");
            }
            Self::Platform(e) => return fmt::Display::fmt(e, f),
        })
    }
}

#[cfg(not(all(target_family = "wasm", target_os = "unknown", feature = "wasm-web")))]
const _: () = {
    const fn assert_thread_safe<T: Send + Sync>() {}
    assert_thread_safe::<Hook>();
};

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    #[ignore = "registers native hotkeys and waits for manual keyboard input"]
    fn test() {
        let hook = Hook::new().unwrap();

        hook.register(KeyCode::Numpad1.with_modifiers(Modifiers::SHIFT), || {
            println!("A")
        })
        .unwrap();
        println!("Press Shift + Numpad1");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::Numpad1.with_modifiers(Modifiers::SHIFT))
            .unwrap();

        hook.register(KeyCode::KeyN.into(), || println!("B"))
            .unwrap();
        println!("Press KeyN");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::KeyN.into()).unwrap();

        hook.register(KeyCode::Numpad1.into(), || println!("C"))
            .unwrap();
        println!("Press Numpad1");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::Numpad1.into()).unwrap();
    }

    #[test]
    #[ignore = "creates a native hook and reads the host keyboard layout"]
    fn resolve() {
        let hook = Hook::new().unwrap();

        // Based on German keyboard layout.
        println!("ß: {}", KeyCode::Minus.resolve(&hook));
        println!("ü: {}", KeyCode::BracketLeft.resolve(&hook));
        println!("#: {}", KeyCode::Backslash.resolve(&hook));
        println!("+: {}", KeyCode::BracketRight.resolve(&hook));
        println!("z: {}", KeyCode::KeyY.resolve(&hook));
        println!("^: {}", KeyCode::Backquote.resolve(&hook));
        println!("<: {}", KeyCode::IntlBackslash.resolve(&hook));
        println!("Yen: {}", KeyCode::IntlYen.resolve(&hook));
        println!("Enter: {}", KeyCode::Enter.resolve(&hook));
        println!("Space: {}", KeyCode::Space.resolve(&hook));
        println!("Tab: {}", KeyCode::Tab.resolve(&hook));
        println!("Numpad0: {}", KeyCode::Numpad0.resolve(&hook));
    }
}
