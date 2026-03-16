//! The timing module provides a pair of Time Formatters that splits up the
//! visualized time into the main part of the time and the fractional part. This
//! is the Time Formatter pair used by the Timer Component.

use super::{
    Accuracy, DASH, DigitsFormat, MINUS, SECONDS_PER_HOUR, SECONDS_PER_MINUTE, TIME_SEPARATOR,
    TimeFormatter, format_padded, format_unpadded,
};
use crate::{TimeSpan, localization::Lang, util::ascii_char::AsciiChar};
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

    /// Gets the current DigitsFormat in use by the instance of Time.
    pub const fn get_digits_format(&self) -> DigitsFormat {
        self.digits_format
    }

    /// Sets the current DigitsFormat in use by the instance of Time.
    pub fn set_digits_format(&mut self, digits_format: DigitsFormat) {
        self.digits_format = digits_format;
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeFormatter<'_> for Time {
    type Inner = TimeInner;

    fn format<T>(&self, time: T, _: Lang) -> Self::Inner
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
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.time {
            let (total_seconds, nanoseconds) = time.to_seconds_and_subsec_nanoseconds();
            let total_seconds = if (total_seconds | nanoseconds as i64) < 0 {
                f.write_str(MINUS)?;
                total_seconds.wrapping_neg() as u64
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
                f.write_str(TIME_SEPARATOR)?;
                f.write_str(format_padded(minutes))?;
                f.write_str(TIME_SEPARATOR)?;
                f.write_str(format_padded(seconds))
            } else if hours > 0 || self.digits_format == DigitsFormat::SingleDigitHours {
                f.write_str(itoa::Buffer::new().format(hours))?;
                f.write_str(TIME_SEPARATOR)?;
                f.write_str(format_padded(minutes))?;
                f.write_str(TIME_SEPARATOR)?;
                f.write_str(format_padded(seconds))
            } else if self.digits_format == DigitsFormat::DoubleDigitMinutes {
                f.write_str(format_padded(minutes))?;
                f.write_str(TIME_SEPARATOR)?;
                f.write_str(format_padded(seconds))
            } else if minutes > 0 || self.digits_format == DigitsFormat::SingleDigitMinutes {
                f.write_str(format_unpadded(minutes))?;
                f.write_str(TIME_SEPARATOR)?;
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
    decimal_separator: AsciiChar,
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

    /// Gets the current Accuracy in use by the instance of Time.
    pub const fn get_accuracy(&self) -> Accuracy {
        self.accuracy
    }

    /// Sets the current Accuracy in use by the instance of Time.
    pub fn set_accuracy(&mut self, accuracy: Accuracy) {
        self.accuracy = accuracy;
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeFormatter<'_> for Fraction {
    type Inner = FractionInner;

    fn format<T>(&self, time: T, lang: Lang) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        FractionInner {
            time: time.into(),
            accuracy: self.accuracy,
            decimal_separator: lang.decimal_separator(),
        }
    }
}

impl Display for FractionInner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.time {
            let nanoseconds = time.to_duration().subsec_nanoseconds().unsigned_abs();
            self.accuracy
                .format_nanoseconds(nanoseconds, self.decimal_separator)
                .fmt(f)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min() {
        // This verifies that flipping the sign of the minimum value doesn't
        // cause a panic.
        let time = TimeSpan::from(crate::platform::Duration::MIN);
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "−2562047788015215:30:08");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".99");
    }

    #[test]
    fn max() {
        let time = TimeSpan::from(crate::platform::Duration::MAX);
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "2562047788015215:30:07");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".99");
    }

    #[test]
    fn zero() {
        let time = TimeSpan::zero();
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "0");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".00");
    }

    #[test]
    fn empty() {
        let inner = Time::new().format(None, Lang::English);
        assert_eq!(inner.to_string(), "—");
        let inner = Fraction::new().format(None, Lang::English);
        assert_eq!(inner.to_string(), "");
    }

    #[test]
    fn slightly_positive() {
        let time = TimeSpan::parse("0.000000001", Lang::English).unwrap();
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "0");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".00");

        let time = TimeSpan::from_seconds(0.5);
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "0");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".50");

        let time = TimeSpan::from_seconds(1.5);
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "1");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".50");
    }

    #[test]
    fn slightly_negative() {
        let time = TimeSpan::parse("-0.000000001", Lang::English).unwrap();
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "−0");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".00");

        let time = TimeSpan::from_seconds(-0.5);
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "−0");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".50");

        let time = TimeSpan::from_seconds(-1.5);
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "−1");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".50");
    }

    #[test]
    fn seconds() {
        let time = TimeSpan::parse("23.1234", Lang::English).unwrap();
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "23");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".12");
    }

    #[test]
    fn minutes() {
        let time = TimeSpan::parse("12:34.987654321", Lang::English).unwrap();
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "12:34");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".98");
    }

    #[test]
    fn hours() {
        let time = TimeSpan::parse("12:34:56.123456789", Lang::English).unwrap();
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "12:34:56");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".12");
    }

    #[test]
    fn negative() {
        let time = TimeSpan::parse("-12:34:56.123456789", Lang::English).unwrap();
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "−12:34:56");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".12");
    }

    #[test]
    fn days() {
        let time = TimeSpan::parse("2148:34:56.123456789", Lang::English).unwrap();
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "2148:34:56");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".12");
    }

    #[test]
    fn negative_days() {
        let time = TimeSpan::parse("-2148:34:56.123456789", Lang::English).unwrap();
        let inner = Time::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "−2148:34:56");
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".12");
    }

    #[test]
    fn fractions_default() {
        let time = TimeSpan::parse("0.987654321", Lang::English).unwrap();
        let inner = Fraction::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".98");
    }

    #[test]
    fn fractions_seconds() {
        let time = TimeSpan::parse("0.987654321", Lang::English).unwrap();
        let inner = Fraction::with_accuracy(Accuracy::Seconds).format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "");
    }

    #[test]
    fn fractions_tenths() {
        let time = TimeSpan::parse("0.987654321", Lang::English).unwrap();
        let inner = Fraction::with_accuracy(Accuracy::Tenths).format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".9");
    }

    #[test]
    fn fractions_hundredths() {
        let time = TimeSpan::parse("0.987654321", Lang::English).unwrap();
        let inner = Fraction::with_accuracy(Accuracy::Hundredths).format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".98");
    }

    #[test]
    fn fractions_milliseconds() {
        let time = TimeSpan::parse("0.987654321", Lang::English).unwrap();
        let inner =
            Fraction::with_accuracy(Accuracy::Milliseconds).format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), ".987");
    }
}
