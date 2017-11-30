use super::Color;

/// Describes a Gradient for coloring a region with more than just a single
/// color.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Gradient {
    /// Don't use any color, keep it transparent.
    Transparent,
    /// Use a single color instead of a full gradient.
    Plain(Color),
    /// Use a vertical gradient (Top, Bottom).
    Vertical(Color, Color),
    /// Use a horizontal gradient (Left, Right).
    Horizontal(Color, Color),
}
