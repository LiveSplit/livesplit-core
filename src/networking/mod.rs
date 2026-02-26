//! The networking module provides functionality to communicate with various
//! speedrunning related websites, such as Speedrun.com to query and submit to
//! the leaderboards of most games. The module is optional and is not compiled
//! in by default.

#[cfg(feature = "std")]
pub mod server_protocol;
#[cfg(feature = "therun-gg")]
pub mod therun_gg;
