//! Translating WASI Paths

use std::path::{Component, Path, PathBuf};

// Windows Paths Documentation:
// https://chrisdenton.github.io/omnipath/
// https://googleprojectzero.blogspot.com/2016/02/the-definitive-guide-on-win32-to-nt.html
// https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file
// https://stackoverflow.com/a/46019856
// https://learn.microsoft.com/en-us/dotnet/standard/io/file-path-formats
// https://medium.com/walmartglobaltech/dos-file-path-magic-tricks-5eda7a7a85fa
// https://reverseengineering.stackexchange.com/a/3799

// TL;DR:
// 1. Win32 paths may be in one of various relative forms that depend on hidden
//    environment variables, such as `C:\Windows` (absolute on C drive),
//    `Windows\System32\user32.dll` (relative to current dir on current drive),
//    `C:Windows` (relative to current dir on C drive), or `\Windows` (absolute
//    on current drive). There's also `\\server\share` paths that are called UNC
//    paths.
// 2. These paths then get converted into a form that's rooted in a device
//    (called device path) starting with `\\.\` followed by a device name. UNC
//    paths get mapped to `\\.\UNC\server\share`. If a reserved device is part
//    of the path, it takes precedence and becomes the device path
//    (`C:\some\dir\COM1.txt` -> `\\.\COM1`).
// 3. The paths then get normalized / canonicalized (forward slashes converted,
//    .. and . resolved, some spaces and dots get removed) into the `\\?\` form
//    (normalized device path). Because this is the step that replaces forward
//    slashes by backward slashes, all previous forms mentioned may use forward
//    slashes instead. `Path::canonicalize` handles all three steps, meaning a
//    path returned by it starts with `\\?\` and skips all these three steps
//    when used.
// 4. The `\\?\` form gets passed almost directly to NT, though it gets replaced
//    with `\??\`. At this point it is an NT path. The NT path `\??\` matches
//    `\GLOBAL??\` where the devices are then looked up. The device names may
//    actually be "symbolic links" in the NT object namespace to other devices
//    (so a symbolic link from one NT path to another). So for example `C:` is
//    actually a symbolic link at `\GLOBAL??\C:` to `\Device\HarddiskVolume1`
//    (or any other number). Various other forms of NT paths are also possible,
//    but you can't get to them from a Win32 path (except via the device symlink
//    called `GLOBALROOT`). The driver is then chosen based on the device that
//    it resolves to.
//
// Depending on what kind of Win32 path you have, you may skip some of the steps
// on the way.

// Notes on Rust's handling of the paths:
//
// Rust does not really treat `\\.\` and `\\?\` the same, which means that
// `\\.\C:\` is parsed as a raw `DeviceNS` prefix, while `\\?\C:\` is parsed as
// a `VerbatimDisk` prefix, special handling is needed to treat them the same.

/// Translates `original_path` into a path that is accessible through the WASI
/// file system, so a Windows path of `C:\foo\bar.exe` would be returned as
/// `/mnt/c/foo/bar.exe`. The original path should be canonicalized or at least
/// absolute.
pub fn from_native(original_path: &Path) -> Option<Box<str>> {
    const BASE: &str = "/mnt";
    let mut path = String::from(BASE);

    for component in original_path.components() {
        if !path.ends_with('/') {
            path.push('/');
        }
        match component {
            Component::Prefix(_prefix) => {
                #[cfg(windows)]
                use std::path::Prefix;
                #[cfg(windows)]
                match _prefix.kind() {
                    Prefix::VerbatimDisk(disk) | Prefix::Disk(disk) => {
                        path.push(disk.to_ascii_lowercase() as char)
                    }
                    Prefix::VerbatimUNC(server, share) | Prefix::UNC(server, share) => {
                        path.push_str("device/");
                        path.push_str("UNC/");
                        path.push_str(server.to_str()?);
                        path.push('/');
                        path.push_str(share.to_str()?);
                    }
                    Prefix::Verbatim(value) | Prefix::DeviceNS(value) => {
                        if let [c @ b'A'..=b'Z' | c @ b'a'..=b'z', b':'] = value.as_encoded_bytes()
                        {
                            path.push(c.to_ascii_lowercase() as char);
                        } else {
                            path.push_str("device/");
                            path.push_str(value.to_str()?);
                        }
                    }
                }
            }
            Component::Normal(c) => path.push_str(c.to_str()?),
            Component::RootDir =>
            {
                #[cfg(windows)]
                if path.len() == BASE.len() {
                    return None;
                }
            }
            Component::CurDir => path.push('.'),
            Component::ParentDir => path.push_str(".."),
        }
    }

    Some(path.into_boxed_str())
}

/// Translates from a path accessible through the WASI file system to a path
/// accessible outside that, so a WASI path of `/mnt/c/foo/bar.exe` would be
/// translated on Windows to `C:\foo\bar.exe`. If `supports_device_path` is
/// true, then the path will be translated to a path that uses the `\\?\`
/// prefix. The DOS device path syntax allows for longer paths, but not every
/// application may support it. The parameter is ignored on non-Windows
/// platforms.
pub fn to_native(wasi_path_str: &str, supports_device_path: bool) -> Option<PathBuf> {
    let path = wasi_path_str.strip_prefix("/mnt")?;
    let _after_slash = path.strip_prefix('/')?;
    #[cfg(windows)]
    {
        // Backslashes would mess up the path, so we don't allow them.
        if _after_slash.contains('\\') {
            return None;
        }

        let mut path_buf = String::with_capacity(path.len() + supports_device_path as usize * 4);
        if supports_device_path {
            path_buf.push_str(r"\\?\");
        }

        let rem = match _after_slash.as_bytes() {
            [c @ b'a'..=b'z', b'/', rem @ ..] => {
                let drive = c.to_ascii_uppercase();
                path_buf.push(drive as char);
                path_buf.push(':');
                rem
            }
            [b'd', b'e', b'v', b'i', b'c', b'e', b'/', rem @ ..] => {
                if supports_device_path {
                    path_buf.pop();
                    rem
                } else {
                    match rem {
                        [b'U', b'N', b'C', b'/', rem @ ..] => {
                            path_buf.push('\\');
                            rem
                        }
                        [c @ b'a'..=b'z' | c @ b'A'..=b'Z', b':', b'/', rem @ ..] => {
                            let drive = c.to_ascii_uppercase();
                            path_buf.push(drive as char);
                            path_buf.push(':');
                            rem
                        }
                        _ => return None,
                    }
                }
            }
            _ => return None,
        };

        // SAFETY: We know that the path is valid UTF-8 because it was
        // originally a WASI path, which is valid UTF-8 and we split after a
        // slash.
        let rem = unsafe { std::str::from_utf8_unchecked(rem) };
        rem.split('/').for_each(|segment| {
            path_buf.push('\\');
            path_buf.push_str(segment);
        });

        Some(path_buf.into())
    }
    #[cfg(not(windows))]
    {
        _ = supports_device_path;
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
            from_native(Path::new(r"C:\Windows\System32\user32.dll")),
            Some(r"/mnt/c/Windows/System32/user32.dll".into())
        );
        assert_eq!(
            from_native(Path::new(r"\\?\C:\Windows\System32\user32.dll")),
            Some(r"/mnt/c/Windows/System32/user32.dll".into())
        );

        assert_eq!(
            from_native(Path::new(r"C:Windows\System32\user32.dll")),
            Some(r"/mnt/c/Windows/System32/user32.dll".into())
        );

        assert_eq!(
            from_native(Path::new(r"\\server\share\bar.exe")),
            Some(r"/mnt/device/UNC/server/share/bar.exe".into())
        );
        assert_eq!(
            from_native(Path::new(r"\\?\UNC\server\share\bar.exe")),
            Some(r"/mnt/device/UNC/server/share/bar.exe".into())
        );

        assert_eq!(
            from_native(Path::new(r"\\.\C:\Test\Foo.txt")),
            Some(r"/mnt/c/Test/Foo.txt".into())
        );
        assert_eq!(
            from_native(Path::new(
                r"\\.\Volume{b75e2c83-0000-0000-0000-602f00000000}\Test\Foo.txt"
            )),
            Some(r"/mnt/device/Volume{b75e2c83-0000-0000-0000-602f00000000}/Test/Foo.txt".into())
        );
        assert_eq!(
            from_native(Path::new(
                r"\\?\Volume{b75e2c83-0000-0000-0000-602f00000000}\Test\Foo.txt"
            )),
            Some(r"/mnt/device/Volume{b75e2c83-0000-0000-0000-602f00000000}/Test/Foo.txt".into())
        );
    }

    #[cfg(windows)]
    #[test]
    fn test_wasi_to_windows() {
        assert_eq!(
            to_native(r"/mnt/c/Windows/System32/user32.dll", false),
            Some(r"C:\Windows\System32\user32.dll".into())
        );
        assert_eq!(
            to_native(r"/mnt/c/Windows/System32/user32.dll", true),
            Some(r"\\?\C:\Windows\System32\user32.dll".into())
        );

        assert_eq!(
            to_native(r"/mnt/device/UNC/server/share/bar.exe", false),
            Some(r"\\server\share\bar.exe".into())
        );
        assert_eq!(
            to_native(r"/mnt/device/UNC/server/share/bar.exe", true),
            Some(r"\\?\UNC\server\share\bar.exe".into())
        );

        assert_eq!(
            to_native(r"/mnt/device/C:/Windows/System32/user32.dll", false),
            Some(r"C:\Windows\System32\user32.dll".into())
        );
        assert_eq!(
            to_native(r"/mnt/device/C:/Windows/System32/user32.dll", true),
            Some(r"\\?\C:\Windows\System32\user32.dll".into())
        );

        assert_eq!(
            to_native(
                r"/mnt/device/Volume{b75e2c83-0000-0000-0000-602f00000000}/Test/Foo.txt",
                false
            ),
            None,
        );
        assert_eq!(
            to_native(
                r"/mnt/device/Volume{b75e2c83-0000-0000-0000-602f00000000}/Test/Foo.txt",
                true
            ),
            Some(r"\\?\Volume{b75e2c83-0000-0000-0000-602f00000000}\Test\Foo.txt".into()),
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
        assert_eq!(
            to_native(r"/mnt/foo/bar.exe", false),
            Some(r"/foo/bar.exe".into())
        );
    }
}
