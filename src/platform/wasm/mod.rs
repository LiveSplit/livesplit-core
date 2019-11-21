cfg_if::cfg_if! {
    if #[cfg(target_os = "wasi")] {
        mod wasi;
        pub use self::wasi::*;
    } else if #[cfg(feature = "wasm-web")] {
        mod web;
        pub use self::web::*;
    } else {
        mod unknown;
        pub use self::unknown::*;
    }
}
