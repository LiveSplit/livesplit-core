use std::collections::HashMap;

/// A setting that is meant to be shown to and modified by the user.
#[non_exhaustive]
pub struct UserSetting {
    /// A unique identifier for this setting. This is not meant to be shown to
    /// the user and is only used to keep track of the setting. This key is used
    /// to store and retrieve the value of the setting from the
    /// [`SettingsStore`].
    pub key: Box<str>,
    /// The name of the setting that is shown to the user.
    pub description: Box<str>,
    /// An optional tooltip that is shown to the user when hovering over the
    /// setting.
    pub tooltip: Option<Box<str>>,
    /// The type of setting and additional information about it.
    pub kind: UserSettingKind,
}

/// The type of a [`UserSetting`] and additional information about it.
pub enum UserSettingKind {
    /// A title that is shown to the user. It doesn't by itself store a value
    /// and is instead used to group settings together.
    Title {
        /// The heading level of the title. This is used to determine the size
        /// of the title and which other settings are grouped together with it.
        /// The top level titles use a heading level of 0.
        heading_level: u32,
    },
    /// A boolean setting. This could be visualized as a checkbox or a slider.
    Bool {
        /// The default value of the setting, if it's not available in the
        /// settings store yet.
        default_value: bool,
    },
}

/// A value that a setting can have.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum SettingValue {
    /// A boolean value.
    Bool(bool),
}

/// Stores all the settings of an auto splitter. Currently this only stores
/// values that are modified. So there may be settings that are registered as
/// user settings, but because the user didn't modify them, they are not stored
/// here yet.
#[derive(Clone, Default)]
pub struct SettingsStore {
    values: HashMap<Box<str>, SettingValue>,
}

impl SettingsStore {
    /// Creates a new empty settings store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a setting to the new value. If the key of the setting doesn't exist
    /// yet it will be stored as a new value. Otherwise the value will be
    /// updated.
    pub fn set(&mut self, key: Box<str>, value: SettingValue) {
        self.values.insert(key, value);
    }

    /// Accesses the value of a setting by its key. While the setting may exist
    /// as part of the user settings, it may not have been stored into the
    /// settings store yet, so it may not exist, despite being registered.
    pub fn get(&self, key: &str) -> Option<&SettingValue> {
        self.values.get(key)
    }

    /// Iterates over all the setting keys and their values in the settings
    /// store.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &SettingValue)> {
        self.values.iter().map(|(k, v)| (k.as_ref(), v))
    }
}
