// For js! macro.
#![recursion_limit = "1024"]
#![cfg_attr(not(feature = "std"), no_std)]

cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        mod other;
        pub use self::other::*;
    } else if #[cfg(windows)] {
        mod windows;
        pub use self::windows::*;
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        pub use self::linux::*;
    } else if #[cfg(target_os = "macos")] {
        mod macos;
        pub use self::macos::*;
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
        mod other;
        pub use self::other::*;
    }
}

mod key_code;
pub use self::key_code::*;

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test() {
        let hook = Hook::new().unwrap();
        hook.register(KeyCode::Numpad1, || println!("A")).unwrap();
        println!("Press Numpad1");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::Numpad1).unwrap();
        hook.register(KeyCode::Numpad4, || println!("B")).unwrap();
        println!("Press Numpad4");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::Numpad4).unwrap();
        hook.register(KeyCode::Numpad1, || println!("C")).unwrap();
        println!("Press Numpad1");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::Numpad1).unwrap();
    }
}
