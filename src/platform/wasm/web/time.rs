use js_sys::{Date, Reflect};
use std::{cell::Cell, ops::Sub};
use time::UtcOffset;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Performance, VisibilityState};

pub use time::{Duration, OffsetDateTime as DateTime};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
#[repr(transparent)]
pub struct Instant(Duration);

// Every browser's `performance.now()` implementation is not spec compliant,
// unless the browser is running on Windows. On every other operating system,
// `performance.now()` does not properly keep ticking while the operating system
// is suspended / sleeping. There isn't much that we can do. What we can do is
// we calculate the initial difference between `performance.now()` and
// `Date.now()` and store it in a thread local. Later, when the phone gets
// locked, `performance.now()` starts to break. However, we can detect when the
// phone gets unlocked again by listening to the `visibilitychange` event. This
// is where we can update the difference again. This of course isn't ideal, as
// `Date.now()` gets adjusted by NTP synchronizations, but it's the best we can
// do.
//
// More information:
// https://developer.mozilla.org/en-US/docs/Web/API/Performance/now#ticking_during_sleep
fn init_fallback(performance: &Performance) {
    let Some(window) = web_sys::window() else {
        // Not running in a browser environment.
        return;
    };

    if window.navigator().platform().is_ok_and(|v| v == "Win32") {
        // Windows is not affected by this issue.
        return;
    }

    let Some(document) = window.document() else {
        // Not running in a browser environment.
        return;
    };

    DIFF_TO_DATE_NOW.set(Date::now() - performance.now());

    let callback = Closure::wrap(Box::new({
        let document = document.clone();
        move || {
            if document.visibility_state() == VisibilityState::Visible {
                PERFORMANCE.with(|p| DIFF_TO_DATE_NOW.set(Date::now() - p.now()));
            }
        }
    }) as Box<dyn FnMut()>);

    if document
        .add_event_listener_with_callback("visibilitychange", callback.as_ref().unchecked_ref())
        .is_ok()
    {
        // Leak the callback to keep it alive. This is only done once, and we
        // need it for the entire duration of the web app anyway.
        callback.forget();
    }
}

thread_local! {
    static DIFF_TO_DATE_NOW: Cell<f64> = const { Cell::new(0.0) };
    static PERFORMANCE: Performance = {
        let performance: Performance = Reflect::get(&js_sys::global(), &JsValue::from_str("performance"))
            .expect("Failed to get performance from global object")
            .unchecked_into();

        init_fallback(&performance);

        performance
    };
}

impl Instant {
    pub fn now() -> Self {
        let secs = PERFORMANCE.with(|p| p.now() + DIFF_TO_DATE_NOW.get()) * 0.001;
        Instant(Duration::seconds_f64(secs))
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
