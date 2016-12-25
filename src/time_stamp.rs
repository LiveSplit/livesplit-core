use std::time::Instant;
use std::ops::Sub;
use TimeSpan;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TimeStamp(Instant, TimeSpan);

impl TimeStamp {
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
        TimeStamp(self.0, self.1 + rhs)
    }
}
