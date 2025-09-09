use super::{
    MINUS, SECONDS_PER_DAY, SECONDS_PER_HOUR, SECONDS_PER_MINUTE, TimeFormatter, format_padded,
};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

pub struct Inner {
    time: Option<TimeSpan>,
}

/// The Days Time Formatter formats a [`TimeSpan`] so that times >24h are prefixed
/// with the amount of days, wrapping the hours around to 0. There's no
/// fractional part for times. The minutes are always shown.
///
/// # Example Formatting
///
/// * Empty Time `0:00`
/// * Seconds `0:23`
/// * Minutes `12:34`
/// * Hours `12:34:56`
/// * Negative Times `−12:34:56`
/// * Days `89d 12:34:56`
/// * Negative Days `−89d 12:34:56`
#[derive(Default)]
pub struct Days;

impl Days {
    /// Creates a new Days Time Formatter.
    pub const fn new() -> Self {
        Days
    }
}

impl TimeFormatter<'_> for Days {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        Inner { time: time.into() }
    }
}

impl Display for Inner {
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
            let hours = ((total_seconds % SECONDS_PER_DAY) / SECONDS_PER_HOUR) as u8;
            let days = total_seconds / SECONDS_PER_DAY;

            let mut buffer = itoa::Buffer::new();

            if days > 0 {
                f.write_str(buffer.format(days))?;
                f.write_str("d ")?;
            }

            if days > 0 || hours > 0 {
                f.write_str(buffer.format(hours))?;
                f.write_str(":")?;
                f.write_str(format_padded(minutes))?;
            } else {
                f.write_str(buffer.format(minutes))?;
            }
            f.write_str(":")?;
            f.write_str(format_padded(seconds))
        } else {
            f.write_str("0:00")
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
        let inner = Days.format(Some(time));
        assert_eq!(inner.to_string(), "−106751991167300d 15:30:08");
    }

    #[test]
    fn max() {
        let time = TimeSpan::from(crate::platform::Duration::MAX);
        let inner = Days.format(Some(time));
        assert_eq!(inner.to_string(), "106751991167300d 15:30:07");
    }

    #[test]
    fn zero() {
        let time = TimeSpan::zero();
        let inner = Days.format(Some(time));
        assert_eq!(inner.to_string(), "0:00");
    }

    #[test]
    fn empty() {
        let inner = Days.format(None);
        assert_eq!(inner.to_string(), "0:00");
    }

    #[test]
    fn slightly_positive() {
        let time = TimeSpan::from_str("0.000000001").unwrap();
        let inner = Days.format(Some(time));
        assert_eq!(inner.to_string(), "0:00");

        assert_eq!(Days.format(TimeSpan::from_seconds(0.5)).to_string(), "0:00");
        assert_eq!(Days.format(TimeSpan::from_seconds(1.5)).to_string(), "0:01");
    }

    #[test]
    fn slightly_negative() {
        let time = TimeSpan::from_str("-0.000000001").unwrap();
        let inner = Days.format(Some(time));
        assert_eq!(inner.to_string(), "−0:00");

        assert_eq!(
            Days.format(TimeSpan::from_seconds(-1.5)).to_string(),
            "−0:01"
        );
        assert_eq!(
            Days.format(TimeSpan::from_seconds(-0.5)).to_string(),
            "−0:00"
        );
    }

    #[test]
    fn seconds() {
        let time = TimeSpan::from_str("23.1234").unwrap();
        let inner = Days.format(Some(time));
        assert_eq!(inner.to_string(), "0:23");
    }

    #[test]
    fn minutes() {
        let time = TimeSpan::from_str("12:34.987654321").unwrap();
        let inner = Days.format(Some(time));
        assert_eq!(inner.to_string(), "12:34");
    }

    #[test]
    fn hours() {
        let time = TimeSpan::from_str("12:34:56.123456789").unwrap();
        let inner = Days.format(Some(time));
        assert_eq!(inner.to_string(), "12:34:56");
    }

    #[test]
    fn negative() {
        let time = TimeSpan::from_str("-12:34:56.123456789").unwrap();
        let inner = Days.format(Some(time));
        assert_eq!(inner.to_string(), "−12:34:56");
    }

    #[test]
    fn days() {
        let time = TimeSpan::from_str("2148:34:56.123456789").unwrap();
        let inner = Days.format(Some(time));
        assert_eq!(inner.to_string(), "89d 12:34:56");
    }

    #[test]
    fn negative_days() {
        let time = TimeSpan::from_str("-2148:34:56.123456789").unwrap();
        let inner = Days.format(Some(time));
        assert_eq!(inner.to_string(), "−89d 12:34:56");
    }
}
