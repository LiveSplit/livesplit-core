quick_error! {
    #[derive(Debug)]
    pub enum Error {
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct KeyCode;

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

use std::{str::FromStr, result::Result as StdResult};

impl FromStr for KeyCode {
    type Err = ();
    fn from_str(_: &str) -> StdResult<Self, Self::Err> {
        Ok(KeyCode)
    }
}
