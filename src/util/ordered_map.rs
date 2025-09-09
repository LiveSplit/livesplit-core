//! An ordered [`Map`] is a map where the iteration order of the key-value pairs is
//! based on the order the pairs were inserted into the map.

use crate::{platform::prelude::*, util::PopulateString};
use core::{fmt, marker::PhantomData};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{MapAccess, Visitor},
    ser::SerializeMap,
};

/// An ordered [`Map`] is a map where the iteration order of the key-value pairs is
/// based on the order the pairs were inserted into the map.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Map<V>(Vec<(Box<str>, V)>);

/// An iterator over the entries of the [`Map`].
pub struct Iter<'a, V>(core::slice::Iter<'a, (Box<str>, V)>);

impl<V> Map<V> {
    /// Insert a key-value pair in the [`Map`].
    pub fn insert<K>(&mut self, key: K, value: V)
    where
        K: PopulateString,
    {
        let as_str = key.as_str();
        if let Some(index) = self.0.iter().position(|(k, _)| &**k == as_str) {
            self.0[index].1 = value;
        } else {
            self.0.push((key.into_string().into(), value));
        }
    }

    /// Return a reference to the value stored for key, if it is present, else [`None`].
    pub fn get(&self, key: &str) -> Option<&V> {
        if let Some(index) = self.0.iter().position(|(k, _)| &**k == key) {
            Some(&self.0[index].1)
        } else {
            None
        }
    }

    /// Get the given key’s corresponding entry in the [`Map`] for insertion
    /// and/or in-place manipulation.
    pub fn entry<K>(&mut self, key: K) -> Entry<'_, K, V>
    where
        K: PopulateString,
    {
        let as_str = key.as_str();
        Entry {
            index: self.0.iter().position(|(k, _)| &**k == as_str),
            map: self,
            key,
        }
    }

    /// Remove the key-value pair equivalent to key.
    pub fn shift_remove(&mut self, key: &str) {
        if let Some(index) = self.0.iter().position(|(k, _)| &**k == key) {
            self.0.remove(index);
        }
    }

    /// Return an iterator over the key-value pairs of the [`Map`], in their order.
    pub fn iter(&self) -> Iter<'_, V> {
        Iter(self.0.iter())
    }

    /// Remove all key-value pairs in the [`Map`], while preserving its capacity.
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

/// Entry for an existing key-value pair or a vacant location to insert one.
pub struct Entry<'a, K, V> {
    map: &'a mut Map<V>,
    index: Option<usize>,
    key: K,
}

impl<'a, K, V> Entry<'a, K, V> {
    /// Inserts a default-constructed value in the entry if it is vacant and
    /// returns a mutable reference to it. Otherwise a mutable reference to
    /// an already existent value is returned.
    pub fn or_default(self) -> &'a mut V
    where
        K: PopulateString,
        V: Default,
    {
        if let Some(index) = self.index {
            &mut self.map.0[index].1
        } else {
            self.map
                .0
                .push((self.key.into_string().into(), Default::default()));
            &mut self.map.0.last_mut().unwrap().1
        }
    }
}

impl<'a, V> Iterator for Iter<'a, V> {
    type Item = (&'a str, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(a, b)| (&**a, b))
    }
}

impl<V> Serialize for Map<V>
where
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de, V> Deserialize<'de> for Map<V>
where
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(IndexMapVisitor::new())
    }
}

struct IndexMapVisitor<V> {
    marker: PhantomData<fn() -> Map<V>>,
}

impl<V> IndexMapVisitor<V> {
    fn new() -> Self {
        IndexMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, V> Visitor<'de> for IndexMapVisitor<V>
where
    V: Deserialize<'de>,
{
    type Value = Map<V>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = Map(Vec::with_capacity(access.size_hint().unwrap_or(0)));

        while let Some((key, value)) = access.next_entry::<String, _>()? {
            map.insert(key, value);
        }

        Ok(map)
    }
}
