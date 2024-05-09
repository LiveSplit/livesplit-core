use crate::{
    platform::{Duration, Instant},
    TimeSpan,
};
use core::ops::Sub;

/// A `TimeStamp` stores a point in time that can be used to calculate a
/// [`TimeSpan`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct TimeStamp(Instant);

impl TimeStamp {
    /// Creates a new `TimeStamp`, representing the current point in time.
    #[inline]
    pub fn now() -> Self {
        TimeStamp(Instant::now())
    }
}

impl Sub for TimeStamp {
    type Output = TimeSpan;

    #[inline]
    fn sub(self, rhs: TimeStamp) -> TimeSpan {
        TimeSpan::from(self.0 - rhs.0)
    }
}

impl Sub<TimeSpan> for TimeStamp {
    type Output = TimeStamp;

    #[inline]
    fn sub(self, rhs: TimeSpan) -> TimeStamp {
        TimeStamp(self.0 - Duration::from(rhs))
    }
}
