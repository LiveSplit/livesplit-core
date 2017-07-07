use std::ops::Sub;
use chrono::{DateTime, Utc};
use TimeSpan;

#[derive(Copy, Clone, Debug)]
pub struct AtomicDateTime {
    pub time: DateTime<Utc>,
    pub synced_with_atomic_clock: bool,
}

impl AtomicDateTime {
    pub fn new(time: DateTime<Utc>, synced_with_atomic_clock: bool) -> Self {
        Self {
            time,
            synced_with_atomic_clock,
        }
    }

    #[inline]
    pub fn now() -> Self {
        AtomicDateTime {
            time: Utc::now(),
            synced_with_atomic_clock: false,
        }
    }

    pub fn option_op<F, R>(a: Option<AtomicDateTime>, b: Option<AtomicDateTime>, f: F) -> Option<R>
    where
        F: FnOnce(AtomicDateTime, AtomicDateTime) -> R,
    {
        match (a, b) {
            (Some(a), Some(b)) => Some(f(a, b)),
            _ => None,
        }
    }
}

impl Sub for AtomicDateTime {
    type Output = TimeSpan;

    fn sub(self, rhs: AtomicDateTime) -> TimeSpan {
        self.time.signed_duration_since(rhs.time).into()
    }
}

impl Sub<DateTime<Utc>> for AtomicDateTime {
    type Output = TimeSpan;

    fn sub(self, rhs: DateTime<Utc>) -> TimeSpan {
        self.time.signed_duration_since(rhs).into()
    }
}
