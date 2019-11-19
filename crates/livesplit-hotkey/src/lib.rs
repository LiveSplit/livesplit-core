// For js! macro.
#![recursion_limit = "1024"]
#![cfg_attr(not(feature = "std"), no_std)]

cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        pub mod other;
        pub use crate::other::*;
    } else if #[cfg(windows)] {
        pub mod windows;
        pub use crate::windows::*;
    } else if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub use crate::linux::*;
    } else if #[cfg(target_os = "emscripten")] {
        pub mod emscripten;
        pub use crate::emscripten::*;
        #[macro_use]
        extern crate stdweb;
    } else if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
        pub mod wasm;
        pub use crate::wasm::*;
    } else {
        pub mod other;
        pub use crate::other::*;
    }
}
