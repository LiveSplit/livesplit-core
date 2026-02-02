use core::{mem::MaybeUninit, ptr, str};

use windows_sys::Win32::{
    Foundation::HANDLE,
    Graphics::Gdi::{
        CreateCompatibleDC, CreateFontW, DEFAULT_PITCH, DEFAULT_QUALITY, DeleteDC, DeleteObject,
        GDI_ERROR, GetFontData, GetTextMetricsW, HDC, HFONT, SelectObject, TEXTMETRICW,
    },
};

pub struct DeviceContext(HDC);

impl Drop for DeviceContext {
    fn drop(&mut self) {
        // SAFETY: We own the `HDC` and this is in `Drop`, so it is safe to call
        // `DeleteDC`.
        unsafe {
            DeleteDC(self.0);
        }
    }
}

impl DeviceContext {
    pub fn new() -> Option<Self> {
        // SAFETY: We are creating a new `HDC` and we are not passing any
        // parameters to `CreateCompatibleDC`. We also properly check the
        // result.
        unsafe {
            let res = CreateCompatibleDC(ptr::null_mut());
            if res.is_null() {
                return None;
            }
            Some(Self(res))
        }
    }

    pub fn select_font(&mut self, font: &mut Font) -> Option<()> {
        // SAFETY: We call `SelectObject` with a valid `HDC` and a valid
        // `HFONT`. We also properly check the result.
        unsafe {
            let res = SelectObject(self.0, font.0);
            if res.is_null() || res == GDI_ERROR as HANDLE {
                return None;
            }
            Some(())
        }
    }

    pub fn get_font_table(&mut self, name: [u8; 4]) -> Option<Vec<u8>> {
        // SAFETY: We call the first `GetFontData` with a valid `HDC` and
        // otherwise default constants. We also properly check the result. For
        // the second `GetFontData` we pass pointer and the size of the buffer
        // we allocated. We also properly check the result again. At this point
        // the function guarantees us that it properly copied the data into the
        // buffer, so we can set the length of the buffer.
        unsafe {
            let name = u32::from_le_bytes(name);
            let len = GetFontData(self.0, name, 0, ptr::null_mut(), 0);
            if len as i32 == GDI_ERROR {
                return None;
            }
            let mut name_table = Vec::<u8>::with_capacity(len as usize);
            let len = GetFontData(self.0, name, 0, name_table.as_mut_ptr().cast(), len);
            if len as i32 == GDI_ERROR {
                return None;
            }
            name_table.set_len(len as usize);
            Some(name_table)
        }
    }

    pub fn get_font_metrics(&mut self) -> Option<TEXTMETRICW> {
        // SAFETY: We call `GetTextMetricsW` with a valid `HDC`. The
        // `text_metric` is an out parameter, so we can use `MaybeUninit` to
        // initialize it. We then check the result and can assume it's properly
        // initialized.
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
        // SAFETY: We own the `HFONT` and this is in `Drop`, so it is safe to
        // call `DeleteObject`.
        unsafe {
            DeleteObject(self.0);
        }
    }
}

impl Font {
    pub fn new(name: &str, bold: bool, italic: bool) -> Option<Self> {
        let mut name_buf = [0; 32];
        let mut cursor = &mut name_buf[..31];

        for c in name.chars() {
            if c.len_utf16() < cursor.len() {
                let len = c.encode_utf16(cursor).len();
                cursor = &mut cursor[len..];
            }
        }

        // SAFETY: The only unsafe parameter is the pointer to the name, which
        // is not allowed to exceed 32 characters and needs to be properly
        // nul-terminated. Our `name_buf` is exactly 32 characters long and we
        // only allow our cursor to write up to 31 characters to ensure that at
        // least the last character stays a zero. We also check the result.
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
                DEFAULT_QUALITY as _,
                DEFAULT_PITCH as _,
                name_buf.as_ptr(),
            );
            if res.is_null() {
                return None;
            }
            Some(Self(res))
        }
    }
}
