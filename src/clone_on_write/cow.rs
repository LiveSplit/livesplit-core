use std::sync::{Arc, Weak as ArcWeak};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Cow<T: ?Sized + Clone> {
    inner: Arc<T>,
}

impl<T: ?Sized + Clone> Cow<T> {
    pub fn new(inner: T) -> Self {
        Self { inner: Arc::new(inner) }
    }

    pub fn downgrade(this: &Cow<T>) -> Weak<T> {
        Weak { inner: Arc::downgrade(&this.inner) }
    }

    pub fn ptr_eq(this: &Cow<T>, other: &Cow<T>) -> bool {
        Arc::ptr_eq(&this.inner, &other.inner)
    }
}

impl<T: ?Sized + Clone> Deref for Cow<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.inner
    }
}

impl<T: ?Sized + Clone> DerefMut for Cow<T> {
    fn deref_mut(&mut self) -> &mut T {
        Arc::make_mut(&mut self.inner)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Weak<T: ?Sized + Clone> {
    inner: ArcWeak<T>,
}

impl<T: ?Sized + Clone> Weak<T> {
    pub fn new() -> Self {
        Self { inner: ArcWeak::new() }
    }

    pub fn upgrade(&self) -> Option<Cow<T>> {
        self.inner.upgrade().map(|a| Cow { inner: a })
    }
}

#[test]
fn test() {
    let mut x = Cow::new(5);
    *x += 3;
    let mut y = x.clone();
    assert_eq!(*x, 8);
    assert_eq!(*y, 8);
    *y = 10;
    assert_eq!(*x, 8);
    assert_eq!(*y, 10);
}
