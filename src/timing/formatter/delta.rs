use super::{
    Accuracy, DASH, MINUS, PLUS, SECONDS_PER_HOUR, SECONDS_PER_MINUTE, TIME_SEPARATOR,
    TimeFormatter, format_padded,
};
use crate::{TimeSpan, localization::Lang, util::ascii_char::AsciiChar};
use core::fmt::{Display, Formatter, Result};

pub struct Inner {
    time: Option<TimeSpan>,
    drop_decimals: bool,
    accuracy: Accuracy,
    decimal_separator: AsciiChar,
}

/// The Delta Time Formatter formats a [`TimeSpan`] as a comparison of two
/// durations, so that it visualizes the difference between both of them.
/// Therefore it always shows whether it is a positive or negative difference,
/// by prepending a plus or minus sign. You can choose how many digits of the
/// fractional part are visualized. Additionally there's an option for removing
/// the fractional part for deltas that are larger than 1 minute.
///
/// # Example Formatting
///
/// * Empty Time `—`
/// * Seconds `+23.1`
/// * Minutes without Decimal Dropping `+12:34.9`
/// * Minutes with Decimal Dropping `+12:34`
/// * Hours without Decimal Dropping `+12:34:56.1`
/// * Hours with Decimal Dropping `+12:34:56`
/// * Negative Times `−23.1`
/// * Exactly zero `0.0`
pub struct Delta(bool, Accuracy);

impl Delta {
    /// Creates a new default Delta Time Formatter that drops the fractional
    /// part and uses tenths when showing the fractional part.
    pub const fn new() -> Self {
        Delta(true, Accuracy::Tenths)
    }

    /// Creates a new custom Delta Time Formatter where you can specify whether
    /// the fractional part should be dropped for deltas that are larger than 1
    /// minute and how many digits to show for the fractional part.
    pub const fn custom(drop_decimals: bool, accuracy: Accuracy) -> Self {
        Delta(drop_decimals, accuracy)
    }

    /// Creates a new Delta Time Formatter that drops the fractional part and
    /// uses tenths when showing the fractional part.
    pub const fn with_decimal_dropping() -> Self {
        Delta(true, Accuracy::Tenths)
    }

    /// Creates a new Delta Time Formatter that does not drop the fractional
    /// part and uses tenths when showing the fractional part.
    pub const fn without_decimal_dropping() -> Self {
        Delta(false, Accuracy::Tenths)
    }
}

impl Default for Delta {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeFormatter<'_> for Delta {
    type Inner = Inner;

    fn format<T>(&self, time: T, lang: Lang) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        Inner {
            time: time.into(),
            drop_decimals: self.0,
            accuracy: self.1,
            decimal_separator: lang.decimal_separator(),
        }
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.time {
            let (total_seconds, nanoseconds) = time.to_seconds_and_subsec_nanoseconds();
            let bit_or = total_seconds | nanoseconds as i64;
            let (total_seconds, nanoseconds) = if bit_or < 0 {
                f.write_str(MINUS)?;
                (total_seconds.wrapping_neg() as u64, (-nanoseconds) as u32)
            } else {
                if bit_or > 0 {
                    f.write_str(PLUS)?;
                }
                (total_seconds as u64, nanoseconds as u32)
            };
            // These are intentionally not data dependent, such that the CPU can
            // calculate all of them in parallel. On top of that they are
            // integer divisions of known constants, which get turned into
            // multiplies and shifts, which is very fast.
            let seconds = (total_seconds % SECONDS_PER_MINUTE) as u8;
            let minutes = ((total_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u8;
            let hours = total_seconds / SECONDS_PER_HOUR;

            let mut buffer = itoa::Buffer::new();

            if hours > 0 {
                f.write_str(buffer.format(hours))?;
                f.write_str(TIME_SEPARATOR)?;
                f.write_str(format_padded(minutes))?;
                f.write_str(TIME_SEPARATOR)?;
                f.write_str(format_padded(seconds))?;
            } else if minutes > 0 {
                f.write_str(buffer.format(minutes))?;
                f.write_str(TIME_SEPARATOR)?;
                f.write_str(format_padded(seconds))?;
            } else {
                f.write_str(buffer.format(seconds))?;
                return self
                    .accuracy
                    .format_nanoseconds(nanoseconds, self.decimal_separator)
                    .fmt(f);
            }
            if !self.drop_decimals {
                self.accuracy
                    .format_nanoseconds(nanoseconds, self.decimal_separator)
                    .fmt(f)
            } else {
                Ok(())
            }
        } else {
            f.write_str(DASH)
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
        let inner = Delta::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "−2562047788015215:30:08");
    }

    #[test]
    fn max() {
        let time = TimeSpan::from(crate::platform::Duration::MAX);
        let inner = Delta::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "+2562047788015215:30:07");
    }

    #[test]
    fn zero() {
        let time = TimeSpan::zero();
        let inner = Delta::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "0.0");
    }

    #[test]
    fn empty() {
        let inner = Delta::new().format(None, Lang::English);
        assert_eq!(inner.to_string(), "—");
    }

    #[test]
    fn slightly_positive() {
        let time = TimeSpan::parse("0.000000001", Lang::English).unwrap();
        let inner = Delta::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "+0.0");

        assert_eq!(
            Delta::new()
                .format(TimeSpan::from_seconds(0.5), Lang::English)
                .to_string(),
            "+0.5"
        );
        assert_eq!(
            Delta::new()
                .format(TimeSpan::from_seconds(1.5), Lang::English)
                .to_string(),
            "+1.5"
        );
    }

    #[test]
    fn slightly_negative() {
        let time = TimeSpan::parse("-0.000000001", Lang::English).unwrap();
        let inner = Delta::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "−0.0");

        assert_eq!(
            Delta::new()
                .format(TimeSpan::from_seconds(-1.5), Lang::English)
                .to_string(),
            "−1.5"
        );
        assert_eq!(
            Delta::new()
                .format(TimeSpan::from_seconds(-0.5), Lang::English)
                .to_string(),
            "−0.5"
        );
    }

    #[test]
    fn seconds() {
        let time = TimeSpan::parse("23.1234", Lang::English).unwrap();
        let inner = Delta::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "+23.1");
    }

    #[test]
    fn minutes_with_decimal_dropping() {
        let time = TimeSpan::parse("12:34.987654321", Lang::English).unwrap();
        let inner = Delta::with_decimal_dropping().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "+12:34");
    }

    #[test]
    fn minutes_without_decimal_dropping() {
        let time = TimeSpan::parse("12:34.987654321", Lang::English).unwrap();
        let inner = Delta::without_decimal_dropping().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "+12:34.9");
    }

    #[test]
    fn hours_with_decimal_dropping() {
        let time = TimeSpan::parse("12:34:56.123456789", Lang::English).unwrap();
        let inner = Delta::with_decimal_dropping().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "+12:34:56");
    }

    #[test]
    fn hours_without_decimal_dropping() {
        let time = TimeSpan::parse("12:34:56.123456789", Lang::English).unwrap();
        let inner = Delta::without_decimal_dropping().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "+12:34:56.1");
    }

    #[test]
    fn negative() {
        let time = TimeSpan::parse("-12:34:56.123456789", Lang::English).unwrap();
        let inner = Delta::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "−12:34:56");
    }

    #[test]
    fn days() {
        let time = TimeSpan::parse("2148:34:56.123456789", Lang::English).unwrap();
        let inner = Delta::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "+2148:34:56");
    }

    #[test]
    fn negative_days() {
        let time = TimeSpan::parse("-2148:34:56.123456789", Lang::English).unwrap();
        let inner = Delta::new().format(Some(time), Lang::English);
        assert_eq!(inner.to_string(), "−2148:34:56");
    }
}
