#[cfg(feature = "std")]
pub use std::path::{Path, PathBuf};

#[cfg(not(feature = "std"))]
pub use alloc::string::String as PathBuf;
#[cfg(not(feature = "std"))]
pub use str as Path;

pub fn relative_to<'path>(
    path_buf: &'path mut PathBuf,
    source: &Path,
    dest: &'path Path,
) -> &'path Path {
    if is_absolute(dest) {
        return dest;
    }
    if let Some(parent) = parent(source) {
        alloc::borrow::ToOwned::clone_into(parent, path_buf);
        push(path_buf, dest);
    } else {
        alloc::borrow::ToOwned::clone_into(dest, path_buf);
    }
    path_buf
}

#[inline]
fn parent(path: &Path) -> Option<&Path> {
    #[cfg(feature = "std")]
    {
        path.parent()
    }
    #[cfg(not(feature = "std"))]
    {
        path.rsplit_once('/').map(|(parent, _)| parent)
    }
}

#[inline]
fn push(path_buf: &mut PathBuf, path: &Path) {
    #[cfg(feature = "std")]
    {
        path_buf.push(path);
    }
    #[cfg(not(feature = "std"))]
    {
        if is_absolute(path) {
            alloc::borrow::ToOwned::clone_into(path, path_buf);
            return;
        }
        if !path_buf.is_empty() && !path_buf.ends_with('/') {
            path_buf.push('/');
        }
        path_buf.push_str(path);
    }
}

#[inline]
fn is_absolute(path: &Path) -> bool {
    #[cfg(feature = "std")]
    {
        path.is_absolute()
    }
    #[cfg(not(feature = "std"))]
    {
        path.starts_with('/')
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::relative_to;
    use std::path::{Path, PathBuf};

    #[cfg(windows)]
    #[test]
    fn relative_to_relative_path_windows() {
        let mut buf = PathBuf::new();
        let base = Path::new(r"C:\base\splits.lss");
        let rel = Path::new(r"icons\icon.png");
        let joined = relative_to(&mut buf, base, rel);

        assert_eq!(joined, Path::new(r"C:\base\icons\icon.png"));
    }

    #[cfg(not(windows))]
    #[test]
    fn relative_to_relative_path_unix() {
        let mut buf = PathBuf::new();
        let base = Path::new("/base/splits.lss");
        let rel = Path::new("icons/icon.png");
        let joined = relative_to(&mut buf, base, rel);

        assert_eq!(joined, Path::new("/base/icons/icon.png"));
    }

    #[cfg(windows)]
    #[test]
    fn relative_to_relative_base_windows() {
        let mut buf = PathBuf::new();
        let base = Path::new(r"base\splits.lss");
        let rel = Path::new(r"icons\icon.png");
        let joined = relative_to(&mut buf, base, rel);

        assert_eq!(joined, Path::new(r"base\icons\icon.png"));
    }

    #[cfg(not(windows))]
    #[test]
    fn relative_to_relative_base_unix() {
        let mut buf = PathBuf::new();
        let base = Path::new("base/splits.lss");
        let rel = Path::new("icons/icon.png");
        let joined = relative_to(&mut buf, base, rel);

        assert_eq!(joined, Path::new("base/icons/icon.png"));
    }

    #[cfg(windows)]
    #[test]
    fn relative_to_no_parent_windows() {
        let mut buf = PathBuf::new();
        let base = Path::new(r"splits.lss");
        let rel = Path::new(r"icons\icon.png");
        let joined = relative_to(&mut buf, base, rel);

        assert_eq!(joined, Path::new(r"icons\icon.png"));
    }

    #[cfg(not(windows))]
    #[test]
    fn relative_to_no_parent_unix() {
        let mut buf = PathBuf::new();
        let base = Path::new("splits.lss");
        let rel = Path::new("icons/icon.png");
        let joined = relative_to(&mut buf, base, rel);

        assert_eq!(joined, Path::new("icons/icon.png"));
    }

    #[cfg(windows)]
    #[test]
    fn relative_to_absolute_path_windows() {
        let mut buf = PathBuf::new();
        let base = Path::new(r"C:\base\splits.lss");
        let abs = Path::new(r"C:\other\icon.png");
        let joined = relative_to(&mut buf, base, abs);

        assert_eq!(joined, abs);
    }

    #[cfg(not(windows))]
    #[test]
    fn relative_to_absolute_path_unix() {
        let mut buf = PathBuf::new();
        let base = Path::new("/base/splits.lss");
        let abs = Path::new("/other/icon.png");
        let joined = relative_to(&mut buf, base, abs);

        assert_eq!(joined, abs);
    }
}
