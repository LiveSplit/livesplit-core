use super::{Accuracy, TimeFormatter, MINUS};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

pub struct Inner {
    time: Option<TimeSpan>,
    accuracy: Accuracy,
}

/// The Short Time Formatter is suitable for situations where you want a short
/// visualization of a Time Span, but also a somewhat accurate one. This
/// specifically means that the fractional part of the time is always shown and
/// the minutes and hours are only shown when necessary. The default accuracy is
/// to show 2 digits of the fractional part, but this can be configured.
///
/// # Example Formatting
///
/// * Empty Time `0.00`
/// * Seconds `23.12`
/// * Minutes `12:34.98`
/// * Hours `12:34:56.12`
/// * Negative Times `−23.12`
pub struct Short {
    accuracy: Accuracy,
}

impl Short {
    /// Creates a new Short Time Formatter that uses hundredths for showing the
    /// fractional part.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Short Time Formatter that uses the accuracy provided for
    /// showing the fractional part.
    pub fn with_accuracy(accuracy: Accuracy) -> Self {
        Short { accuracy }
    }
}

impl Default for Short {
    fn default() -> Self {
        Short {
            accuracy: Accuracy::Hundredths,
        }
    }
}

impl TimeFormatter<'_> for Short {
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
            match self.accuracy {
                Accuracy::Seconds => write!(f, "0"),
                Accuracy::Tenths => write!(f, "0.0"),
                Accuracy::Hundredths => write!(f, "0.00"),
                Accuracy::Milliseconds => write!(f, "0.000"),
            }
        }
    }
}

#[test]
fn test() {
    let time = "4:20.69".parse::<TimeSpan>().unwrap();
    let formatted = Short::new().format(time).to_string();
    assert!(
        // Modern processors
        formatted == "4:20.69" ||
        // Old x86 processors are apparently less precise
        formatted == "4:20.68"
    );
}
