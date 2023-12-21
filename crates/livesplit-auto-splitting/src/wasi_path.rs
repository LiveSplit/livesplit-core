//! Translating WASI Paths

use std::path::{Component, Path, PathBuf, Prefix};

/// Translates `original_path` into a path that
/// is accessible through the WASI file system,
/// so a Windows path of `C:\foo\bar.exe` would
/// be returned as `/mnt/c/foo/bar.exe`.
pub fn from_native(original_path: &Path) -> Option<Box<str>> {
    let mut path = String::from("/mnt");
    for component in original_path.components() {
        if !path.ends_with('/') {
            path.push('/');
        }
        match component {
            Component::Prefix(prefix) => match prefix.kind() {
                Prefix::VerbatimDisk(disk) | Prefix::Disk(disk) => {
                    path.push(disk.to_ascii_lowercase() as char)
                }
                _ => return None,
            },
            Component::Normal(c) => {
                path.push_str(c.to_str()?);
            }
            Component::RootDir => {}
            Component::CurDir => path.push('.'),
            Component::ParentDir => path.push_str(".."),
        }
    }
    Some(path.into_boxed_str())
}

/// Translates from a path accessible through the WASI
/// file system to a path accessible outside that,
/// so a WASI path of `/mnt/c/foo/bar.exe` would
/// be translated on Windows to `C:\foo\bar.exe`.
pub fn to_native(wasi_path_str: &str) -> Option<PathBuf> {
    let path = wasi_path_str.strip_prefix("/mnt")?;
    let _after_slash = path.strip_prefix('/')?;
    #[cfg(windows)]
    {
        let mut path_buf = String::with_capacity(path.len());
        let [c @ b'a'..=b'z', b'/', ..] = _after_slash.as_bytes() else {
            return None;
        };
        let drive = c.to_ascii_uppercase();
        path_buf.push(drive as char);
        path_buf.push(':');
        _after_slash[2..].split('/').for_each(|segment| {
            path_buf.push('\\');
            path_buf.push_str(segment);
        });
        Some(path_buf.into())
    }
    #[cfg(not(windows))]
    {
        Some(PathBuf::from(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn test_windows_to_wasi() {
        assert_eq!(
            from_native(Path::new(r"C:\foo\bar.exe")),
            Some(r"/mnt/c/foo/bar.exe".into())
        );
        assert_eq!(
            from_native(Path::new(r"\\?\C:\foo\bar.exe")),
            Some(r"/mnt/c/foo/bar.exe".into())
        );
    }

    #[cfg(windows)]
    #[test]
    fn test_wasi_to_windows() {
        assert_eq!(
            to_native(r"/mnt/c/foo/bar.exe"),
            Some(r"C:\foo\bar.exe".into())
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn test_non_windows_to_wasi() {
        assert_eq!(
            from_native(Path::new(r"/foo/bar.exe")),
            Some(r"/mnt/foo/bar.exe".into())
        );
    }

    #[cfg(not(windows))]
    #[test]
    fn test_wasi_to_non_windows() {
        assert_eq!(to_native(r"/mnt/foo/bar.exe"), Some(r"/foo/bar.exe".into()));
    }
}
