use super::{
    Accuracy, DASH, MINUS, SECONDS_PER_HOUR, SECONDS_PER_MINUTE, TimeFormatter, format_padded,
};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

pub struct Inner {
    time: Option<TimeSpan>,
    accuracy: Accuracy,
}

/// The Segment Time Formatter formats a [`TimeSpan`] for them to be shown as
/// Segment Times. This specifically means that the fractional part of the time
/// is always shown and the minutes and hours are only shown when necessary. The
/// default accuracy is to show 2 digits of the fractional part, but this can be
/// configured.
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
    /// The default accuracy that the segment times are formatted with.
    pub const DEFAULT_ACCURACY: Accuracy = Accuracy::Hundredths;

    /// Creates a new Segment Time Formatter that uses hundredths for showing
    /// the fractional part.
    pub const fn new() -> Self {
        SegmentTime {
            accuracy: Self::DEFAULT_ACCURACY,
        }
    }

    /// Creates a new Segment Time Formatter that uses the accuracy provided for
    /// showing the fractional part.
    pub const fn with_accuracy(accuracy: Accuracy) -> Self {
        SegmentTime { accuracy }
    }
}

impl Default for SegmentTime {
    fn default() -> Self {
        Self::new()
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
    fn fmt(&self, f: &mut Formatter) -> Result {
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
                f.write_str(":")?;
                f.write_str(format_padded(seconds))?;
            } else if minutes > 0 {
                f.write_str(buffer.format(minutes))?;
                f.write_str(":")?;
                f.write_str(format_padded(seconds))?;
            } else {
                f.write_str(buffer.format(seconds))?;
            }
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
        let inner = SegmentTime::new().format(Some(time));
        assert_eq!(inner.to_string(), "−2562047788015215:30:08.99");
    }

    #[test]
    fn max() {
        let time = TimeSpan::from(crate::platform::Duration::MAX);
        let inner = SegmentTime::new().format(Some(time));
        assert_eq!(inner.to_string(), "2562047788015215:30:07.99");
    }

    #[test]
    fn zero() {
        let time = TimeSpan::zero();
        let inner = SegmentTime::new().format(Some(time));
        assert_eq!(inner.to_string(), "0.00");
    }

    #[test]
    fn empty() {
        let inner = SegmentTime::new().format(None);
        assert_eq!(inner.to_string(), "—");
    }

    #[test]
    fn slightly_positive() {
        let time = TimeSpan::from_str("0.000000001").unwrap();
        let inner = SegmentTime::new().format(Some(time));
        assert_eq!(inner.to_string(), "0.00");

        assert_eq!(
            SegmentTime::new()
                .format(TimeSpan::from_seconds(0.5))
                .to_string(),
            "0.50"
        );
        assert_eq!(
            SegmentTime::new()
                .format(TimeSpan::from_seconds(1.5))
                .to_string(),
            "1.50"
        );
    }

    #[test]
    fn slightly_negative() {
        let time = TimeSpan::from_str("-0.000000001").unwrap();
        let inner = SegmentTime::new().format(Some(time));
        assert_eq!(inner.to_string(), "−0.00");

        assert_eq!(
            SegmentTime::new()
                .format(TimeSpan::from_seconds(-1.5))
                .to_string(),
            "−1.50"
        );
        assert_eq!(
            SegmentTime::new()
                .format(TimeSpan::from_seconds(-0.5))
                .to_string(),
            "−0.50"
        );
    }

    #[test]
    fn seconds() {
        let time = TimeSpan::from_str("23.1234").unwrap();
        let inner = SegmentTime::new().format(Some(time));
        assert_eq!(inner.to_string(), "23.12");
    }

    #[test]
    fn minutes() {
        let time = TimeSpan::from_str("12:34.987654321").unwrap();
        let inner = SegmentTime::new().format(Some(time));
        assert_eq!(inner.to_string(), "12:34.98");
    }

    #[test]
    fn hours() {
        let time = TimeSpan::from_str("12:34:56.123456789").unwrap();
        let inner = SegmentTime::new().format(Some(time));
        assert_eq!(inner.to_string(), "12:34:56.12");
    }

    #[test]
    fn negative() {
        let time = TimeSpan::from_str("-12:34:56.123456789").unwrap();
        let inner = SegmentTime::new().format(Some(time));
        assert_eq!(inner.to_string(), "−12:34:56.12");
    }

    #[test]
    fn days() {
        let time = TimeSpan::from_str("2148:34:56.123456789").unwrap();
        let inner = SegmentTime::new().format(Some(time));
        assert_eq!(inner.to_string(), "2148:34:56.12");
    }

    #[test]
    fn negative_days() {
        let time = TimeSpan::from_str("-2148:34:56.123456789").unwrap();
        let inner = SegmentTime::new().format(Some(time));
        assert_eq!(inner.to_string(), "−2148:34:56.12");
    }
}
