use std::{env, path::Path};

#[unsafe(no_mangle)]
pub extern "C" fn update() {
    assert!(env::args().next().is_none());
    assert!(env::current_exe().is_err());
    assert_eq!(env::current_dir().unwrap(), Path::new("/"));
    assert!(env::vars().next().is_none());

    unsafe {
        env::set_var("AUTOSPLITTER_HOST_SHOULDNT_SEE_THIS", "YES");
    }

    // but auto splitter should
    assert_eq!(
        env::var("AUTOSPLITTER_HOST_SHOULDNT_SEE_THIS").unwrap(),
        "YES",
    );
}

fn main() {}
