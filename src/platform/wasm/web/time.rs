use js_sys::{Date, Reflect};
use std::ops::Sub;
use time::UtcOffset;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::Performance;

pub use time::{Duration, OffsetDateTime as DateTime};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub struct Instant(Duration);

thread_local! {
    static PERFORMANCE: Performance =
        Reflect::get(&js_sys::global(), &JsValue::from_str("performance"))
            .expect("Failed to get performance from global object")
            .unchecked_into();
}

impl Instant {
    pub fn now() -> Self {
        let secs = PERFORMANCE.with(|p| p.now()) * 0.001;
        Instant(Duration::seconds_f64(secs))
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, rhs: Duration) -> Instant {
        Self(self.0 - rhs)
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.0 - rhs.0
    }
}

pub fn utc_now() -> DateTime {
    DateTime::from_unix_timestamp_nanos((Date::now() * 1_000_000.0) as i128)
        .expect("Can't query current date")
}

pub fn to_local(date_time: DateTime) -> DateTime {
    let date_time = date_time.to_offset(UtcOffset::UTC);

    let (year, month, day) = date_time.to_calendar_date();
    let (hour, minute, second, milli) = date_time.to_hms_milli();

    let offset_in_minutes = -Date::new_with_year_month_day_hr_min_sec_milli(
        year as u32,
        month as i32 - 1,
        day as i32,
        hour as i32,
        minute as i32,
        second as i32,
        milli as i32,
    )
    .get_timezone_offset();

    let total_seconds = (offset_in_minutes * 60.0) as i32;
    let seconds = total_seconds % 60;
    let total_minutes = total_seconds / 60;
    let minutes = total_minutes % 60;
    let hours = total_minutes / 60;

    date_time.to_offset(
        UtcOffset::from_hms(hours as i8, minutes as i8, seconds as i8).unwrap_or(UtcOffset::UTC),
    )
}
