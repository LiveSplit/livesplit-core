use std::ops::Sub;
use chrono::{DateTime, UTC};
use TimeSpan;

#[derive(Copy, Clone)]
pub struct AtomicDateTime {
    pub time: DateTime<UTC>,
    pub synced_with_atomic_clock: bool,
}

impl AtomicDateTime {
    #[inline]
    pub fn new(time: DateTime<UTC>, synced: bool) -> Self {
        AtomicDateTime {
            time: time,
            synced_with_atomic_clock: synced,
        }
    }

    #[inline]
    pub fn now() -> Self {
        AtomicDateTime {
            time: UTC::now(),
            synced_with_atomic_clock: false,
        }
    }
}

impl Sub for AtomicDateTime {
    type Output = TimeSpan;

    fn sub(self, rhs: AtomicDateTime) -> TimeSpan {
        (self.time - rhs.time).into()
    }
}

impl Sub<DateTime<UTC>> for AtomicDateTime {
    type Output = TimeSpan;

    fn sub(self, rhs: DateTime<UTC>) -> TimeSpan {
        (self.time - rhs).into()
    }
}
