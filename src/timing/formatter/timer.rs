//! The timing module provides a pair of Time Formatters that splits up the
//! visualized time into the main part of the time and the fractional part. This
//! is the Time Formatter pair used by the Timer Component.

use super::{
    format_padded, format_unpadded, Accuracy, DigitsFormat, TimeFormatter, DASH, MINUS,
    SECONDS_PER_HOUR, SECONDS_PER_MINUTE,
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
    pub const fn new() -> Self {
        Time {
            digits_format: DigitsFormat::SingleDigitSeconds,
        }
    }

    /// Creates a new Time Formatter that uses the digits format specified to
    /// determine how many digits to always show. Zeros are prefixed to fill up
    /// the missing digits.
    pub const fn with_digits_format(digits_format: DigitsFormat) -> Self {
        Time { digits_format }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
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
            let (total_seconds, nanoseconds) = time.to_seconds_and_subsec_nanoseconds();
            let total_seconds = if (total_seconds | nanoseconds as i64) < 0 {
                f.write_str(MINUS)?;
                (-total_seconds) as u64
            } else {
                total_seconds as u64
            };
            // These are intentionally not data dependent, such that the CPU can
            // calculate all of them in parallel. On top of that they are
            // integer divisions of known constants, which get turned into
            // multiplies and shifts, which is very fast.
            let seconds = (total_seconds % SECONDS_PER_MINUTE) as u8;
            let minutes = ((total_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u8;
            let hours = total_seconds / SECONDS_PER_HOUR;

            if self.digits_format == DigitsFormat::DoubleDigitHours {
                let mut buffer = itoa::Buffer::new();
                let hours = buffer.format(hours);
                if hours.len() < 2 {
                    f.write_str("0")?;
                }
                f.write_str(hours)?;
                f.write_str(":")?;
                f.write_str(format_padded(minutes))?;
                f.write_str(":")?;
                f.write_str(format_padded(seconds))
            } else if hours > 0 || self.digits_format == DigitsFormat::SingleDigitHours {
                f.write_str(itoa::Buffer::new().format(hours))?;
                f.write_str(":")?;
                f.write_str(format_padded(minutes))?;
                f.write_str(":")?;
                f.write_str(format_padded(seconds))
            } else if self.digits_format == DigitsFormat::DoubleDigitMinutes {
                f.write_str(format_padded(minutes))?;
                f.write_str(":")?;
                f.write_str(format_padded(seconds))
            } else if minutes > 0 || self.digits_format == DigitsFormat::SingleDigitMinutes {
                f.write_str(format_unpadded(minutes))?;
                f.write_str(":")?;
                f.write_str(format_padded(seconds))
            } else if self.digits_format == DigitsFormat::DoubleDigitSeconds {
                f.write_str(format_padded(seconds))
            } else {
                f.write_str(format_unpadded(seconds))
            }
        } else {
            f.write_str(DASH)
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
    pub const fn new() -> Self {
        Fraction {
            accuracy: Accuracy::Hundredths,
        }
    }

    /// Creates a new Time Formatter that uses the accuracy provided for showing
    /// the fractional part.
    pub const fn with_accuracy(accuracy: Accuracy) -> Self {
        Fraction { accuracy }
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Self::new()
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
            let nanoseconds = time.to_duration().subsec_nanoseconds().unsigned_abs();
            self.accuracy.format_nanoseconds(nanoseconds).fmt(f)
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
