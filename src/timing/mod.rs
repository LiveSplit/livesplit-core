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
pub use self::timer::{CreationError as TimerCreationError, Timer};
pub use self::timer_phase::TimerPhase;
pub use self::timing_method::TimingMethod;
