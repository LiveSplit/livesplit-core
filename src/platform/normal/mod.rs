#![allow(missing_docs)]

pub use chrono::{DateTime, Duration, Local, Utc};
pub use indexmap;
pub use std::time::Instant;

pub fn utc_now() -> DateTime<Utc> {
    Utc::now()
}
