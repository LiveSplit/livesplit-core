use crate::{
    platform::{utc_now, DateTime},
    TimeSpan,
};
use core::ops::Sub;

/// An Atomic Date Time represents a UTC [`DateTime`] that tries to be as close to
/// an atomic clock as possible.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AtomicDateTime {
    /// The UTC Date Time represented by this Atomic Date Time.
    pub time: DateTime,
    /// Represents whether the date time is actually properly derived from an
    /// atomic clock. If the synchronization with the atomic clock didn't happen
    /// yet or failed, this is set to `false`.
    pub synced_with_atomic_clock: bool,
}

impl AtomicDateTime {
    /// Creates a new Atomic Date Time from the UTC Date Time and the
    /// information of whether this Date Time is derived from an atomic clock or
    /// the local system that may be out of sync with the atomic clock.
    pub const fn new(time: DateTime, synced_with_atomic_clock: bool) -> Self {
        Self {
            time,
            synced_with_atomic_clock,
        }
    }

    /// Creates a new Atomic Date Time that describes the current moment in
    /// time. If a successful synchronization with an atomic clock occurred,
    /// this value is marked as synchronized. Otherwise the local system's timer
    /// is used.
    ///
    /// # Warning
    ///
    /// livesplit-core doesn't synchronize with any atomic clock yet.
    #[inline]
    pub fn now() -> Self {
        AtomicDateTime {
            time: utc_now(),
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

impl Sub<DateTime> for AtomicDateTime {
    type Output = TimeSpan;

    fn sub(self, rhs: DateTime) -> TimeSpan {
        (self.time - rhs).into()
    }
}
