use core::{mem::MaybeUninit, ops::Sub};
use ordered_float::OrderedFloat;

pub use time::{Duration, OffsetDateTime as DateTime};

#[repr(C)]
struct FFIDateTime {
    secs: i64,
    nsecs: u32,
}

extern "C" {
    fn Date_now(data: *mut FFIDateTime);
}

pub fn utc_now() -> DateTime {
    unsafe {
        let mut date_time = MaybeUninit::uninit();
        Date_now(date_time.as_mut_ptr());
        let date_time = date_time.assume_init();
        DateTime::from_unix_timestamp(date_time.secs).unwrap()
            + Duration::nanoseconds(date_time.nsecs as _)
    }
}

extern "C" {
    fn Instant_now() -> f64;
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub struct Instant(OrderedFloat<f64>);

impl Instant {
    pub fn now() -> Self {
        Instant(OrderedFloat(unsafe { Instant_now() }))
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, rhs: Duration) -> Instant {
        Self(self.0 - rhs.as_seconds_f64())
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        let secs = (self.0).0 - (rhs.0).0;
        let nanos = (secs.fract() * 1_000_000_000.0) as _;
        let secs = secs as _;
        Duration::new(secs, nanos)
    }
}

pub fn to_local(date_time: DateTime) -> DateTime {
    date_time
}
