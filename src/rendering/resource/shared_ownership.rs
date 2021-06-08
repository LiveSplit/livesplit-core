use std::{rc::Rc, sync::Arc};

/// Describes that ownership of a value can be cheaply shared. This is similar
/// to the [`Clone`] trait, but is expected to only be implemented if sharing is
/// cheap, such as for [`Rc`] and [`Arc`].
pub trait SharedOwnership {
    /// Share the value.
    fn share(&self) -> Self;
}

impl<T: ?Sized> SharedOwnership for Rc<T> {
    fn share(&self) -> Self {
        self.clone()
    }
}

impl<T: ?Sized> SharedOwnership for Arc<T> {
    fn share(&self) -> Self {
        self.clone()
    }
}

impl<T: SharedOwnership> SharedOwnership for Option<T> {
    fn share(&self) -> Self {
        self.as_ref().map(SharedOwnership::share)
    }
}

impl SharedOwnership for () {
    fn share(&self) -> Self {}
}
