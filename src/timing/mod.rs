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

#[cfg(feature = "std")]
pub use self::timer::SharedTimer;
pub use self::{
    atomic_date_time::AtomicDateTime,
    time::{GameTime, RealTime, Time},
    time_span::{ParseError, TimeSpan},
    time_stamp::TimeStamp,
    timer::{CreationError as TimerCreationError, Snapshot, Timer},
    timer_phase::TimerPhase,
    timing_method::TimingMethod,
};
