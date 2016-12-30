use std::ops::{Add, Sub};
use std::time::Duration as StdDuration;
use chrono::Duration;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TimeSpan(Duration);

impl TimeSpan {
    pub fn zero() -> Self {
        Default::default()
    }

    pub fn option_op<F, R>(a: Option<TimeSpan>, b: Option<TimeSpan>, f: F) -> Option<R>
        where F: FnOnce(TimeSpan, TimeSpan) -> R
    {
        match (a, b) {
            (Some(a), Some(b)) => Some(f(a, b)),
            _ => None,
        }
    }

    pub fn total_seconds(&self) -> f64 {
        self.0.num_nanoseconds().unwrap() as f64 / 1_000_000_000.0
    }
}

impl Default for TimeSpan {
    fn default() -> Self {
        TimeSpan(Duration::nanoseconds(0))
    }
}

impl From<StdDuration> for TimeSpan {
    fn from(duration: StdDuration) -> Self {
        TimeSpan(Duration::from_std(duration).unwrap())
    }
}

impl From<Duration> for TimeSpan {
    fn from(duration: Duration) -> Self {
        TimeSpan(duration)
    }
}

impl Add for TimeSpan {
    type Output = TimeSpan;

    fn add(self, rhs: TimeSpan) -> TimeSpan {
        TimeSpan(self.0 + rhs.0)
    }
}

impl Sub for TimeSpan {
    type Output = TimeSpan;

    fn sub(self, rhs: TimeSpan) -> TimeSpan {
        TimeSpan(self.0 - rhs.0)
    }
}
