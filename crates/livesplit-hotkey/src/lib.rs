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
    } else if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
        cfg_if::cfg_if! {
            if #[cfg(feature = "wasm-web")] {
                mod wasm_web;
                pub use self::wasm_web::*;
            } else {
                mod wasm_unknown;
                pub use self::wasm_unknown::*;
            }
        }
    } else {
        pub mod other;
        pub use crate::other::*;
    }
}
