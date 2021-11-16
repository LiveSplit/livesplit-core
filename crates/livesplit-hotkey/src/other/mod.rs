use crate::KeyCode;
use alloc::string::String;

/// The error type for this crate.
#[derive(Debug, snafu::Snafu)]
#[non_exhaustive]
pub enum Error {}

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
    pub fn register<F>(&self, _: KeyCode, _: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        Ok(())
    }

    /// Unregisters a previously registered hotkey.
    pub fn unregister(&self, _: KeyCode) -> Result<()> {
        Ok(())
    }
}

pub(crate) fn try_resolve(_key_code: KeyCode) -> Option<String> {
    None
}
