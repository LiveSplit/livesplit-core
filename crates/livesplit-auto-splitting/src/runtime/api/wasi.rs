use std::{
    path::{Path, PathBuf},
    str,
};

use wasi_common::{
    dir::{OpenResult, ReaddirCursor, ReaddirEntity},
    file::{FdFlags, Filestat, OFlags},
    ErrorExt, WasiCtx, WasiDir,
};
use wasmtime_wasi::{ambient_authority, WasiCtxBuilder};

use crate::wasi_path;

pub fn build(script_path: Option<&Path>) -> WasiCtx {
    let mut wasi = WasiCtxBuilder::new().build();

    if let Some(script_path) = script_path {
        if let Some(path) = wasi_path::from_native(script_path) {
            let _ = wasi.push_env("SCRIPT_PATH", &path);
        }
    }

    #[cfg(windows)]
    {
        // SAFETY: This is always safe to call.
        let mut drives = unsafe { windows_sys::Win32::Storage::FileSystem::GetLogicalDrives() };
        loop {
            let drive_idx = drives.trailing_zeros();
            if drive_idx >= 26 {
                break;
            }
            drives &= !(1 << drive_idx);
            let drive = drive_idx as u8 + b'a';
            if let Ok(path) = wasmtime_wasi::Dir::open_ambient_dir(
                str::from_utf8(&[b'\\', b'\\', b'?', b'\\', drive, b':', b'\\']).unwrap(),
                ambient_authority(),
            ) {
                wasi.push_dir(
                    Box::new(ReadOnlyDir(wasmtime_wasi::dir::Dir::from_cap_std(path))),
                    PathBuf::from(str::from_utf8(&[b'/', b'm', b'n', b't', b'/', drive]).unwrap()),
                )
                .unwrap();
            }
        }

        wasi.push_dir(Box::new(DeviceDir), PathBuf::from("/mnt/device"))
            .unwrap();
    }
    #[cfg(not(windows))]
    {
        if let Ok(path) = wasmtime_wasi::Dir::open_ambient_dir("/", ambient_authority()) {
            wasi.push_dir(
                Box::new(ReadOnlyDir(wasmtime_wasi::dir::Dir::from_cap_std(path))),
                PathBuf::from("/mnt"),
            )
            .unwrap();
        }
    }
    wasi
}

struct ReadOnlyDir(wasmtime_wasi::dir::Dir);

#[async_trait::async_trait]
impl WasiDir for ReadOnlyDir {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn open_file(
        &self,
        symlink_follow: bool,
        path: &str,
        oflags: OFlags,
        read: bool,
        write: bool,
        fdflags: FdFlags,
    ) -> Result<OpenResult, wasi_common::Error> {
        // We whitelist the OFlags and FdFlags to not accidentally allow
        // ways to modify the file system.
        const WHITELISTED_O_FLAGS: OFlags = OFlags::DIRECTORY;
        const WHITELISTED_FD_FLAGS: FdFlags = FdFlags::NONBLOCK;

        if write || !WHITELISTED_O_FLAGS.contains(oflags) || !WHITELISTED_FD_FLAGS.contains(fdflags)
        {
            return Err(wasi_common::Error::not_supported());
        }

        Ok(
            match self
                .0
                .open_file_(symlink_follow, path, oflags, read, write, fdflags)?
            {
                wasmtime_wasi::dir::OpenResult::Dir(d) => OpenResult::Dir(Box::new(ReadOnlyDir(d))),
                // We assume that wrapping the file type itself is not
                // necessary, because we ensure that the open flags don't allow
                // for any modifications anyway.
                wasmtime_wasi::dir::OpenResult::File(f) => OpenResult::File(Box::new(f)),
            },
        )
    }

    async fn readdir(
        &self,
        cursor: ReaddirCursor,
    ) -> Result<
        Box<dyn Iterator<Item = Result<ReaddirEntity, wasi_common::Error>> + Send>,
        wasi_common::Error,
    > {
        self.0.readdir(cursor).await
    }

    async fn read_link(&self, path: &str) -> Result<PathBuf, wasi_common::Error> {
        self.0.read_link(path).await
    }

    async fn get_filestat(&self) -> Result<Filestat, wasi_common::Error> {
        // FIXME: Make sure this says it's readonly, if it ever contains the
        // permissions.
        self.0.get_filestat().await
    }

    async fn get_path_filestat(
        &self,
        path: &str,
        follow_symlinks: bool,
    ) -> Result<Filestat, wasi_common::Error> {
        // FIXME: Make sure this says it's readonly, if it ever contains the
        // permissions.
        self.0.get_path_filestat(path, follow_symlinks).await
    }
}

#[cfg(windows)]
struct DeviceDir;

#[cfg(windows)]
#[async_trait::async_trait]
impl WasiDir for DeviceDir {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn open_file(
        &self,
        symlink_follow: bool,
        path: &str,
        oflags: OFlags,
        read: bool,
        write: bool,
        fdflags: FdFlags,
    ) -> Result<OpenResult, wasi_common::Error> {
        let (dir, file) = device_path(path)?;
        dir.open_file(symlink_follow, file, oflags, read, write, fdflags)
            .await
    }

    // FIXME: cap-primitives/src/windows/fs/get_path tries to strip `\\?\`,
    // which breaks paths that aren't valid without it, such as UNC paths:
    // https://github.com/bytecodealliance/cap-std/issues/348

    async fn read_link(&self, path: &str) -> Result<PathBuf, wasi_common::Error> {
        let (dir, file) = device_path(path)?;
        dir.read_link(file).await
    }

    async fn get_path_filestat(
        &self,
        path: &str,
        follow_symlinks: bool,
    ) -> Result<Filestat, wasi_common::Error> {
        let (dir, file) = device_path(path)?;
        dir.get_path_filestat(file, follow_symlinks).await
    }
}

#[cfg(windows)]
fn device_path(path: &str) -> Result<(ReadOnlyDir, &str), wasi_common::Error> {
    let (parent, file) = path
        .strip_suffix('/')
        .unwrap_or(path)
        .rsplit_once('/')
        .ok_or_else(wasi_common::Error::not_supported)?;

    let parent = wasi_path::to_native(&format!("/mnt/device/{parent}"), true)
        .ok_or_else(wasi_common::Error::not_supported)?;

    let dir = wasmtime_wasi::dir::Dir::from_cap_std(
        wasmtime_wasi::Dir::open_ambient_dir(parent, ambient_authority())
            .map_err(|_| wasi_common::Error::not_supported())?,
    );

    Ok((ReadOnlyDir(dir), file))
}
