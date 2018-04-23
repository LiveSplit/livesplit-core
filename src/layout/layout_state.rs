use super::ComponentState;
use serde_json::{to_writer, Result};
use settings::{Color, Gradient};
use std::io::Write;

/// The state object describes the information to visualize for the layout.
#[derive(Serialize, Deserialize)]
pub struct LayoutState {
    /// The state objects for all of the components in the layout.
    pub components: Vec<ComponentState>,
    /// The background to show behind the layout.
    pub background: Gradient,
    /// The color of thin separators.
    pub thin_separators_color: Color,
    /// The color of normal separators.
    pub separators_color: Color,
    /// The text color to use for text that doesn't specify its own color.
    pub text_color: Color,
}

impl LayoutState {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, &self)
    }
}
