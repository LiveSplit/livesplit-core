use super::{ComponentSettings, GeneralSettings};
use crate::platform::prelude::*;
use serde::{Deserialize, Serialize};

/// Describes a whole layout by its settings in a way that can easily be
/// serialized and deserialized.
#[derive(Clone, Serialize, Deserialize)]
pub struct LayoutSettings {
    /// The settings for all the components.
    pub components: Vec<ComponentSettings>,
    /// The general settings of the layout that apply to all components.
    pub general: GeneralSettings,
}

#[cfg(feature = "std")]
impl LayoutSettings {
    /// Decodes the layout's settings from JSON.
    pub fn from_json<R>(reader: R) -> serde_json::Result<LayoutSettings>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(reader)
    }

    /// Encodes the layout's settings as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }
}
