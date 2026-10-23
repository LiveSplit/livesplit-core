cfg_select! {
    feature = "wasm-web" => {
        mod web;
        pub use self::web::*;
    }
    _ => {
        mod unknown;
        pub use self::unknown::*;
    }
}
