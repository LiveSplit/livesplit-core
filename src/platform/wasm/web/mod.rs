#![allow(missing_docs)]

mod time;

pub use self::time::*;
pub use chrono::{DateTime, Duration, Local, Utc};
pub use indexmap;

pub fn utc_now() -> DateTime<Utc> {
    Utc::now()
}
