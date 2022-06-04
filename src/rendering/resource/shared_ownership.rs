use alloc::rc::Rc;

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

#[cfg(target_has_atomic = "ptr")]
impl<T: ?Sized> SharedOwnership for alloc::sync::Arc<T> {
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
