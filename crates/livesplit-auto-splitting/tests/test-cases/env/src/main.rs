use std::env;

#[no_mangle]
pub extern "C" fn configure() {
    assert!(env::args().next().is_none());
    assert!(env::current_exe().is_err());
    assert!(env::current_dir().is_err());
    assert!(env::vars().next().is_none());

    env::set_var("AUTOSPLITTER_HOST_SHOULDNT_SEE_THIS", "YES");

    // but auto splitter should
    assert_eq!(
        env::var("AUTOSPLITTER_HOST_SHOULDNT_SEE_THIS").unwrap(),
        "YES",
    );
}

fn main() {}
