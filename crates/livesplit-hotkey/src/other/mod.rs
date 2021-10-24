use crate::KeyCode;
use alloc::string::String;

#[derive(Debug, snafu::Snafu)]
pub enum Error {}

pub type Result<T> = core::result::Result<T, Error>;

pub struct Hook;

impl Hook {
    pub fn new() -> Result<Self> {
        Ok(Hook)
    }

    pub fn register<F>(&self, _: KeyCode, _: F) -> Result<()>
    where
        F: FnMut() + Send + 'static,
    {
        Ok(())
    }

    pub fn unregister(&self, _: KeyCode) -> Result<()> {
        Ok(())
    }
}

pub(crate) fn try_resolve(_key_code: KeyCode) -> Option<String> {
    None
}
