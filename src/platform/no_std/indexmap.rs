//! no-std polyfill for indexmap.

/// IndexMap is a hash table where the iteration order of the key-value pairs is
/// independent of the hash values of the keys.
pub mod map {
    use crate::platform::prelude::*;
    use core::{fmt, marker::PhantomData};
    use serde::{
        de::{MapAccess, Visitor},
        ser::SerializeMap,
        Deserialize, Deserializer, Serialize, Serializer,
    };

    /// A hash table where the iteration order of the key-value pairs is
    /// independent of the hash values of the keys.
    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct IndexMap<K, V>(Vec<(K, V)>);

    /// An iterator over the entries of a IndexMap.
    pub struct Iter<'a, K, V>(core::slice::Iter<'a, (K, V)>);

    impl<K: PartialEq, V> IndexMap<K, V> {
        /// Insert a key-value pair in the map.
        pub fn insert(&mut self, key: K, value: V) {
            if let Some(index) = self.0.iter().position(|(k, _)| k == &key) {
                self.0[index] = (key, value);
            } else {
                self.0.push((key, value));
            }
        }

        /// Remove the key-value pair equivalent to key.
        pub fn shift_remove<K2>(&mut self, key: &K2)
        where
            K: core::borrow::Borrow<K2>,
            K2: ?Sized + PartialEq,
        {
            if let Some(index) = self.0.iter().position(|(k, _)| k.borrow() == key) {
                self.0.remove(index);
            }
        }

        /// Return an iterator over the key-value pairs of the map, in their order.
        pub fn iter(&self) -> Iter<'_, K, V> {
            Iter(self.0.iter())
        }

        /// Remove all key-value pairs in the map, while preserving its capacity.
        pub fn clear(&mut self) {
            self.0.clear();
        }
    }

    impl<'a, K, V> Iterator for Iter<'a, K, V> {
        type Item = (&'a K, &'a V);
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(|(a, b)| (a, b))
        }
    }

    impl<K, V> Serialize for IndexMap<K, V>
    where
        K: PartialEq + Serialize,
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

    impl<'de, K, V> Deserialize<'de> for IndexMap<K, V>
    where
        K: PartialEq + Deserialize<'de>,
        V: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_map(IndexMapVisitor::new())
        }
    }

    struct IndexMapVisitor<K, V> {
        marker: PhantomData<fn() -> IndexMap<K, V>>,
    }

    impl<K, V> IndexMapVisitor<K, V> {
        fn new() -> Self {
            IndexMapVisitor {
                marker: PhantomData,
            }
        }
    }

    impl<'de, K, V> Visitor<'de> for IndexMapVisitor<K, V>
    where
        K: PartialEq + Deserialize<'de>,
        V: Deserialize<'de>,
    {
        type Value = IndexMap<K, V>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut map = IndexMap(Vec::with_capacity(access.size_hint().unwrap_or(0)));

            while let Some((key, value)) = access.next_entry()? {
                map.insert(key, value);
            }

            Ok(map)
        }
    }
}
