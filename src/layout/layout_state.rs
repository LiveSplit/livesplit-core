use super::ComponentState;
use settings::Color;
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct LayoutState {
    pub components: Vec<ComponentState>,
    pub background_color: Color,
    pub thin_separators_color: Color,
    pub separators_color: Color,
    pub text_color: Color,
}

impl LayoutState {
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, &self)
    }
}
