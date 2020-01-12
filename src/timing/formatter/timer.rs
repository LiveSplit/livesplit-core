//! The timing module provides a pair of Time Formatters that splits up the
//! visualized time into the main part of the time and the fractional part. This
//! is the Time Formatter pair used by the Timer Component.

use super::{
    extract_hundredths, extract_milliseconds, extract_tenths, Accuracy, DigitsFormat,
    TimeFormatter, DASH, MINUS,
};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

/// A Time Span to be formatted as the main part of the Time Formatter Pair.
pub struct TimeInner {
    time: Option<TimeSpan>,
    digits_format: DigitsFormat,
}

/// The Time Formatter that visualizes the main part of the Time Formatter Pair
/// for the Timer Component. This Time Formatter shows no fractional part and
/// prefixes as many zeros as you want. By default no zeros are used as a prefix.
///
/// # Example Formatting
///
/// * Empty Time `—`
/// * Seconds `23`
/// * Minutes `12:34`
/// * Hours `12:34:56`
/// * Negative Times `−23`
pub struct Time {
    digits_format: DigitsFormat,
}

impl Time {
    /// Creates a new default Time Formatter that doesn't prefix any zeros.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Time Formatter that uses the digits format specified to
    /// determine how many digits to always show. Zeros are prefixed to fill up
    /// the missing digits.
    pub fn with_digits_format(digits_format: DigitsFormat) -> Self {
        Time { digits_format }
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            digits_format: DigitsFormat::SingleDigitSeconds,
        }
    }
}

impl TimeFormatter<'_> for Time {
    type Inner = TimeInner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        TimeInner {
            time: time.into(),
            digits_format: self.digits_format,
        }
    }
}

impl Display for TimeInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(time) = self.time {
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "{}", MINUS)?;
            }
            let seconds = (total_seconds % 60.0) as u8;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let hours = total_minutes / 60;
            if self.digits_format == DigitsFormat::DoubleDigitHours {
                write!(f, "{:02}:{:02}:{:02}", hours, minutes, seconds)
            } else if hours > 0 || self.digits_format == DigitsFormat::SingleDigitHours {
                write!(f, "{}:{:02}:{:02}", hours, minutes, seconds)
            } else if self.digits_format == DigitsFormat::DoubleDigitMinutes {
                write!(f, "{:02}:{:02}", minutes, seconds)
            } else if total_minutes > 0 || self.digits_format == DigitsFormat::SingleDigitMinutes {
                write!(f, "{}:{:02}", minutes, seconds)
            } else if self.digits_format == DigitsFormat::DoubleDigitSeconds {
                write!(f, "{:02}", seconds)
            } else {
                write!(f, "{}", seconds)
            }
        } else {
            write!(f, "{}", DASH)
        }
    }
}

/// A Time Span to be formatted as the fractional part of the Time Formatter
/// Pair.
pub struct FractionInner {
    time: Option<TimeSpan>,
    accuracy: Accuracy,
}

/// The Time Formatter that visualizes the fractional part of the Time Formatter
/// Pair for the Timer Component. This Time Formatter shows only the fractional
/// part of the time and uses as many digits for it as you want. By default 2
/// digits of the fractional part are shown.
///
/// # Example Formatting
///
/// * Empty Time ``
/// * No Fractional Part `​`
/// * Tenths `.1`
/// * Hundredths `.12`
pub struct Fraction {
    accuracy: Accuracy,
}

impl Fraction {
    /// Creates a new default Time Formatter that uses hundredths for showing
    /// the fractional part.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new Time Formatter that uses the accuracy provided for showing
    /// the fractional part.
    pub fn with_accuracy(accuracy: Accuracy) -> Self {
        Fraction { accuracy }
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Fraction {
            accuracy: Accuracy::Hundredths,
        }
    }
}

impl TimeFormatter<'_> for Fraction {
    type Inner = FractionInner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        FractionInner {
            time: time.into(),
            accuracy: self.accuracy,
        }
    }
}

impl Display for FractionInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(time) = self.time {
            match self.accuracy {
                Accuracy::Seconds => Ok(()),
                Accuracy::Tenths => write!(f, ".{}", extract_tenths(time.total_seconds())),
                Accuracy::Hundredths => {
                    write!(f, ".{:02}", extract_hundredths(time.total_seconds()))
                }
                Accuracy::Milliseconds => {
                    write!(f, ".{:03}", extract_milliseconds(time.total_seconds()))
                }
            }
        } else {
            Ok(())
        }
    }
}

#[test]
fn test() {
    let time = "4:20.999999".parse::<TimeSpan>().unwrap();
    assert_eq!(Fraction::new().format(Some(time)).to_string(), ".99");
}
