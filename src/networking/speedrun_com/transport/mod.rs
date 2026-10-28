//! Target-specific HTTP clients, redirect policy and response-body readers.

cfg_select! {
    target_family = "wasm" => {
        mod web;
        pub use web::{client, read_artwork};
    }
    _ => {
        mod native;
        pub use native::{client, read_artwork};
    }
}
