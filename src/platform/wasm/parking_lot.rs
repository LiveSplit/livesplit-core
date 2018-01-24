use std::sync::RwLock as StdLock;
pub use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub struct RwLock<T>(StdLock<T>);

impl<T> RwLock<T> {
    pub fn new(obj: T) -> Self {
        RwLock(StdLock::new(obj))
    }

    pub fn write(&self) -> RwLockWriteGuard<T> {
        self.0.write().unwrap()
    }

    pub fn read(&self) -> RwLockReadGuard<T> {
        self.0.read().unwrap()
    }
}
