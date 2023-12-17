use std::{fmt, sync::Arc};

use indexmap::IndexMap;

use super::Value;

/// A key-value map that stores the settings of an auto splitter. It only stores
/// values that are modified. So there may be settings that are registered as
/// user settings, but because the user didn't modify them, they are not stored
/// here yet.
#[derive(Clone, Default, PartialEq)]
#[repr(transparent)]
pub struct Map {
    pub(crate) values: Arc<IndexMap<Arc<str>, Value>>,
}

impl fmt::Debug for Map {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.values, f)
    }
}

impl Map {
    /// Creates a new empty settings map.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a setting to the new value. If the key of the setting doesn't exist
    /// yet it will be stored as a new value. Otherwise the value will be
    /// updated.
    #[inline]
    pub fn insert(&mut self, key: Arc<str>, value: Value) {
        Arc::make_mut(&mut self.values).insert(key, value);
    }

    /// Accesses the value of a setting by its key. While the setting may exist
    /// as part of the user settings, it may not have been stored into the
    /// settings map yet, so it may not exist, despite being registered.
    #[must_use]
    #[inline]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    /// Accesses the value of a setting by its index. The index is the position
    /// of the setting in the list of all settings. This may be useful for
    /// iterating over all settings. Prefer using [`iter`](Self::iter) in most
    /// situations though.
    #[must_use]
    #[inline]
    pub fn get_by_index(&self, index: usize) -> Option<(&str, &Value)> {
        self.values.get_index(index).map(|(k, v)| (k.as_ref(), v))
    }

    /// Iterates over all the setting keys and their values in the map.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.values.iter().map(|(k, v)| (k.as_ref(), v))
    }

    /// Returns the number of settings that are stored in the map.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns [`true`] if the map doesn't contain any settings.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut map = Map::new();
        map.insert("a".into(), Value::Bool(true));
        map.insert("b".into(), Value::Bool(false));
        map.insert("c".into(), Value::Bool(true));
        map.insert("b".into(), Value::Bool(true));
        assert_eq!(map.len(), 3);
        assert_eq!(map.get("a").unwrap(), &Value::Bool(true));
        assert_eq!(map.get("b").unwrap(), &Value::Bool(true));
        assert_eq!(map.get("c").unwrap(), &Value::Bool(true));
    }

    #[test]
    fn test_get() {
        let mut map = Map::new();
        map.insert("a".into(), Value::Bool(true));
        map.insert("b".into(), Value::Bool(false));
        map.insert("c".into(), Value::Bool(true));
        assert_eq!(map.get("a").unwrap(), &Value::Bool(true));
        assert_eq!(map.get("b").unwrap(), &Value::Bool(false));
        assert_eq!(map.get("c").unwrap(), &Value::Bool(true));
    }

    #[test]
    fn test_get_by_index() {
        let mut map = Map::new();
        map.insert("a".into(), Value::Bool(true));
        map.insert("b".into(), Value::Bool(false));
        map.insert("c".into(), Value::Bool(true));
        assert_eq!(map.get_by_index(0).unwrap(), ("a", &Value::Bool(true)));
        assert_eq!(map.get_by_index(1).unwrap(), ("b", &Value::Bool(false)));
        assert_eq!(map.get_by_index(2).unwrap(), ("c", &Value::Bool(true)));
    }

    #[test]
    fn test_iter() {
        let mut map = Map::new();
        map.insert("a".into(), Value::Bool(true));
        map.insert("b".into(), Value::Bool(false));
        map.insert("c".into(), Value::Bool(true));
        let mut iter = map.iter();
        assert_eq!(iter.next().unwrap(), ("a", &Value::Bool(true)));
        assert_eq!(iter.next().unwrap(), ("b", &Value::Bool(false)));
        assert_eq!(iter.next().unwrap(), ("c", &Value::Bool(true)));
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_len() {
        let mut map = Map::new();
        assert_eq!(map.len(), 0);
        map.insert("a".into(), Value::Bool(true));
        assert_eq!(map.len(), 1);
        map.insert("b".into(), Value::Bool(false));
        assert_eq!(map.len(), 2);
        map.insert("c".into(), Value::Bool(true));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_is_empty() {
        let mut map = Map::new();
        assert!(map.is_empty());
        map.insert("a".into(), Value::Bool(true));
        assert!(!map.is_empty());
        map.insert("b".into(), Value::Bool(false));
        assert!(!map.is_empty());
        map.insert("c".into(), Value::Bool(true));
        assert!(!map.is_empty());
    }
}
