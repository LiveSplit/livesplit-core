mod time;
pub use self::time::*;

#[allow(unused)]
pub struct RwLock<T>(core::cell::RefCell<T>);

#[allow(unused)]
impl<T> RwLock<T> {
    pub fn new(value: T) -> Self {
        Self(core::cell::RefCell::new(value))
    }

    pub fn write(&self) -> Result<core::cell::RefMut<'_, T>, ()> {
        Ok(self.0.borrow_mut())
    }

    pub fn read(&self) -> Result<core::cell::Ref<'_, T>, ()> {
        Ok(self.0.borrow())
    }
}
