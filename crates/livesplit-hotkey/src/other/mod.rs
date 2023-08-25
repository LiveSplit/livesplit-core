use crate::{Hotkey, KeyCode};
use alloc::{fmt, string::String};

/// The error type for this crate.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

/// The result type for this crate.
pub type Result<T> = core::result::Result<T, Error>;

/// A hook allows you to listen to hotkeys.
pub struct Hook;

impl Hook {
    /// Creates a new hook.
    pub fn new() -> Result<Self> {
        Ok(Hook)
    }

    /// Registers a hotkey to listen to.
    pub fn register<F>(&self, _: Hotkey, _: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        Ok(())
    }

    /// Unregisters a previously registered hotkey.
    pub fn unregister(&self, _: Hotkey) -> Result<()> {
        Ok(())
    }

    pub(crate) fn try_resolve(&self, _key_code: KeyCode) -> Option<String> {
        None
    }
}
