// For js! macro.
#![recursion_limit = "1024"]

#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use crate::windows::*;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use crate::linux::*;

#[cfg(target_os = "emscripten")]
pub mod emscripten;
#[cfg(target_os = "emscripten")]
pub use crate::emscripten::*;
#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate stdweb;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub mod wasm;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use crate::wasm::*;

#[cfg(not(any(
    windows,
    target_os = "linux",
    target_os = "emscripten",
    all(target_arch = "wasm32", target_os = "unknown")
)))]
pub mod other;
#[cfg(not(any(
    windows,
    target_os = "linux",
    target_os = "emscripten",
    all(target_arch = "wasm32", target_os = "unknown")
)))]
pub use crate::other::*;
