mod time;
pub use self::time::*;

pub struct RwLock<T>(core::cell::RefCell<T>);

impl<T> RwLock<T> {
    pub fn new(value: T) -> Self {
        Self(core::cell::RefCell::new(value))
    }

    pub fn write(&self) -> core::cell::RefMut<'_, T> {
        self.0.borrow_mut()
    }

    pub fn read(&self) -> core::cell::Ref<'_, T> {
        self.0.borrow()
    }
}
