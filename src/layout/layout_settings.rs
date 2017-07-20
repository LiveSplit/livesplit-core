use super::{ComponentSettings, GeneralSettings};
use serde_json::{to_writer, from_reader, Result};
use std::io::{Read, Write};

#[derive(Clone, Serialize, Deserialize)]
pub struct LayoutSettings {
    pub components: Vec<ComponentSettings>,
    pub general: GeneralSettings,
}

impl LayoutSettings {
    pub fn from_json<R>(reader: R) -> Result<LayoutSettings>
    where
        R: Read,
    {
        from_reader(reader)
    }

    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}
