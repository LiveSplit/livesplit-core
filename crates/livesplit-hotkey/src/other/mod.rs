use crate::{ConsumePreference, Hotkey, KeyCode, Result};
use alloc::{fmt, string::String};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

pub struct Hook;

impl Hook {
    #[inline]
    pub fn new(_: ConsumePreference) -> Result<Self> {
        Ok(Hook)
    }

    #[inline]
    pub fn register<F>(&self, _: Hotkey, _: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        Ok(())
    }

    #[inline]
    #[cfg(feature = "press_and_release")]
    pub fn register_specific<F>(&self, _: Hotkey, _: F) -> Result<()>
    where
        F: FnMut(bool) + Send + 'static,
    {
        Ok(())
    }

    #[inline]
    pub fn unregister(&self, _: Hotkey) -> Result<()> {
        Ok(())
    }

    #[inline]
    pub fn try_resolve(&self, _key_code: KeyCode) -> Option<String> {
        None
    }
}
