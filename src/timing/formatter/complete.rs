use super::{TimeFormatter, ASCII_MINUS};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

pub struct Inner(Option<TimeSpan>);

/// The Complete Time Formatter formats Time Spans in a way that preserves as
/// much information as possible. The hours and minutes are always shown and a
/// fractional part of 7 digits is used. If there's >24h, then a day prefix is
/// attached (with the hours wrapping around to 0): `dd.hh:mm:ss.fffffff`
///
/// This formatter uses an ASCII minus for negative times and shows a zero time
/// for empty times.
///
/// # Example Formatting
///
/// * Empty Time `00:00:00.0000000`
/// * Seconds `00:00:23.1234000`
/// * Minutes `00:12:34.9876543`
/// * Hours `12:34:56.1234567`
/// * Negative Times `-12:34:56.1234567`
/// * Days `89.12:34:56.1234567`
#[derive(Default)]
pub struct Complete;

impl Complete {
    /// Creates a new Complete Time Formatter.
    pub fn new() -> Self {
        Complete
    }
}

impl TimeFormatter<'_> for Complete {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        Inner(time.into())
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(time) = self.0 {
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                // Since, this Formatter is used for writing out split files, we
                // have to use an ASCII Minus here.
                write!(f, "{}", ASCII_MINUS)?;
            }
            let seconds = total_seconds % 60.0;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let total_hours = total_minutes / 60;
            let hours = total_hours % 24;
            let days = total_hours / 24;
            if days > 0 {
                write!(f, "{}.", days)?;
            }
            write!(f, "{:02}:{:02}:{:010.7}", hours, minutes, seconds)
        } else {
            write!(f, "00:00:00.0000000")
        }
    }
}
