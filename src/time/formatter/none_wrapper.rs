//! The none_wrapper modules provides wrapper types for Time Formatters that
//! allows you to create a new Time Formatter by wrapping another one and
//! changing its behavior when formatting empty times.

use super::{TimeFormatter, DASH};
use std::fmt::{Display, Formatter, Result};
use TimeSpan;

/// A Time Span to be formatted by a None Wrapper.
pub struct Inner<'a, F: 'a, S: 'a> {
    time: Option<TimeSpan>,
    wrapper: &'a NoneWrapper<F, S>,
}

/// A None Wrapper wraps a Time Formatter and changes its behavior when
/// formatting an empty time. The None Wrapper in particular replaces the empty
/// time with any string provided to the None Wrapper.
pub struct NoneWrapper<F, S>(F, S);

/// The Dash Wrapper is a helper type for creating a None Wrapper that always
/// uses a dash for the empty times.
pub struct DashWrapper;

/// The Empty Wrapper is a helper type for creating a None Wrapper that always
/// uses an empty string for the empty times.
pub struct EmptyWrapper;

impl<'a, F: 'a + TimeFormatter<'a>, S: AsRef<str>> NoneWrapper<F, S> {
    /// Creates a new None Wrapper that wraps around the Time Formatter provided
    /// and replaces its empty time formatting by the string provided to this
    /// Wrapper.
    pub fn new(inner: F, none_text: S) -> Self {
        NoneWrapper(inner, none_text.into())
    }
}

impl DashWrapper {
    /// Creates a new Dash Wrapper.
    #[allow(new_ret_no_self)]
    pub fn new<'a, F: 'a + TimeFormatter<'a>>(inner: F) -> NoneWrapper<F, &'static str> {
        NoneWrapper::new(inner, DASH)
    }
}

impl EmptyWrapper {
    /// Creates a new Empty Wrapper.
    #[allow(new_ret_no_self)]
    pub fn new<'a, F: 'a + TimeFormatter<'a>>(inner: F) -> NoneWrapper<F, &'static str> {
        NoneWrapper::new(inner, "")
    }
}

impl<'a, F: 'a + TimeFormatter<'a>, S: 'a + AsRef<str>> TimeFormatter<'a> for NoneWrapper<F, S> {
    type Inner = Inner<'a, F, S>;

    fn format<T>(&'a self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        Inner {
            time: time.into(),
            wrapper: self,
        }
    }
}

impl<'a, F: TimeFormatter<'a>, S: 'a + AsRef<str>> Display for Inner<'a, F, S> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.time.is_none() {
            write!(f, "{}", self.wrapper.1.as_ref())
        } else {
            write!(f, "{}", self.wrapper.0.format(self.time))
        }
    }
}
