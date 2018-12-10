#![recursion_limit = "1024"]

#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows::*;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "emscripten")]
pub mod emscripten;
#[cfg(target_os = "emscripten")]
pub use emscripten::*;
#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate stdweb;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub mod wasm;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use wasm::*;

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
pub use other::*;
