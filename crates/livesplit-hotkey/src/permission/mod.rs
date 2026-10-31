//! Native privacy permissions, separate from creating hooks or registering keys.
//!
//! Only the macOS backend currently uses a separate privacy-grant API. Other backends
//! still report resource, device and registration failures when creating hooks;
//! a successful privacy check does not promise that those operations will work.

use core::fmt;

#[cfg(any(all(feature = "std", target_os = "macos"), test))]
pub mod macos;

#[cfg(all(feature = "std", target_os = "macos"))]
pub use crate::macos::permission::{check, request};
#[cfg(not(all(feature = "std", target_os = "macos")))]
mod unrestricted;
#[cfg(not(all(feature = "std", target_os = "macos")))]
pub use unrestricted::{check, request};

/// A native privacy grant required by the selected keyboard hook.
///
/// The consume preference describes the desired hook behavior; this identifies
/// the specific native grant that the backend requires, rather than a portable
/// consume/listen capability. Currently only macOS queries these grants. Other
/// platforms do not return dummy permission identifiers.
///
/// These are permission identifiers, not localized application messages. They
/// do not describe device-file access, conflicting shortcuts or other failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Permission {
    /// The macOS Input Monitoring grant for listen-only keyboard taps.
    ///
    /// Available starting with macOS 10.15. On current macOS, this is managed in
    /// System Settings under Privacy & Security > Input Monitoring.
    InputMonitoring,
    /// The macOS Accessibility grant for filtering keyboard events or legacy
    /// keyboard monitoring.
    ///
    /// On current macOS, this is managed in System Settings under Privacy &
    /// Security > Accessibility.
    Accessibility,
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InputMonitoring => "Input Monitoring",
            Self::Accessibility => "Accessibility",
        })
    }
}
