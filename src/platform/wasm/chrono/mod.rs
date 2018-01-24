use std::marker::PhantomData;
use std::ops::Add;

mod duration;

pub use self::duration::Duration;

pub struct Local;
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DateTime<T>(PhantomData<T>, FFIDateTime);
pub trait TimeZone: Sized {
    fn datetime_from_str(&self, s: &str, fmt: &str) -> Result<DateTime<Self>, ParseError>;
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Utc;
#[derive(Debug)]
pub struct ParseError;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct FFIDateTime {
    year: u16,
    month: u8,
    day: u8,
    hours: u8,
    minutes: u8,
    seconds: u8,
    milliseconds: u16,
}

extern "C" {
    fn Date_now(data: *mut FFIDateTime);
}

use std::mem;

impl Utc {
    pub fn now() -> DateTime<Self> {
        unsafe {
            let mut data: FFIDateTime = mem::uninitialized();
            Date_now(&mut data);
            DateTime(PhantomData, data)
        }
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
    pub fn signed_duration_since<Tz2: TimeZone>(self, _: DateTime<Tz2>) -> Duration {
        Duration::zero()
    }

    pub fn format(&self, _: &str) -> String {
        format!(
            "{:02}/{:02}/{:04} {:02}:{:02}:{:02}",
            self.1.month, self.1.day, self.1.year, self.1.hours, self.1.minutes, self.1.seconds
        )
    }

    pub fn to_rfc2822(&self) -> &'static str {
        "Tue, 1 Jul 2003 10:52:37 +0200"
    }

    pub fn to_rfc3339(&self) -> &'static str {
        "1996-12-19T16:39:57-08:00"
    }

    pub fn with_timezone<Tz2>(&self, _: &Tz2) -> DateTime<Tz2> {
        DateTime(PhantomData, Default::default())
    }
}
