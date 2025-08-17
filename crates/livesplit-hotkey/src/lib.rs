#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    missing_docs,
    rust_2018_idioms
)]
#![forbid(clippy::incompatible_msrv)]
#![cfg_attr(not(feature = "std"), no_std)]

//! `livesplit-hotkey` is a crate that allows listening to hotkeys even when the
//! application is not in focus. The crate currently supports Windows, macOS,
//! Linux and the web via wasm-bindgen. On unsupported platforms the crate still
//! compiles but uses a stubbed out implementation instead that never receives
//! any hotkeys.

extern crate alloc;

cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        mod other;
        use self::other as platform;
    } else if #[cfg(windows)] {
        mod windows;
        use self::windows as platform;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        use self::linux as platform;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        use self::macos as platform;
    } else if #[cfg(all(target_family = "wasm", target_os = "unknown", feature = "wasm-web"))] {
        mod wasm_web;
        use self::wasm_web as platform;
    } else {
        mod other;
        use self::other as platform;
    }
}

mod hotkey;
mod key_code;
mod modifiers;
use core::fmt;

pub use self::{hotkey::*, key_code::*, modifiers::*};

/// A hook allows you to listen to hotkeys.
#[repr(transparent)]
pub struct Hook(platform::Hook);

/// The preference of whether the hotkeys should be consumed or not. Consuming a
/// hotkey means that the hotkey won't be passed on to the application that is
/// currently in focus.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ConsumePreference {
    /// There is no preference, the crate chooses the most suitable implementation.
    NoPreference,
    /// Prefers the hotkeys to be consumed, but does not require it.
    PreferConsume,
    /// Prefers the hotkeys to not be consumed, but does not require it.
    PreferNoConsume,
    /// Requires the hotkeys to be consumed, the [`Hook`] won't be created otherwise.
    MustConsume,
    /// Requires the hotkeys to not be consumed, the [`Hook`] won't be created
    /// otherwise.
    MustNotConsume,
}

impl Hook {
    /// Creates a new hook without any preference of whether the hotkeys should
    /// be consumed or not.
    pub fn new() -> Result<Self> {
        Ok(Self(platform::Hook::new(ConsumePreference::NoPreference)?))
    }

    /// Creates a new hook with a specific preference of whether the hotkeys
    /// should be consumed or not.
    pub fn with_consume_preference(consume: ConsumePreference) -> Result<Self> {
        Ok(Self(platform::Hook::new(consume)?))
    }

    /// Registers a hotkey to listen to.
    pub fn register<F>(&self, hotkey: Hotkey, callback: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        self.0.register(hotkey, callback)
    }

    /// Registers a hotkey to listen to, but with specific handling for
    /// press and release events.
    /// 
    /// Requires the `press_and_release` feature to be enabled.
    #[cfg(feature = "press_and_release")]
    pub fn register_specific<F>(&self, hotkey: Hotkey, callback: F) -> Result<()>
    where
        F: FnMut(bool) + Send + 'static,
    {
        self.0.register_specific(hotkey, callback)
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
    /// A platform specific error occurred.
    Platform(platform::Error),
}

// FIXME: Impl core::error::Error once it's stable.
#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnmatchedPreference => {
                "The consume preference could not be met on the current platform."
            }
            Self::AlreadyRegistered => "The hotkey was already registered.",
            Self::NotRegistered => "The hotkey to unregister was not registered.",
            Self::Platform(e) => return fmt::Display::fmt(e, f),
        })
    }
}

#[cfg(not(all(target_family = "wasm", target_os = "unknown", feature = "wasm-web")))]
const _: () = {
    #[allow(unused)]
    const fn assert_thread_safe<T: Send + Sync>() {}
    assert_thread_safe::<Hook>();
};

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
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
