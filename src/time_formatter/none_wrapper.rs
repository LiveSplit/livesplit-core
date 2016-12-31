use std::fmt::{Result, Formatter, Display};
use TimeSpan;
use super::TimeFormatter;

pub struct Inner<'a, F: 'a, S: 'a> {
    time: Option<TimeSpan>,
    wrapper: &'a NoneWrapper<F, S>,
}
pub struct NoneWrapper<F, S>(F, S);
pub struct DashWrapper;
pub struct EmptyWrapper;

impl<'a, F: 'a + TimeFormatter<'a>, S: AsRef<str>> NoneWrapper<F, S> {
    pub fn new(inner: F, none_text: S) -> Self {
        NoneWrapper(inner, none_text.into())
    }
}

impl DashWrapper {
    pub fn new<'a, F: 'a + TimeFormatter<'a>>(inner: F) -> NoneWrapper<F, &'static str> {
        NoneWrapper::new(inner, "-")
    }
}

impl EmptyWrapper {
    pub fn new<'a, F: 'a + TimeFormatter<'a>>(inner: F) -> NoneWrapper<F, &'static str> {
        NoneWrapper::new(inner, "")
    }
}

impl<'a, F: 'a + TimeFormatter<'a>, S: 'a + AsRef<str>> TimeFormatter<'a> for NoneWrapper<F, S> {
    type Inner = Inner<'a, F, S>;

    fn format<T>(&'a self, time: T) -> Self::Inner
        where T: Into<Option<TimeSpan>>
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
