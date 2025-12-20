use std::{ffi::OsString, iter, os::windows::ffi::OsStringExt, path::PathBuf};

use wasmtime_wasi::{DirPerms, FilePerms, WasiCtxBuilder};
use windows_sys::Win32::{
    Foundation::{ERROR_MORE_DATA, MAX_PATH, NO_ERROR},
    NetworkManagement::WNet::WNetGetConnectionW,
    Storage::FileSystem::{GetDriveTypeA, GetLogicalDrives},
    System::WindowsProgramming::DRIVE_REMOTE,
};

use crate::wasi_path;

pub fn add_drives(wasi: &mut WasiCtxBuilder) {
    let mut remote_buffer = Vec::new();

    for drive in iter_drives() {
        // Unfortunate if this fails, but we should still continue.
        let _ = wasi.preopened_dir(
            str::from_utf8(&[b'\\', b'\\', b'?', b'\\', drive, b':', b'\\']).unwrap(),
            str::from_utf8(&[b'/', b'm', b'n', b't', b'/', drive]).unwrap(),
            DirPerms::READ,
            FilePerms::READ,
        );

        if is_network_drive(drive)
            && let Some(remote_path) = resolve_network_drive_path(drive, &mut remote_buffer)
            && let Some(wasi_path) = wasi_path::from_native(&remote_path)
        {
            // Unfortunate if this fails, but we should still continue.
            let _ = wasi.preopened_dir(remote_path, wasi_path, DirPerms::READ, FilePerms::READ);
        }
    }

    // FIXME: Unfortunately wasmtime doesn't support us defining our own
    // file system logic anymore. Above we try to at least support mapped
    // network drives, but all other device paths are unsupported for now.

    // wasi.push_dir(Box::new(DeviceDir), PathBuf::from("/mnt/device"))
    //     .unwrap();
}

fn iter_drives() -> impl Iterator<Item = u8> {
    // SAFETY: This is always safe to call.
    let mut drives = unsafe { GetLogicalDrives() };

    iter::from_fn(move || {
        let drive_idx = drives.trailing_zeros();
        if drive_idx >= 26 {
            return None;
        }
        drives &= !(1 << drive_idx);
        Some(drive_idx as u8 + b'a')
    })
}

fn is_network_drive(drive: u8) -> bool {
    // SAFETY: We pass a valid nul-terminated string.
    let drive_type = unsafe { GetDriveTypeA([drive, b':', b'\\', b'\0'].as_ptr()) };

    drive_type == DRIVE_REMOTE
}

fn resolve_network_drive_path(drive: u8, remote_buffer: &mut Vec<u16>) -> Option<PathBuf> {
    remote_buffer.clear();
    remote_buffer.reserve(MAX_PATH as usize);

    let drive_path = [drive, b':', b'\0'].map(|b| b as u16);

    loop {
        let mut remote_len = remote_buffer.capacity().try_into().ok()?;

        // SAFETY: We pass a valid nul-terminated string and a valid
        // buffer, with the length set correctly.
        let res = unsafe {
            WNetGetConnectionW(
                drive_path.as_ptr(),
                remote_buffer.as_mut_ptr(),
                &mut remote_len,
            )
        };

        if res == ERROR_MORE_DATA {
            let previous_cap = remote_buffer.capacity();
            remote_buffer.reserve(remote_len as usize);
            if remote_buffer.capacity() == previous_cap {
                // Failed to reserve more space.
                return None;
            }
            continue;
        } else if res == NO_ERROR {
            // SAFETY: The buffer is properly initialized now, at
            // least up until the nul-terminator.
            unsafe {
                // There should always be a nul-terminator, but if there isn't,
                // it's better if we return `None` than read out of bounds /
                // uninitialized bytes.
                let len = remote_buffer
                    .spare_capacity_mut()
                    .iter()
                    .position(|b| b.assume_init() == 0)?;

                remote_buffer.set_len(len);
            }

            let remote_path = PathBuf::from(OsString::from_wide(remote_buffer));

            return Some(remote_path);
        } else {
            return None;
        }
    }
}
