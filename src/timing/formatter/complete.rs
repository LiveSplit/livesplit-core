use super::{TimeFormatter, ASCII_MINUS, SECONDS_PER_DAY, SECONDS_PER_HOUR, SECONDS_PER_MINUTE};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

pub struct Inner(Option<TimeSpan>);

/// The Complete Time Formatter formats Time Spans in a way that preserves as
/// much information as possible. The hours and minutes are always shown and a
/// fractional part of 9 digits is used. If there's >24h, then a day prefix is
/// attached (with the hours wrapping around to 0): `dd.hh:mm:ss.fffffffff`
///
/// This formatter uses an ASCII minus for negative times and shows a zero time
/// for empty times.
///
/// # Example Formatting
///
/// * Empty Time `00:00:00.000000000`
/// * Seconds `00:00:23.123400000`
/// * Minutes `00:12:34.987654321`
/// * Hours `12:34:56.123456789`
/// * Negative Times `-12:34:56.123456789`
/// * Days `89.12:34:56.123456789`
#[derive(Default)]
pub struct Complete;

impl Complete {
    /// Creates a new Complete Time Formatter.
    pub const fn new() -> Self {
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
            let (total_seconds, nanoseconds) = time.to_seconds_and_subsec_nanoseconds();
            let (total_seconds, nanoseconds) = if total_seconds < 0 {
                // Since, this Formatter is used for writing out split files, we
                // have to use an ASCII Minus here.
                f.write_str(ASCII_MINUS)?;
                ((-total_seconds) as u64, (-nanoseconds) as u32)
            } else {
                (total_seconds as u64, nanoseconds as u32)
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
                write!(f, "{days}.")?;
            }
            write!(f, "{hours:02}:{minutes:02}:{seconds:02}.{nanoseconds:09}")
        } else {
            f.write_str("00:00:00.000000000")
        }
    }
}
