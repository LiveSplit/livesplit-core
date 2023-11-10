use std::{fmt, sync::Arc};

use indexmap::IndexMap;

/// A setting that is meant to be shown to and modified by the user.
#[non_exhaustive]
#[derive(Clone)]
pub struct UserSetting {
    /// A unique identifier for this setting. This is not meant to be shown to
    /// the user and is only used to keep track of the setting. This key is used
    /// to store and retrieve the value of the setting from the main
    /// [`SettingsMap`].
    pub key: Arc<str>,
    /// The name of the setting that is shown to the user.
    pub description: Arc<str>,
    /// An optional tooltip that is shown to the user when hovering over the
    /// setting.
    pub tooltip: Option<Arc<str>>,
    /// The type of setting and additional information about it.
    pub kind: UserSettingKind,
}

/// The type of a [`UserSetting`] and additional information about it.
#[derive(Clone)]
pub enum UserSettingKind {
    /// A title that is shown to the user. It doesn't by itself store a value
    /// and is instead used to group settings together.
    Title {
        /// The heading level of the title. This is used to determine the size
        /// of the title and which other settings are grouped together with it.
        /// The top level titles use a heading level of 0.
        heading_level: u32,
    },
    /// A boolean setting. This could be visualized as a checkbox or a toggle.
    Bool {
        /// The default value of the setting, if it's not available in the
        /// settings map yet.
        default_value: bool,
    },
}

/// A value that a setting can have.
#[non_exhaustive]
#[derive(Clone)]
pub enum SettingValue {
    /// A map of settings that are stored in a [`SettingsMap`].
    Map(SettingsMap),
    /// A list of settings that are stored in a [`SettingsList`].
    List(SettingsList),
    /// A boolean value.
    Bool(bool),
    /// A 64-bit signed integer value.
    I64(i64),
    /// A 64-bit floating point value.
    F64(f64),
    /// A string value.
    String(Arc<str>),
}

impl fmt::Debug for SettingValue {
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

/// A key-value map that stores the settings of an auto splitter. It only stores
/// values that are modified. So there may be settings that are registered as
/// user settings, but because the user didn't modify them, they are not stored
/// here yet.
#[derive(Clone, Default)]
pub struct SettingsMap {
    values: Arc<IndexMap<Arc<str>, SettingValue>>,
}

impl fmt::Debug for SettingsMap {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.values, f)
    }
}

impl SettingsMap {
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
    pub fn insert(&mut self, key: Arc<str>, value: SettingValue) {
        Arc::make_mut(&mut self.values).insert(key, value);
    }

    /// Accesses the value of a setting by its key. While the setting may exist
    /// as part of the user settings, it may not have been stored into the
    /// settings map yet, so it may not exist, despite being registered.
    #[must_use]
    #[inline]
    pub fn get(&self, key: &str) -> Option<&SettingValue> {
        self.values.get(key)
    }

    /// Accesses the value of a setting by its index. The index is the position
    /// of the setting in the list of all settings. This may be useful for
    /// iterating over all settings. Prefer using [`iter`](Self::iter) in most
    /// situations though.
    #[must_use]
    #[inline]
    pub fn get_by_index(&self, index: usize) -> Option<(&str, &SettingValue)> {
        self.values.get_index(index).map(|(k, v)| (k.as_ref(), v))
    }

    /// Iterates over all the setting keys and their values in the map.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (&str, &SettingValue)> {
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

    /// Returns [`true`] if the identity of the map is the same as the identity
    /// of the other map. Maps use the copy-on-write principle. This means that
    /// cloning a map is cheap because it references all the same data as the
    /// original until one of the variables is changed. With this function you
    /// can check if two variables internally share the same data and are
    /// therefore identical. This is useful to determine if the map has changed
    /// since the last time it was checked. You may use this as part of a
    /// compare-and-swap loop.
    #[must_use]
    #[inline]
    pub fn is_unchanged(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.values, &other.values)
    }
}

/// A list of settings that may be used as a [`SettingValue`] as part of a
/// [`SettingsMap`]. It logically resembles a [`Vec`] of [`SettingValue`] and
/// therefore provides similar functionality.
#[derive(Clone, Default)]
pub struct SettingsList {
    list: Arc<Vec<SettingValue>>,
}

impl fmt::Debug for SettingsList {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.list, f)
    }
}

impl SettingsList {
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
    pub fn get(&self, index: usize) -> Option<&SettingValue> {
        self.list.get(index)
    }

    /// Pushes a setting value to the end of the list.
    #[inline]
    pub fn push(&mut self, value: SettingValue) {
        Arc::make_mut(&mut self.list).push(value);
    }

    /// Inserts a setting value at the given index. If the index is larger than
    /// the length of the list, the value will be appended to the end of the
    /// list.
    #[inline]
    pub fn insert(&mut self, index: usize, value: SettingValue) {
        let list = Arc::make_mut(&mut self.list);
        list.insert(index.min(list.len()), value);
    }

    /// Removes the setting value at the given index and returns it. If the
    /// index is larger than the length of the list, [`None`] will be returned.
    #[inline]
    pub fn remove(&mut self, index: usize) -> Option<SettingValue> {
        let list = Arc::make_mut(&mut self.list);
        if index >= list.len() {
            return None;
        }
        Some(list.remove(index))
    }

    /// Iterates over all the setting values in the list.
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SettingValue> {
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
