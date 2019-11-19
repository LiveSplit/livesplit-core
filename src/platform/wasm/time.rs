use core::{ops::Sub, time::Duration};
use ordered_float::OrderedFloat;

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

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        let secs = (self.0).0 - (rhs.0).0;
        let nanos = ((secs % 1.0) * 1_000_000_000.0) as _;
        let secs = secs as _;
        Duration::new(secs, nanos)
    }
}
