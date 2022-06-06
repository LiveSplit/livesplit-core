//! The timing module provides everything necessary for working with times and
//! measuring them.

mod atomic_date_time;
pub mod formatter;
mod time;
mod time_span;
mod time_stamp;
mod timer;
mod timer_phase;
mod timing_method;

pub use self::atomic_date_time::AtomicDateTime;
pub use self::time::{GameTime, RealTime, Time};
pub use self::time_span::{ParseError, TimeSpan};
pub use self::time_stamp::TimeStamp;
#[cfg(feature = "std")]
pub use self::timer::SharedTimer;
pub use self::timer::{CreationError as TimerCreationError, Snapshot, Timer};
pub use self::timer_phase::TimerPhase;
pub use self::timing_method::TimingMethod;


/// A function used for visual elements that change over time, without any
/// specific connection to an actual timer. This function returns an f64
/// which approximately changes by 1 every millisecond. No guarantees are 
/// made about the initial value of this, so you'll almost definitely want to 
/// modulo this. Currently this is used to animate the rainbows on gold segments.
pub fn visual_cycle_timer() -> f64 {
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TIME: TimeStamp = TimeStamp::now();
    }

    (TimeStamp::now() - *TIME).total_milliseconds()
}