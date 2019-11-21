use ordered_float::OrderedFloat;
use std::ops::Sub;
use std::time::Duration;
use web_sys::window;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
pub struct Instant(OrderedFloat<f64>);

impl Instant {
    pub fn now() -> Self {
        let seconds = window()
            .and_then(|w| w.performance())
            .expect("Can't measure time without a performance object")
            .now()
            / 1000.0;
        Instant(OrderedFloat(seconds))
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
