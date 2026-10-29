//! Communication with speedrunning websites.
//!
//! Services are independently feature gated.

#[cfg(feature = "std")]
pub mod server_protocol;
#[cfg(feature = "speedrun-com")]
pub mod speedrun_com;
#[cfg(feature = "therun-gg")]
pub mod therun_gg;
