use crate::platform::prelude::*;
use core::marker::PhantomData;
use core::ops::{Add, Sub};
use core::sync::atomic::{self, AtomicPtr};
use derive_more::{Add, Neg, Sub};

/// A clock is a global handler that can be registered for providing the high
/// precision time stamps on a `no_std` target.
pub trait Clock: 'static {
    /// Returns the current point in time as a Duration.
    fn now(&self) -> Duration;
}

static CLOCK: AtomicPtr<Box<dyn Clock>> = AtomicPtr::new(core::ptr::null_mut());

/// Registers a clock as the global handler for providing the high precision
/// time stamps on a `no_std` target.
pub fn register_clock(clock: impl Clock) {
    let clock: Box<dyn Clock> = Box::new(clock);
    let clock = Box::new(clock);
    // FIXME: This isn't entirely clean as this should really be
    // compare_and_swap, but we can't do that on every platform.
    if !CLOCK.load(atomic::Ordering::SeqCst).is_null() {
        panic!("The clock has already been registered");
    }
    CLOCK.store(Box::into_raw(clock), atomic::Ordering::SeqCst);
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct FFIDateTime {
    year: u16,
    month: u8,
    day: u8,
    hours: u8,
    minutes: u8,
    seconds: u8,
}

/// The local time zone.
pub struct Local;

/// A date and a time of day.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DateTime<T>(PhantomData<T>, FFIDateTime);

/// A time zone.
pub trait TimeZone: Sized {
    fn datetime_from_str(&self, s: &str, fmt: &str) -> Result<DateTime<Self>, ParseError>;
}

/// The UTC time zone.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Utc;

/// Failed to parse a time.
#[derive(Debug)]
pub struct ParseError;

/// A span of time.
#[derive(Add, Sub, Neg, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Duration(i128);

/// A point in time.
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub struct Instant(i128);

impl Instant {
    /// Accesses the current point in time.
    pub fn now() -> Self {
        let clock = CLOCK.load(atomic::Ordering::SeqCst);
        if clock.is_null() {
            panic!("No clock registered");
        }
        let clock = unsafe { &*clock };
        let Duration(t) = clock.now();
        Instant(t)
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        Duration(self.0 - rhs.0)
    }
}

impl Duration {
    /// Creates a duration from the provided amount of nanoseconds.
    pub fn nanoseconds(nanos: i64) -> Self {
        Duration(nanos as _)
    }

    /// Creates a duration from the provided amount of microseconds.
    pub fn microseconds(micros: i64) -> Self {
        Duration(micros as i128 * 1_000)
    }

    /// Returns the total amount of microseconds.
    pub fn num_microseconds(self) -> Option<i128> {
        Some(self.0 / 1_000)
    }

    /// Converts a core::time::Duration to a Duration.
    pub fn from_std(val: core::time::Duration) -> Option<Self> {
        let secs = val.as_secs() as i128;
        let secs_as_nanos = secs * 1_000_000_000;
        Some(Duration(val.subsec_nanos() as i128 + secs_as_nanos))
    }
}

impl TimeZone for Utc {
    fn datetime_from_str(&self, _: &str, _: &str) -> Result<DateTime<Self>, ParseError> {
        Ok(DateTime(PhantomData, Default::default()))
    }
}

impl<T> Add<Duration> for DateTime<T> {
    type Output = DateTime<T>;

    fn add(self, _: Duration) -> DateTime<T> {
        self
    }
}

impl<T> DateTime<T> {
    /// Returns the duration between two date times.
    pub fn signed_duration_since<Tz2: TimeZone>(self, _: DateTime<Tz2>) -> Duration {
        Duration::nanoseconds(0)
    }

    /// Formats the date time as a string.
    pub fn format(&self, _: &str) -> String {
        format!(
            "{:02}/{:02}/{:04} {:02}:{:02}:{:02}",
            self.1.month, self.1.day, self.1.year, self.1.hours, self.1.minutes, self.1.seconds
        )
    }

    /// Formats the date time as a RFC 2822 string.
    pub fn to_rfc2822(&self) -> &'static str {
        "Tue, 1 Jul 2003 10:52:37 +0200"
    }

    /// Formats the date time as a RFC 3339 string.
    pub fn to_rfc3339(&self) -> &'static str {
        "1996-12-19T16:39:57-08:00"
    }

    /// Changes the time zone of a date time (and adjust the time accordingly).
    pub fn with_timezone<Tz2>(&self, _: &Tz2) -> DateTime<Tz2> {
        DateTime(PhantomData, Default::default())
    }
}

/// Returns the current date time.
pub fn utc_now() -> DateTime<Utc> {
    DateTime(PhantomData, Default::default())
}
