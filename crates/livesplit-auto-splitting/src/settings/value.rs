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

impl Value {
    /// Accesses the value as boolean, if it is one.
    pub const fn to_bool(&self) -> Option<bool> {
        if let Self::Bool(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    /// Accesses the value as 64-bit signed integer, if it is one.
    pub const fn to_i64(&self) -> Option<i64> {
        if let Self::I64(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    /// Accesses the value as 64-bit floating point, if it is one.
    pub const fn to_f64(&self) -> Option<f64> {
        if let Self::F64(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    /// Accesses the value as a map, if it is one.
    pub const fn as_map(&self) -> Option<&Map> {
        if let Self::Map(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Accesses the value as a list, if it is one.
    pub const fn as_list(&self) -> Option<&List> {
        if let Self::List(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Accesses the value as a string, if it is one.
    pub const fn as_string(&self) -> Option<&Arc<str>> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Converts the value into a map, if it is one.
    pub fn into_map(self) -> Option<Map> {
        if let Self::Map(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Converts the value into a list, if it is one.
    pub fn into_list(self) -> Option<List> {
        if let Self::List(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Converts the value into a string, if it is one.
    pub fn into_string(self) -> Option<Arc<str>> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
