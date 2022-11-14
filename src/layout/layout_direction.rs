use serde::{Deserialize, Serialize};

/// Describes the direction the components of a layout are laid out in.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayoutDirection {
    /// The components are placed on top of each other vertically.
    #[default]
    Vertical,
    /// The components are placed next to each other horizontally.
    Horizontal,
}
