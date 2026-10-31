//! Privacy-check defaults for backends without a separate native grant API.
//!
//! This does not create hooks, inspect desktop state or claim that registration
//! will succeed. Normal backend creation still checks its own resource access.

use crate::{ConsumePreference, Result};

/// No separate privacy grant is needed; resource access remains with hook creation.
pub fn check(_consume: ConsumePreference) -> Result<()> {
    Ok(())
}

/// Preserve the absence of a native privacy prompt on these backends.
pub fn request(_consume: ConsumePreference) -> Result<()> {
    Ok(())
}
