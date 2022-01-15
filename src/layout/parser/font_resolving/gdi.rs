use std::{ffi::OsStr, mem, ptr, str};

use mem::MaybeUninit;
use winapi::{
    shared::windef::{HDC, HFONT},
    um::wingdi::{
        CreateCompatibleDC, CreateFontW, DeleteDC, DeleteObject, GetFontData, GetTextMetricsW,
        SelectObject, DEFAULT_PITCH, DEFAULT_QUALITY, GDI_ERROR, HGDI_ERROR, TEXTMETRICW,
    },
};

pub struct DeviceContext(HDC);

impl Drop for DeviceContext {
    fn drop(&mut self) {
        unsafe {
            DeleteDC(self.0);
        }
    }
}

impl DeviceContext {
    pub fn new() -> Option<Self> {
        unsafe {
            let res = CreateCompatibleDC(ptr::null_mut());
            if res.is_null() {
                return None;
            }
            Some(Self(res))
        }
    }

    pub fn select_font(&mut self, font: &mut Font) -> Option<()> {
        unsafe {
            let res = SelectObject(self.0, font.0.cast());
            if res.is_null() || res == HGDI_ERROR {
                return None;
            }
            Some(())
        }
    }

    pub fn get_font_table(&mut self, name: [u8; 4]) -> Option<Vec<u8>> {
        unsafe {
            let name = u32::from_le_bytes(name);
            let len = GetFontData(self.0, name, 0, ptr::null_mut(), 0);
            if len == GDI_ERROR {
                return None;
            }
            let mut name_table = Vec::<u8>::with_capacity(len as usize);
            let len = GetFontData(self.0, name, 0, name_table.as_mut_ptr().cast(), len);
            if len == GDI_ERROR {
                return None;
            }
            name_table.set_len(len as usize);
            Some(name_table)
        }
    }

    pub fn get_font_metrics(&mut self) -> Option<TEXTMETRICW> {
        unsafe {
            let mut text_metric = MaybeUninit::uninit();
            let res = GetTextMetricsW(self.0, text_metric.as_mut_ptr());
            if res == 0 {
                return None;
            }
            Some(text_metric.assume_init())
        }
    }
}

pub struct Font(HFONT);

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            DeleteObject(self.0.cast());
        }
    }
}

impl Font {
    pub fn new(name: &str, bold: bool, italic: bool) -> Option<Self> {
        use std::os::windows::ffi::OsStrExt;

        let mut name_buf = [0; 32];
        let min_len = name.len().min(32);
        name_buf[..min_len].copy_from_slice(&name.as_bytes()[..min_len]);

        let name = OsStr::new(str::from_utf8(&name_buf).ok()?)
            .encode_wide()
            .collect::<Vec<u16>>();

        unsafe {
            let res = CreateFontW(
                0,
                0,
                0,
                0,
                if bold { 700 } else { 400 },
                italic as _,
                0,
                0,
                0,
                0,
                0,
                DEFAULT_QUALITY,
                DEFAULT_PITCH,
                name.as_ptr(),
            );
            if res.is_null() {
                return None;
            }
            Some(Self(res))
        }
    }
}
