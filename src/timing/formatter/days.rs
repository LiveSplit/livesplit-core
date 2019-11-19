use super::{TimeFormatter, MINUS};
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
    pub fn new() -> Self {
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
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "{}", MINUS)?;
            }
            let total_seconds = total_seconds as u64;
            let seconds = total_seconds % 60;
            let total_minutes = total_seconds / 60;
            let minutes = total_minutes % 60;
            let total_hours = total_minutes / 60;
            let hours = total_hours % 24;
            let days = total_hours / 24;

            if days > 0 {
                write!(f, "{}d ", days)?;
            }

            if total_hours > 0 {
                write!(f, "{}:{:02}:{:02}", hours, minutes, seconds)
            } else {
                write!(f, "{}:{:02}", minutes, seconds)
            }
        } else {
            write!(f, "0:00")
        }
    }
}
