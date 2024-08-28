use super::{
    format_padded, Accuracy, TimeFormatter, DASH, MINUS, SECONDS_PER_HOUR, SECONDS_PER_MINUTE,
};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

pub struct Inner {
    time: Option<TimeSpan>,
    accuracy: Accuracy,
}

/// The Regular Time Formatter formats a [`TimeSpan`] to always show the minutes and
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
            let (total_seconds, nanoseconds) = if (total_seconds | nanoseconds as i64) < 0 {
                f.write_str(MINUS)?;
                (total_seconds.wrapping_neg() as u64, (-nanoseconds) as u32)
            } else {
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
                f.write_str(":")?;
                f.write_str(format_padded(minutes))?;
            } else {
                f.write_str(buffer.format(minutes))?;
            }
            f.write_str(":")?;
            f.write_str(format_padded(seconds))?;
            self.accuracy.format_nanoseconds(nanoseconds).fmt(f)
        } else {
            f.write_str(DASH)
        }
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;

    use super::*;

    #[test]
    fn min() {
        // This verifies that flipping the sign of the minimum value doesn't
        // cause a panic.
        let time = TimeSpan::from(crate::platform::Duration::MIN);
        let inner = Regular::new().format(Some(time));
        assert_eq!(inner.to_string(), "−2562047788015215:30:08");
    }

    #[test]
    fn max() {
        let time = TimeSpan::from(crate::platform::Duration::MAX);
        let inner = Regular::new().format(Some(time));
        assert_eq!(inner.to_string(), "2562047788015215:30:07");
    }

    #[test]
    fn zero() {
        let time = TimeSpan::zero();
        let inner = Regular::new().format(Some(time));
        assert_eq!(inner.to_string(), "0:00");
    }

    #[test]
    fn empty() {
        let inner = Regular::new().format(None);
        assert_eq!(inner.to_string(), "—");
    }

    #[test]
    fn slightly_positive() {
        let time = TimeSpan::from_str("0.000000001").unwrap();
        let inner = Regular::new().format(Some(time));
        assert_eq!(inner.to_string(), "0:00");

        assert_eq!(
            Regular::new()
                .format(TimeSpan::from_seconds(0.5))
                .to_string(),
            "0:00"
        );
        assert_eq!(
            Regular::new()
                .format(TimeSpan::from_seconds(1.5))
                .to_string(),
            "0:01"
        );
    }

    #[test]
    fn slightly_negative() {
        let time = TimeSpan::from_str("-0.000000001").unwrap();
        let inner = Regular::new().format(Some(time));
        assert_eq!(inner.to_string(), "−0:00");

        assert_eq!(
            Regular::new()
                .format(TimeSpan::from_seconds(-1.5))
                .to_string(),
            "−0:01"
        );
        assert_eq!(
            Regular::new()
                .format(TimeSpan::from_seconds(-0.5))
                .to_string(),
            "−0:00"
        );
    }

    #[test]
    fn seconds() {
        let time = TimeSpan::from_str("23.1234").unwrap();
        let inner = Regular::new().format(Some(time));
        assert_eq!(inner.to_string(), "0:23");
    }

    #[test]
    fn minutes() {
        let time = TimeSpan::from_str("12:34.987654321").unwrap();
        let inner = Regular::new().format(Some(time));
        assert_eq!(inner.to_string(), "12:34");
    }

    #[test]
    fn hours() {
        let time = TimeSpan::from_str("12:34:56.123456789").unwrap();
        let inner = Regular::new().format(Some(time));
        assert_eq!(inner.to_string(), "12:34:56");
    }

    #[test]
    fn seconds_with_hundredths() {
        let time = TimeSpan::from_str("23.1234").unwrap();
        let inner = Regular::with_accuracy(Accuracy::Hundredths).format(Some(time));
        assert_eq!(inner.to_string(), "0:23.12");
    }

    #[test]
    fn minutes_with_hundredths() {
        let time = TimeSpan::from_str("12:34.987654321").unwrap();
        let inner = Regular::with_accuracy(Accuracy::Hundredths).format(Some(time));
        assert_eq!(inner.to_string(), "12:34.98");
    }

    #[test]
    fn hours_with_hundredths() {
        let time = TimeSpan::from_str("12:34:56.123456789").unwrap();
        let inner = Regular::with_accuracy(Accuracy::Hundredths).format(Some(time));
        assert_eq!(inner.to_string(), "12:34:56.12");
    }

    #[test]
    fn negative() {
        let time = TimeSpan::from_str("-12:34:56.123456789").unwrap();
        let inner = Regular::new().format(Some(time));
        assert_eq!(inner.to_string(), "−12:34:56");
    }

    #[test]
    fn days() {
        let time = TimeSpan::from_str("2148:34:56.123456789").unwrap();
        let inner = Regular::new().format(Some(time));
        assert_eq!(inner.to_string(), "2148:34:56");
    }

    #[test]
    fn negative_days() {
        let time = TimeSpan::from_str("-2148:34:56.123456789").unwrap();
        let inner = Regular::new().format(Some(time));
        assert_eq!(inner.to_string(), "−2148:34:56");
    }
}
