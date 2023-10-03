use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hasher, Hash}};

use xmltree::{Element, XMLNode};

/// A setting that is meant to be shown to and modified by the user.
#[non_exhaustive]
#[derive(Clone)]
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
    objects: HashMap<u64, ObjectValue>,
    auto_splitter_settings: u64, // u64 to be looked up in SettingsStore objects
}

impl SettingsStore {
    /// Creates a new empty settings store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new settings store from the AutoSplitterSettings XML contents.
    pub fn new_auto_splitter_settings(xml: String) -> Self {
        let mut objects = HashMap::new();
        let auto_splitter_settings = ObjectValue::from_xml_string(xml, &mut objects).insert(&mut objects);
        Self {
            values: Default::default(),
            objects,
            auto_splitter_settings,
        }
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

    /// Gets the AutoSplitterSettings object.
    pub fn get_auto_splitter_settings(&self) -> u64 {
        self.auto_splitter_settings
    }

    /// Query a settings object as a string.
    pub fn object_as_str(&self, o: u64) -> Option<&str> {
        match self.objects.get(&o)? {
            ObjectValue::Text(s) => Some(s),
            ObjectValue::List(l) => {
                match l[..] {
                    [] => Some(""),
                    [e] => self.object_as_str(e),
                    _ => None,
                }
            },
            _ => None,
        }
    }
    /// Query a settings objects as a bool.
    pub fn object_as_bool(&self, o: u64) -> Option<bool> {
        match self.object_as_str(o)?.trim() {
            "True" => Some(true),
            "False" => Some(false),
            _ => None,
        }
    }
    /// Query a settings object as a list.
    pub fn object_as_list(&self, o: u64) -> Option<&[u64]> {
        match self.objects.get(&o)? {
            ObjectValue::List(l) => Some(l),
            _ => None,
        }
    }
    /// Query a settings object as a list, get the length.
    pub fn object_list_len(&self, o: u64) -> Option<usize> {
        Some(self.object_as_list(o)?.len())
    }
    /// Query a settings object as a list, get an element by index.
    pub fn object_list_get(&self, o: u64, index: usize) -> Option<u64> {
        self.object_as_list(o)?.get(index).copied()
    }
    /// Query a settings object as a dictionary, get a value by key.
    pub fn object_dict_get(&self, o: u64, key: &str) -> Option<u64> {
        if let Some(v) = self.object_entry_get(o, key) {
            return Some(v);
        }
        for e in self.object_as_list(o)? {
            if let Some(v) = self.object_entry_get(*e, key) {
                return Some(v);
            }
        }
        None
    }
    fn object_entry_get(&self, o: u64, key: &str) -> Option<u64> {
        match self.objects.get(&o) {
            Some(ObjectValue::Entry(k, v)) if k == key => {
                Some(*v)
            },
            _ => None
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum ObjectValue {
    Text(String),
    Entry(String, u64), // u64 to be looked up in SettingsStore objects
    List(Vec<u64>), // u64 to be looked up in SettingsStore objects
}

impl ObjectValue {
    fn insert(self, objects: &mut HashMap<u64, ObjectValue>) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        objects.len().hash(&mut hasher);
        let h = hasher.finish();
        objects.insert(h, self);
        h
    }
    fn from_xml_string(xml: String, objects: &mut HashMap<u64, ObjectValue>) -> ObjectValue {
        ObjectValue::from_xml_nodes(Element::parse_all(xml.as_bytes()).unwrap_or_default(), objects)
    }
    fn from_xml_nodes(xml: Vec<XMLNode>, objects: &mut HashMap<u64, ObjectValue>) -> ObjectValue {
        let hs = xml.into_iter().filter_map(|e| {
            ObjectValue::from_xml_node(e, objects).map(|o| o.insert(objects))
        }).collect();
        ObjectValue::List(hs)
    }
    fn from_xml_node(xml: XMLNode, objects: &mut HashMap<u64, ObjectValue>) -> Option<ObjectValue> {
        match xml {
            XMLNode::Text(s) => Some(ObjectValue::Text(s)),
            XMLNode::Element(Element { name, children, .. }) => {
                let h = ObjectValue::from_xml_nodes(children, objects).insert(objects);
                Some(ObjectValue::Entry(name, h))
            },
            _ => None
        }
    }
}
