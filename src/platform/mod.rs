cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        mod no_std;
        pub use self::no_std::*;
    } else if #[cfg(all(
        target_family = "wasm",
        not(any(
            target_os = "wasi",
            target_os = "emscripten",
        ))
    ))] {
        mod wasm;
        pub use self::wasm::*;
    } else {
        mod normal;
        pub use self::normal::*;
    }
}

pub mod math;

pub(crate) mod prelude {
    pub use alloc::{
        borrow::ToOwned,
        boxed::Box,
        format,
        string::{String, ToString},
        vec,
        vec::Vec,
    };
}
