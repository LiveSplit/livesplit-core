use super::Color;
use serde_derive::{Deserialize, Serialize};

/// Describes a Gradient for coloring a region with more than just a single
/// color.
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum Gradient {
    /// Don't use any color, keep it transparent.
    #[default]
    Transparent,
    /// Use a single color instead of a full gradient.
    Plain(Color),
    /// Use a vertical gradient (Top, Bottom).
    Vertical(Color, Color),
    /// Use a horizontal gradient (Left, Right).
    Horizontal(Color, Color),
}

/// Describes an extended form of a gradient, specifically made for use with
/// lists. It allows specifying different coloration for the rows in a list.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum ListGradient {
    /// Use the same gradient for every row in the list.
    Same(Gradient),
    /// Alternate between two colors for each row (Even Index, Odd Index).
    Alternating(Color, Color),
}

impl Default for ListGradient {
    fn default() -> Self {
        ListGradient::Same(Gradient::Transparent)
    }
}
