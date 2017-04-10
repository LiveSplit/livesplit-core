#[macro_use]
extern crate quick_error;

#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows::*;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(not(any(windows, target_os = "linux")))]
pub mod other;
#[cfg(not(any(windows, target_os = "linux")))]
pub use other::*;
