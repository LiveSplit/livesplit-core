use super::{Accuracy, TimeFormatter, DASH, MINUS, SECONDS_PER_HOUR, SECONDS_PER_MINUTE};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

pub struct Inner {
    time: Option<TimeSpan>,
    accuracy: Accuracy,
}

/// The Regular Time Formatter formats Time Spans to always show the minutes and
/// is configurable by how many digits of the fractional part are shown. By
/// default no fractional part is shown. This Time Formatter is most suitable
/// for visualizing split times.
///
/// # Example Formatting
///
/// * Empty Time `—`
/// * Seconds `0:23`
/// * Minutes `12:34`
/// * Hours `12:34:56`
/// * Seconds with Hundredths `0:23.12`
/// * Minutes with Hundredths `12:34.98`
/// * Hours with Hundredths `12:34:56.12`
/// * Negative Times `−0:23`
pub struct Regular {
    accuracy: Accuracy,
}

impl Regular {
    /// Creates a new default Regular Time Formatter that doesn't show a
    /// fractional part.
    pub const fn new() -> Self {
        Regular {
            accuracy: Accuracy::Seconds,
        }
    }

    /// Creates a new custom Regular Time Formatter where you can specify how
    /// many digits to show for the fractional part.
    pub const fn with_accuracy(accuracy: Accuracy) -> Self {
        Regular { accuracy }
    }
}

impl Default for Regular {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeFormatter<'_> for Regular {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        Inner {
            time: time.into(),
            accuracy: self.accuracy,
        }
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(time) = self.time {
            let (total_seconds, nanoseconds) = time.to_seconds_and_subsec_nanoseconds();
            let (total_seconds, nanoseconds) = if total_seconds < 0 {
                f.write_str(MINUS)?;
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
            let hours = total_seconds / SECONDS_PER_HOUR;
            if hours > 0 {
                write!(
                    f,
                    "{hours}:{minutes:02}:{seconds:02}{}",
                    self.accuracy.format_nanoseconds(nanoseconds)
                )
            } else {
                write!(
                    f,
                    "{minutes}:{seconds:02}{}",
                    self.accuracy.format_nanoseconds(nanoseconds)
                )
            }
        } else {
            f.write_str(DASH)
        }
    }
}
