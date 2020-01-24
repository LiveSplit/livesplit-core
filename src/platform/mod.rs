cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        mod no_std;
        pub use self::no_std::*;
    } else if #[cfg(target_arch = "wasm32")] {
        mod wasm;
        pub use self::wasm::*;
    } else {
        mod normal;
        pub use self::normal::*;
    }
}

pub mod math;

pub(crate) mod prelude {
    pub use alloc::borrow::ToOwned;
    pub use alloc::boxed::Box;
    pub use alloc::string::String;
    pub use alloc::string::ToString;
    pub use alloc::vec::Vec;
    pub use alloc::{format, vec};
}
