use super::{ComponentState, LayoutDirection};
use crate::{
    platform::prelude::*,
    settings::{Color, Font, Gradient},
};
use serde_derive::{Deserialize, Serialize};

/// The state object describes the information to visualize for the layout.
#[derive(Default, Serialize, Deserialize)]
pub struct LayoutState {
    /// The state objects for all of the components in the layout.
    pub components: Vec<ComponentState>,
    /// The direction which the components are laid out in.
    pub direction: LayoutDirection,
    /// The font to use for the timer text. `None` means a default font should
    /// be used.
    pub timer_font: Option<Font>,
    /// The font to use for the times and other values. `None` means a default
    /// font should be used.
    pub times_font: Option<Font>,
    /// The font to use for regular text. `None` means a default font should be
    /// used.
    pub text_font: Option<Font>,
    /// The background to show behind the layout.
    pub background: Gradient,
    /// The color of thin separators.
    pub thin_separators_color: Color,
    /// The color of normal separators.
    pub separators_color: Color,
    /// The text color to use for text that doesn't specify its own color.
    pub text_color: Color,
}

#[cfg(feature = "std")]
impl LayoutState {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }
}
