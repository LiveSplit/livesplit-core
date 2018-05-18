#![allow(missing_docs)]

pub use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};

pub fn utc_now() -> DateTime<Utc> {
    Utc::now()
}
