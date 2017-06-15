use super::ComponentState;
use serde_json::{to_writer, Result};
use std::io::Write;

pub struct LayoutState {
    pub components: Vec<ComponentState>,
}

impl LayoutState {
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, &self.components)
    }
}
