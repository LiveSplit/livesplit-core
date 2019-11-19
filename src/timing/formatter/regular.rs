use super::{Accuracy, TimeFormatter, DASH, MINUS};
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
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new custom Regular Time Formatter where you can specify how
    /// many digits to show for the fractional part.
    pub fn with_accuracy(accuracy: Accuracy) -> Self {
        Regular { accuracy }
    }
}

impl Default for Regular {
    fn default() -> Self {
        Regular {
            accuracy: Accuracy::Seconds,
        }
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
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "{}", MINUS)?;
            }
            let seconds = total_seconds % 60.0;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let hours = total_minutes / 60;
            if hours > 0 {
                write!(
                    f,
                    "{}:{:02}:{}",
                    hours,
                    minutes,
                    self.accuracy.format_seconds(seconds, true)
                )
            } else {
                write!(
                    f,
                    "{}:{}",
                    minutes,
                    self.accuracy.format_seconds(seconds, true)
                )
            }
        } else {
            write!(f, "{}", DASH)
        }
    }
}
