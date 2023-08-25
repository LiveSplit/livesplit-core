//! Provides the state for key value based components. Examples of these
//! components include the Previous Segment and the Possible Time Save
//! components. They all share the same visual appearance and thus use the same
//! state object representation.

use crate::{
    platform::prelude::*,
    settings::{Color, Gradient, SemanticColor},
};
use alloc::borrow::Cow;
use serde_derive::{Deserialize, Serialize};

/// The state object describes the information to visualize for a key value
/// based component.
#[derive(Default, Serialize, Deserialize)]
pub struct State {
    /// The background shown behind the component.
    pub background: Gradient,
    /// The color of the key. If `None` is specified, the color is taken from
    /// the layout.
    pub key_color: Option<Color>,
    /// The color of the value. If `None` is specified, the color is taken from
    /// the layout.
    pub value_color: Option<Color>,
    /// The semantic coloring information the value carries.
    pub semantic_color: SemanticColor,
    /// The key to visualize.
    pub key: String,
    /// The value to visualize.
    pub value: String,
    /// Specifies additional abbreviations for the key that can be used instead
    /// of the key, if there is not enough space to show the whole key.
    pub key_abbreviations: Vec<Cow<'static, str>>,
    /// Specifies whether to display the key and the value in two separate rows.
    pub display_two_rows: bool,
    /// This value indicates whether the value is currently frequently being
    /// updated. This can be used for rendering optimizations.
    pub updates_frequently: bool,
}

#[cfg(feature = "std")]
impl State {
    /// Encodes the state object's information as JSON.
    pub fn write_json<W>(&self, writer: W) -> serde_json::Result<()>
    where
        W: std::io::Write,
    {
        serde_json::to_writer(writer, self)
    }
}

pub(super) const DEFAULT_GRADIENT: Gradient = Gradient::Vertical(
    Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 0.06,
    },
    Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 0.005,
    },
);
