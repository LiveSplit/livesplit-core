use crate::platform::Instant;
use crate::TimeSpan;
use core::ops::Sub;

/// A Time Stamp stores a point in time, that can be used to calculate Time
/// Spans.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TimeStamp(Instant, TimeSpan);

impl TimeStamp {
    /// Creates a new Time Stamp, representing the current point in time.
    pub fn now() -> Self {
        TimeStamp(Instant::now(), TimeSpan::zero())
    }
}

impl Sub for TimeStamp {
    type Output = TimeSpan;

    fn sub(self, rhs: TimeStamp) -> TimeSpan {
        TimeSpan::from(self.0 - rhs.0) + self.1 - rhs.1
    }
}

impl Sub<TimeSpan> for TimeStamp {
    type Output = TimeStamp;

    fn sub(self, rhs: TimeSpan) -> TimeStamp {
        TimeStamp(self.0, self.1 - rhs)
    }
}
