#[unsafe(no_mangle)]
pub extern "C" fn update() {
    assert!(
        std::fs::write(
            "shouldnt_exist.txt",
            "This file should never exist. File a bug if you see this.",
        )
        .is_err()
    );

    let path = std::path::PathBuf::from(std::env::var("SCRIPT_PATH").unwrap());
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "read-only fixture");
    assert!(std::fs::write(&path, "modified").is_err());
    let directory = path.parent().unwrap();
    assert!(std::fs::read_dir(directory).is_ok());
    assert!(std::fs::write(directory.join("shouldnt_exist.txt"), "modified").is_err());
    assert!(std::fs::create_dir(directory.join("shouldnt_exist")).is_err());
    assert!(std::fs::remove_file(&path).is_err());
}

fn main() {}
