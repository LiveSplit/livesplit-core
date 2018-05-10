#![allow(missing_docs)]

pub use std::time::{Duration, Instant};

use chrono::{Utc, DateTime};

pub fn utc_now() -> DateTime<Utc> {
    Utc::now()
}
