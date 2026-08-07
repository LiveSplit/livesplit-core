//! Optional path-based text engines for renderers that do not render text
//! themselves.

#[cfg(not(feature = "parley-text-engine"))]
mod cosmic;
#[cfg(feature = "parley-text-engine")]
mod parley;

#[cfg(not(feature = "parley-text-engine"))]
pub use cosmic::*;
#[cfg(feature = "parley-text-engine")]
pub use parley::*;
