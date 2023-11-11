use std::{fmt, sync::Arc};

use super::{List, Map};

/// A value that a setting can have.
#[non_exhaustive]
#[derive(Clone, PartialEq)]
pub enum Value {
    /// A key-value map of values.
    Map(Map),
    /// A list of values.
    List(List),
    /// A boolean value.
    Bool(bool),
    /// A 64-bit signed integer value.
    I64(i64),
    /// A 64-bit floating point value.
    F64(f64),
    /// A string value.
    String(Arc<str>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Map(v) => fmt::Debug::fmt(v, f),
            Self::List(v) => fmt::Debug::fmt(v, f),
            Self::Bool(v) => fmt::Debug::fmt(v, f),
            Self::I64(v) => fmt::Debug::fmt(v, f),
            Self::F64(v) => fmt::Debug::fmt(v, f),
            Self::String(v) => fmt::Debug::fmt(v, f),
        }
    }
}
