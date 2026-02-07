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

#[cfg(not(target_has_atomic = "ptr"))]
#[allow(unused)]
pub use alloc::rc::Rc as Arc;
#[cfg(target_has_atomic = "ptr")]
#[allow(unused)]
pub use alloc::sync::Arc;

pub mod math;
pub mod path;

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
