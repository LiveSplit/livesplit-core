use crate::platform::prelude::*;

/// A type that can be used to populate an owned [`String`]. This is useful for
/// reducing unnecessary memory allocations.
pub trait PopulateString: Sized {
    /// Populate the [`String`] with the contents of this type.
    fn populate(self, buf: &mut String);

    /// Accesses the contents of this type as a [`str`].
    fn as_str(&self) -> &str;

    /// Turns this type into a [`String`].
    fn into_string(self) -> String {
        let mut buf = String::new();
        self.populate(&mut buf);
        buf
    }
}

impl PopulateString for String {
    fn populate(self, buf: &mut String) {
        *buf = self;
    }
    fn as_str(&self) -> &str {
        self
    }
}

impl PopulateString for &str {
    // If the string doesn't fit into the capacity of the buffer, we just
    // allocate a new buffer instead of forcing it to reallocate, which would
    // mean copying all the bytes of the previous buffer, which we don't care
    // about.
    #[expect(clippy::assigning_clones)]
    fn populate(self, buf: &mut String) {
        if self.len() <= buf.capacity() {
            buf.clear();
            buf.push_str(self);
        } else {
            *buf = self.to_owned();
        }
    }
    fn as_str(&self) -> &str {
        self
    }
}

impl PopulateString for alloc::borrow::Cow<'_, str> {
    fn populate(self, buf: &mut String) {
        match self {
            alloc::borrow::Cow::Borrowed(s) => s.populate(buf),
            alloc::borrow::Cow::Owned(s) => s.populate(buf),
        }
    }
    fn as_str(&self) -> &str {
        self
    }
}
