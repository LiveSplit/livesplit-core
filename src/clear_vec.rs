//! A ClearVec is a special kind of Vec that when being cleared doesn't drop its
//! elements. Instead the elements get cleared out. This allows reusing them
//! later on again with all their capacity retained. The elements are only
//! dropped once the ClearVec itself gets dropped.

use crate::platform::prelude::*;
use alloc::{
    borrow::{Cow, ToOwned},
    vec,
};
use core::{
    iter::FromIterator,
    ops::{Deref, DerefMut, Index, IndexMut},
    slice,
};
use serde::{Deserialize, Serialize};

/// Allows clearing an object while retaining its capacity. This usually brings
/// the object back into the same state that the Default trait would create, but
/// that's not a hard requirement.
pub trait Clear {
    /// Clears the object.
    fn clear(&mut self);
}

/// A ClearVec is a special kind of Vec that when being cleared doesn't drop its
/// elements. Instead the elements get cleared out. This allows reusing them
/// later on again with all their capacity retained. The elements are only
/// dropped once the ClearVec itself gets dropped.
#[derive(Clone, Debug)]
pub struct ClearVec<T: Clear> {
    vec: Vec<T>,
    len: usize,
}

impl<T: Clear> ClearVec<T> {
    /// Creates an empty ClearVec.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns the number of elements in the ClearVec, also referred to as its
    /// 'length'.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the ClearVec contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Pushes an element into the ClearVec by either reusing an unused element
    /// or constructing a new one via the Default trait if there is no unused
    /// element available.
    pub fn push(&mut self) -> &mut T
    where
        T: Default,
    {
        self.push_with(Default::default)
    }

    /// Pushes an element into the ClearVec by either reusing an unused element
    /// or constructing a new one with the closure provided if there is no
    /// unused element available.
    pub fn push_with(&mut self, new_element: impl FnOnce() -> T) -> &mut T {
        let index = self.len;
        if index >= self.vec.len() {
            self.vec.push(new_element());
        }
        let element = &mut self.vec[index];
        self.len += 1;
        element
    }

    /// Clears the ClearVec and sets its length back to 0. All elements that
    /// were in use get cleared and stay in the ClearVec as unused elements,
    /// ready to be reused again.
    pub fn clear(&mut self) {
        for element in self.iter_mut() {
            element.clear();
        }
        self.len = 0;
    }

    /// Turns the ClearVec into a normal Vec, dropping all the unused elements
    /// in the progress.
    pub fn into_vec(mut self) -> Vec<T> {
        let len = self.len;
        self.vec.truncate(len);
        self.vec
    }
}

impl<T: Clear> Default for ClearVec<T> {
    fn default() -> Self {
        Vec::default().into()
    }
}

impl<T: Clear> FromIterator<T> for ClearVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Vec::from_iter(iter).into()
    }
}

impl<'a, T: Clear> Index<usize> for ClearVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<'a, T: Clear> IndexMut<usize> for ClearVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}

impl<'a, T: Clear> IntoIterator for &'a ClearVec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: Clear> IntoIterator for &'a mut ClearVec<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: Clear> IntoIterator for ClearVec<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}

impl<T: Clear> Deref for ClearVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.vec[..self.len]
    }
}

impl<T: Clear> DerefMut for ClearVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let len = self.len;
        &mut self.vec[..len]
    }
}

impl<T: Clear> From<Vec<T>> for ClearVec<T> {
    fn from(vec: Vec<T>) -> Self {
        Self {
            len: vec.len(),
            vec,
        }
    }
}

impl<T: Clear + Serialize> Serialize for ClearVec<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&**self).serialize(serializer)
    }
}

impl<'de, T: Clear + Deserialize<'de>> Deserialize<'de> for ClearVec<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Vec::<T>::deserialize(deserializer)?.into())
    }
}

impl Clear for String {
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T> Clear for Vec<T> {
    fn clear(&mut self) {
        self.clear();
    }
}

impl<T: Clear> Clear for ClearVec<T> {
    fn clear(&mut self) {
        self.clear();
    }
}

impl<B: ?Sized + ToOwned> Clear for Cow<'_, B>
where
    B::Owned: Clear,
{
    fn clear(&mut self) {
        if let Cow::Owned(o) = self {
            o.clear();
        }
    }
}
