use super::{TimeFormatter, MINUS, SECONDS_PER_DAY, SECONDS_PER_HOUR, SECONDS_PER_MINUTE};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

pub struct Inner {
    time: Option<TimeSpan>,
}

/// The Days Time Formatter formats Time Spans so that times >24h are prefixed
/// with the amount of days, wrapping the hours around to 0. There's no
/// fractional part for times. The minutes are always shown.
///
/// # Example Formatting
///
/// * Empty Time `0:00`
/// * Seconds `0:23`
/// * Minutes `12:34`
/// * Hours `12:34:56`
/// * Negative Times `−12:34:56`
/// * Days `89d 12:34:56`
/// * Negative Days `−89d 12:34:56`
#[derive(Default)]
pub struct Days;

impl Days {
    /// Creates a new Days Time Formatter.
    pub const fn new() -> Self {
        Days
    }
}

impl TimeFormatter<'_> for Days {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        Inner { time: time.into() }
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(time) = self.time {
            let total_seconds = time.to_duration().whole_seconds();
            let total_seconds = if total_seconds < 0 {
                f.write_str(MINUS)?;
                (-total_seconds) as u64
            } else {
                total_seconds as u64
            };
            // These are intentionally not data dependent, such that the CPU can
            // calculate all of them in parallel. On top of that they are
            // integer divisions of known constants, which get turned into
            // multiplies and shifts, which is very fast.
            let seconds = total_seconds % SECONDS_PER_MINUTE;
            let minutes = (total_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
            let hours = (total_seconds % SECONDS_PER_DAY) / SECONDS_PER_HOUR;
            let days = total_seconds / SECONDS_PER_DAY;

            if days > 0 {
                write!(f, "{days}d ")?;
            }

            if days > 0 || hours > 0 {
                write!(f, "{hours}:{minutes:02}:{seconds:02}")
            } else {
                write!(f, "{minutes}:{seconds:02}")
            }
        } else {
            f.write_str("0:00")
        }
    }
}
