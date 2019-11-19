#![allow(missing_docs)]

mod time;

pub use self::time::*;
pub use chrono::{DateTime, Duration, Local, Utc};
pub use indexmap;

use core::mem;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, cap: usize) {
    unsafe {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

use chrono::NaiveDateTime;
use std::mem::MaybeUninit;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct FFIDateTime {
    secs: i64,
    nsecs: u32,
}

extern "C" {
    fn Date_now(data: *mut FFIDateTime);
}

pub fn utc_now() -> DateTime<Utc> {
    unsafe {
        let mut date_time = MaybeUninit::uninit();
        Date_now(date_time.as_mut_ptr());
        let date_time = date_time.assume_init();
        DateTime::from_utc(
            NaiveDateTime::from_timestamp(date_time.secs, date_time.nsecs),
            Utc,
        )
    }
}
