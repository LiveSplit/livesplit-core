use std::{fmt, sync::Arc};

use super::Value;

/// A list of settings that may be used as a [`Value`] as part of a
/// [`Map`](super::Map). It logically resembles a [`Vec`] of [`Value`] and
/// therefore provides similar functionality.
#[derive(Clone, Default, PartialEq)]
pub struct List {
    list: Arc<Vec<Value>>,
}

impl fmt::Debug for List {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.list, f)
    }
}

impl List {
    /// Creates a new empty settings list.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of settings that are stored in the list.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Returns [`true`] if the list doesn't contain any settings.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Accesses the value of a setting by its index. The index is the position
    /// of the setting in the list of all settings. This may be useful for
    /// iterating over all settings. Prefer using [`iter`](Self::iter) in most
    /// situations though.
    #[must_use]
    #[inline]
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.list.get(index)
    }

    /// Pushes a setting value to the end of the list.
    #[inline]
    pub fn push(&mut self, value: Value) {
        Arc::make_mut(&mut self.list).push(value);
    }

    /// Inserts a setting value at the given index. If the index is larger than
    /// the length of the list, [`Err`] will be returned.
    #[inline]
    pub fn insert(&mut self, index: usize, value: Value) -> Result<(), InsertError> {
        let list = Arc::make_mut(&mut self.list);
        if index > list.len() {
            return Err(InsertError {});
        }
        list.insert(index, value);
        Ok(())
    }

    /// Removes the setting value at the given index and returns it. If the
    /// index is larger than the length of the list, [`None`] will be returned.
    #[inline]
    pub fn remove(&mut self, index: usize) -> Option<Value> {
        let list = Arc::make_mut(&mut self.list);
        if index >= list.len() {
            return None;
        }
        Some(list.remove(index))
    }

    /// Iterates over all the setting values in the list.
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.list.iter()
    }

    /// Returns [`true`] if the identity of the list is the same as the identity
    /// of the other list. Lists use the copy-on-write principle. This means
    /// that cloning a list is cheap because it references all the same data as
    /// the original until one of the variables is changed. With this function
    /// you can check if two variables internally share the same data and are
    /// therefore identical. This is useful to determine if the list has changed
    /// since the last time it was checked. You may use this as part of a
    /// compare-and-swap loop.
    pub fn is_unchanged(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.list, &other.list)
    }
}

/// An error that may occur when inserting a setting value into a list.
#[non_exhaustive]
pub struct InsertError {}

impl fmt::Debug for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("insertion failed")
    }
}

impl std::error::Error for InsertError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut list = List::new();
        list.push(Value::Bool(true));
        list.push(Value::Bool(false));
        list.push(Value::Bool(true));
        list.insert(1, Value::Bool(true)).unwrap();
        assert_eq!(list.len(), 4);
        assert_eq!(list.get(0).unwrap(), &Value::Bool(true));
        assert_eq!(list.get(1).unwrap(), &Value::Bool(true));
        assert_eq!(list.get(2).unwrap(), &Value::Bool(false));
        assert_eq!(list.get(3).unwrap(), &Value::Bool(true));
    }

    #[test]
    fn test_get() {
        let mut list = List::new();
        list.push(Value::Bool(true));
        list.push(Value::Bool(false));
        list.push(Value::Bool(true));
        assert_eq!(list.get(0).unwrap(), &Value::Bool(true));
        assert_eq!(list.get(1).unwrap(), &Value::Bool(false));
        assert_eq!(list.get(2).unwrap(), &Value::Bool(true));
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(Value::Bool(true));
        list.push(Value::Bool(false));
        list.push(Value::Bool(true));
        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap(), &Value::Bool(true));
        assert_eq!(iter.next().unwrap(), &Value::Bool(false));
        assert_eq!(iter.next().unwrap(), &Value::Bool(true));
    }

    #[test]
    fn test_len() {
        let mut list = List::new();
        assert_eq!(list.len(), 0);
        list.push(Value::Bool(true));
        assert_eq!(list.len(), 1);
        list.push(Value::Bool(false));
        assert_eq!(list.len(), 2);
        list.push(Value::Bool(true));
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_is_empty() {
        let mut list = List::new();
        assert!(list.is_empty());
        list.push(Value::Bool(true));
        assert!(!list.is_empty());
        list.push(Value::Bool(false));
        assert!(!list.is_empty());
        list.push(Value::Bool(true));
        assert!(!list.is_empty());
    }

    #[test]
    fn test_is_unchanged() {
        let mut list = List::new();
        let mut list2 = list.clone();
        assert!(list.is_unchanged(&list2));
        list.push(Value::Bool(true));
        assert!(!list.is_unchanged(&list2));
        list2.push(Value::Bool(true));
        assert!(!list.is_unchanged(&list2));
        list.push(Value::Bool(false));
        assert!(!list.is_unchanged(&list2));
        list2.push(Value::Bool(false));
        assert!(!list.is_unchanged(&list2));
        list2 = list.clone();
        assert!(list.is_unchanged(&list2));
        assert!(list2.is_unchanged(&list));
    }
}
