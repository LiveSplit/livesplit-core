use serde::{Deserialize, Serialize};

/// Describes the Alignment of the Title in the Title Component.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Alignment {
    /// Automatically align the title based on whether a game icon is shown.
    Auto,
    /// Always align the title to the left.
    Left,
    /// Always align the title to the center.
    Center,
}
