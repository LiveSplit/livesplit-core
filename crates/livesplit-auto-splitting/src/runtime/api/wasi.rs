use std::path::Path;

use wasmtime_wasi::{preview1::WasiP1Ctx, DirPerms, FilePerms, WasiCtxBuilder};

use crate::wasi_path;

pub fn build(script_path: Option<&Path>) -> WasiP1Ctx {
    let mut wasi = WasiCtxBuilder::new();

    if let Some(script_path) = script_path {
        if let Some(path) = wasi_path::from_native(script_path) {
            wasi.env("SCRIPT_PATH", &path);
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
            // Unfortunate if this fails, but we should still continue.
            let _ = wasi.preopened_dir(
                std::str::from_utf8(&[b'\\', b'\\', b'?', b'\\', drive, b':', b'\\']).unwrap(),
                std::str::from_utf8(&[b'/', b'm', b'n', b't', b'/', drive]).unwrap(),
                DirPerms::READ,
                FilePerms::READ,
            );
        }

        // FIXME: Unfortunately wasmtime doesn't support us defining our own
        // file system logic anymore.

        // wasi.push_dir(Box::new(DeviceDir), PathBuf::from("/mnt/device"))
        //     .unwrap();
    }
    #[cfg(not(windows))]
    {
        // Unfortunate if this fails, but we should still continue.
        let _ = wasi.preopened_dir("/", "/mnt", DirPerms::READ, FilePerms::READ);
    }
    wasi.build_p1()
}
