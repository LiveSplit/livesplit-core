use core::{mem::MaybeUninit, ops::Sub};

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
pub struct Instant(Duration);

impl Instant {
    pub fn now() -> Self {
        let secs = unsafe { Instant_now() };
        let nanos = (secs.fract() * 1_000_000_000.0) as _;
        let secs = secs as _;
        Instant(Duration::new(secs, nanos))
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

pub fn to_local(date_time: DateTime) -> DateTime {
    date_time
}
