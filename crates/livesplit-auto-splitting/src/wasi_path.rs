//! Translating WASI Paths

use std::path;
use std::path::{Component, Path, PathBuf};

/// Translates `original_path` into a path that
/// is accessible through the WASI file system,
/// so a Windows path of `C:\foo\bar.exe` would
/// be returned as `/mnt/c/foo/bar.exe`.
pub fn path_to_wasi(original_path: &Path) -> Option<Box<str>> {
    let mut path = String::from("/mnt");
    for component in original_path.components() {
        if !path.ends_with('/') {
            path.push('/');
        }
        match component {
            path::Component::Prefix(prefix) => match prefix.kind() {
                path::Prefix::VerbatimDisk(disk) | path::Prefix::Disk(disk) => {
                    path.push(disk.to_ascii_lowercase() as char)
                }
                _ => return None,
            },
            path::Component::Normal(c) => {
                path.push_str(c.to_str()?);
            }
            path::Component::RootDir => {}
            path::Component::CurDir => path.push('.'),
            path::Component::ParentDir => path.push_str(".."),
        }
    }
    Some(path.into_boxed_str())
}

/// Translates from a path accessible through the WASI
/// file system to a path accessible outside that,
/// so a WASI path of `/mnt/c/foo/bar.exe` would
/// be translated on Windows to `C:\foo\bar.exe`.
pub fn wasi_to_path(wasi_path_str: &str) -> Option<PathBuf> {
    let wasi_path = PathBuf::from(wasi_path_str);
    let mut path = PathBuf::new();
    for (i, wasi_component) in wasi_path.components().enumerate() {
        match (i, wasi_component) {
            (0, Component::RootDir) => (),
            (1, Component::Normal(mnt)) if mnt == "mnt" => (),
            (2, Component::Normal(d)) if d.len() == 1 && d.to_ascii_lowercase() == d => {
                let disk = d.to_string_lossy().to_ascii_uppercase();
                path.push(format!("{}{}{}", r"\\?\", disk, r":\"));
            },
            (2, Component::Normal(c)) if !path.has_root() => {
                if let Some(root) = [r"/", r"\"].into_iter().map(PathBuf::from).find(|p| p.has_root()) {
                    path.push(root);
                }
                path.push(c);
            },
            (i, Component::Normal(c)) if 2 <= i => path.push(c),
            _ => return None,
        }
    }
    Some(path)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn test_windows_to_wasi() {
        assert_eq!(path_to_wasi(Path::new(r"C:\foo\bar.exe")), Some(r"/mnt/c/foo/bar.exe".into()));
    }

    #[cfg(windows)]
    #[test]
    fn test_wasi_to_windows() {
        assert_eq!(wasi_to_path(r"/mnt/c/foo/bar.exe"), Some(r"C:\foo\bar.exe".into()));
    }

    #[cfg(not(windows))]
    #[test]
    fn test_non_windows_to_wasi() {
        assert_eq!(path_to_wasi(Path::new(r"/foo/bar.exe")), Some(r"/mnt/foo/bar.exe".into()));
    }

    #[cfg(not(windows))]
    #[test]
    fn test_wasi_to_non_windows() {
        assert_eq!(wasi_to_path(r"/mnt/foo/bar.exe"), Some(r"/foo/bar.exe".into()));
    }
}
