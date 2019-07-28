use super::{ComponentSettings, GeneralSettings};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer, Result};
use std::io::{Read, Write};

/// Describes a whole layout by its settings in a way that can easily be
/// serialized and deserialized.
#[derive(Clone, Serialize, Deserialize)]
pub struct LayoutSettings {
    /// The settings for all the components.
    pub components: Vec<ComponentSettings>,
    /// The general settings of the layout that apply to all components.
    pub general: GeneralSettings,
}

impl LayoutSettings {
    /// Decodes the layout's settings from JSON.
    pub fn from_json<R>(reader: R) -> Result<LayoutSettings>
    where
        R: Read,
    {
        from_reader(reader)
    }

    /// Encodes the layout's settings as JSON.
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}
