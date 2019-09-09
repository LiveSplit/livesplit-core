use crate::platform::prelude::*;
use chrono::{DateTime, Utc};
use core::ops::Sub;
use core::sync::atomic::{self, AtomicPtr};
use core::time::Duration;

/// A clock is a global handler that can be registered for providing the high
/// precision time stamps on a `no_std` target.
pub trait Clock: 'static {
    /// Returns the current point in time as a Duration. This is expected to be
    /// a monotonic high precision time stamp and does not need to represent a
    /// time based on a calendar.
    fn now(&self) -> Duration;

    /// Returns the current point in time as a DateTime. This is expected to
    /// represent the current date and time of day. It does not need to be a
    /// high precision time stamp and is allowed to suddenly change to due
    /// synchronization with a time server. If there's no notion of a calendar
    /// on the system, you may return a dummy value instead.
    fn date_now(&self) -> DateTime<Utc>;
}

static CLOCK: AtomicPtr<Box<dyn Clock>> = AtomicPtr::new(core::ptr::null_mut());

/// Registers a clock as the global handler for providing the high precision
/// time stamps on a `no_std` target.
pub fn register_clock(clock: impl Clock) {
    let clock: Box<dyn Clock> = Box::new(clock);
    let clock = Box::new(clock);
    // FIXME: This isn't entirely clean as this should really be
    // compare_and_swap, but we can't do that on every platform.
    if !CLOCK.load(atomic::Ordering::SeqCst).is_null() {
        panic!("The clock has already been registered");
    }
    CLOCK.store(Box::into_raw(clock), atomic::Ordering::SeqCst);
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub struct Instant(Duration);

impl Instant {
    /// Accesses the current point in time.
    pub fn now() -> Self {
        let clock = CLOCK.load(atomic::Ordering::SeqCst);
        if clock.is_null() {
            panic!("No clock registered");
        }
        let clock = unsafe { &*clock };
        Instant(clock.now())
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.0 - rhs.0
    }
}

pub fn utc_now() -> DateTime<Utc> {
    let clock = CLOCK.load(atomic::Ordering::SeqCst);
    if clock.is_null() {
        panic!("No clock registered");
    }
    let clock = unsafe { &*clock };
    clock.date_now()
}
