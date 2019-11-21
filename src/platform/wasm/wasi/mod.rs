#![allow(missing_docs)]

pub use chrono::{DateTime, Duration, Local, Utc};
pub use indexmap;
pub use std::time::{Instant, SystemTime};

use chrono::NaiveDateTime;

pub fn utc_now() -> DateTime<Utc> {
    let unix = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    DateTime::from_utc(
        NaiveDateTime::from_timestamp(unix.as_secs() as _, unix.subsec_nanos()),
        Utc,
    )
}
