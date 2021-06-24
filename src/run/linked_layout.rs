use serde::{Deserialize, Serialize};
use crate::platform::prelude::*;

/// A `LinkedLayout` associates a [`Layout`](crate::Layout) with a
/// [`Run`](crate::Run). If the [`Run`](crate::Run) has a `LinkedLayout`, it is
/// supposed to be visualized with the [`Layout`](crate::Layout) that is linked
/// with it.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum LinkedLayout {
    /// The default [`Layout`](crate::Layout) is associated with the
    /// [`Run`](crate::Run).
    Default,
    /// A [`Layout`](crate::Layout) that is specified through its path on the
    /// file system is associated with the [`Run`](crate::Run).
    Path(String),
}
