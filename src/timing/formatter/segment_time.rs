use super::{Accuracy, TimeFormatter, DASH, MINUS};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

pub struct Inner {
    time: Option<TimeSpan>,
    accuracy: Accuracy,
}

/// The Segment Time Formatter formats Time Spans for them to be shown as
/// Segment Times. This specifically means that the fractional part of the time
/// is always shown and the minutes and hours are only shown when necessary. The
/// default accuracy is to show 2 digits of the fractional part, but this can be
/// configured. Unlike the Short Time Formatter, the Segment Time Formatter
/// shows a dash when there's an empty time.
///
/// # Example Formatting
///
/// * Empty Time `—`
/// * Seconds `23.12`
/// * Minutes `12:34.98`
/// * Hours `12:34:56.12`
/// * Negative Times `−23.12`
pub struct SegmentTime {
    accuracy: Accuracy,
}

impl SegmentTime {
    /// Creates a new Segment Time Formatter that uses hundredths for showing
    /// the fractional part.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Segment Time Formatter that uses the accuracy provided for
    /// showing the fractional part.
    pub fn with_accuracy(accuracy: Accuracy) -> Self {
        SegmentTime { accuracy }
    }
}

impl Default for SegmentTime {
    fn default() -> Self {
        SegmentTime {
            accuracy: Accuracy::Hundredths,
        }
    }
}

impl TimeFormatter<'_> for SegmentTime {
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
            } else if total_minutes > 0 {
                write!(
                    f,
                    "{}:{}",
                    minutes,
                    self.accuracy.format_seconds(seconds, true)
                )
            } else {
                write!(f, "{}", self.accuracy.format_seconds(seconds, false))
            }
        } else {
            write!(f, "{}", DASH)
        }
    }
}
