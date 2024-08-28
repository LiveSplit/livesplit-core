use super::{
    format_padded, TimeFormatter, ASCII_MINUS, SECONDS_PER_DAY, SECONDS_PER_HOUR,
    SECONDS_PER_MINUTE,
};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

pub struct Inner(Option<TimeSpan>);

/// The Complete Time Formatter formats a [`TimeSpan`] in a way that preserves as
/// much information as possible. The hours and minutes are always shown and a
/// fractional part of 9 digits is used. If there's >24h, then a day prefix is
/// attached (with the hours wrapping around to 0): `dd.hh:mm:ss.fffffffff`
///
/// This formatter uses an ASCII minus for negative times and shows a zero time
/// for empty times.
///
/// # Example Formatting
///
/// * Empty Time `00:00:00.000000000`
/// * Seconds `00:00:23.123400000`
/// * Minutes `00:12:34.987654321`
/// * Hours `12:34:56.123456789`
/// * Negative Times `-12:34:56.123456789`
/// * Days `89.12:34:56.123456789`
#[derive(Default)]
pub struct Complete;

impl Complete {
    /// Creates a new Complete Time Formatter.
    pub const fn new() -> Self {
        Complete
    }
}

impl TimeFormatter<'_> for Complete {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        Inner(time.into())
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(time) = self.0 {
            let (total_seconds, nanoseconds) = time.to_seconds_and_subsec_nanoseconds();
            let (total_seconds, nanoseconds) = if (total_seconds | nanoseconds as i64) < 0 {
                // Since, this Formatter is used for writing out split files, we
                // have to use an ASCII Minus here.
                f.write_str(ASCII_MINUS)?;
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
            let hours = ((total_seconds % SECONDS_PER_DAY) / SECONDS_PER_HOUR) as u8;
            let days = total_seconds / SECONDS_PER_DAY;

            let mut buffer = itoa::Buffer::new();

            if days > 0 {
                f.write_str(buffer.format(days))?;
                f.write_str(".")?;
            }

            f.write_str(format_padded(hours))?;
            f.write_str(":")?;
            f.write_str(format_padded(minutes))?;
            f.write_str(":")?;
            f.write_str(format_padded(seconds))?;
            f.write_str(".")?;
            let nanoseconds = buffer.format(nanoseconds);
            f.write_str(&"000000000"[nanoseconds.len()..])?;
            f.write_str(nanoseconds)
        } else {
            f.write_str("00:00:00.000000000")
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
        let inner = Complete.format(Some(time));
        assert_eq!(inner.to_string(), "-106751991167300.15:30:08.999999999");
    }

    #[test]
    fn max() {
        let time = TimeSpan::from(crate::platform::Duration::MAX);
        let inner = Complete.format(Some(time));
        assert_eq!(inner.to_string(), "106751991167300.15:30:07.999999999");
    }

    #[test]
    fn zero() {
        let time = TimeSpan::zero();
        let inner = Complete.format(Some(time));
        assert_eq!(inner.to_string(), "00:00:00.000000000");
    }

    #[test]
    fn empty() {
        let inner = Complete.format(None);
        assert_eq!(inner.to_string(), "00:00:00.000000000");
    }

    #[test]
    fn slightly_positive() {
        let time = TimeSpan::from_str("0.000000001").unwrap();
        let inner = Complete.format(Some(time));
        assert_eq!(inner.to_string(), "00:00:00.000000001");

        assert_eq!(
            Complete.format(TimeSpan::from_seconds(0.5)).to_string(),
            "00:00:00.500000000"
        );
        assert_eq!(
            Complete.format(TimeSpan::from_seconds(1.5)).to_string(),
            "00:00:01.500000000"
        );
    }

    #[test]
    fn slightly_negative() {
        let time = TimeSpan::from_str("-0.000000001").unwrap();
        let inner = Complete.format(Some(time));
        assert_eq!(inner.to_string(), "-00:00:00.000000001");

        assert_eq!(
            Complete.format(TimeSpan::from_seconds(-1.5)).to_string(),
            "-00:00:01.500000000"
        );
        assert_eq!(
            Complete.format(TimeSpan::from_seconds(-0.5)).to_string(),
            "-00:00:00.500000000"
        );
    }

    #[test]
    fn seconds() {
        let time = TimeSpan::from_str("23.1234").unwrap();
        let inner = Complete.format(Some(time));
        assert_eq!(inner.to_string(), "00:00:23.123400000");
    }

    #[test]
    fn minutes() {
        let time = TimeSpan::from_str("12:34.987654321").unwrap();
        let inner = Complete.format(Some(time));
        assert_eq!(inner.to_string(), "00:12:34.987654321");
    }

    #[test]
    fn hours() {
        let time = TimeSpan::from_str("12:34:56.123456789").unwrap();
        let inner = Complete.format(Some(time));
        assert_eq!(inner.to_string(), "12:34:56.123456789");
    }

    #[test]
    fn negative() {
        let time = TimeSpan::from_str("-12:34:56.123456789").unwrap();
        let inner = Complete.format(Some(time));
        assert_eq!(inner.to_string(), "-12:34:56.123456789");
    }

    #[test]
    fn days() {
        let time = TimeSpan::from_str("2148:34:56.123456789").unwrap();
        let inner = Complete.format(Some(time));
        assert_eq!(inner.to_string(), "89.12:34:56.123456789");
    }

    #[test]
    fn negative_days() {
        let time = TimeSpan::from_str("-2148:34:56.123456789").unwrap();
        let inner = Complete.format(Some(time));
        assert_eq!(inner.to_string(), "-89.12:34:56.123456789");
    }
}
