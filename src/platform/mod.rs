#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
mod wasm;
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
pub use self::wasm::*;

#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
mod normal;
#[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
pub use self::normal::*;
