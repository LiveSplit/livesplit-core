use crate::{
    platform::{prelude::*, Duration},
    util::ascii_char::AsciiChar,
};
use core::{
    num::ParseIntError,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
    str::FromStr,
};
use snafu::{ensure, OptionExt, ResultExt};

/// A `TimeSpan` represents a certain span of time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(transparent)]
pub struct TimeSpan(Duration);

impl TimeSpan {
    /// Creates a new `TimeSpan` of zero length.
    pub const fn zero() -> Self {
        Self(Duration::ZERO)
    }

    /// Creates a new `TimeSpan` from a given amount of seconds.
    pub fn from_seconds(seconds: f64) -> Self {
        Self(Duration::seconds_f64(seconds))
    }

    /// Creates a new `TimeSpan` from a given amount of milliseconds.
    pub fn from_milliseconds(milliseconds: f64) -> Self {
        Self(Duration::seconds_f64(0.001 * milliseconds))
    }

    /// Converts the `TimeSpan` to a `Duration` from the `time` crate.
    pub const fn to_duration(&self) -> Duration {
        self.0
    }

    /// Returns the underlying raw seconds and the nanoseconds past the last
    /// full second that make up the `TimeSpan`. This is the most lossless
    /// representation of a `TimeSpan`.
    pub const fn to_seconds_and_subsec_nanoseconds(&self) -> (i64, i32) {
        (self.0.whole_seconds(), self.0.subsec_nanoseconds())
    }

    /// Returns the total amount of seconds (including decimals) this `TimeSpan`
    /// represents.
    pub fn total_seconds(&self) -> f64 {
        self.0.as_seconds_f64()
    }

    /// Returns the total amount of milliseconds (including decimals) this
    /// `TimeSpan` represents.
    pub fn total_milliseconds(&self) -> f64 {
        1_000.0 * self.total_seconds()
    }

    /// Parses an optional `TimeSpan` from a given textual representation of the
    /// `TimeSpan`. If the given text consists entirely of whitespace or is
    /// empty, `None` is returned.
    pub fn parse_opt(text: &str) -> Result<Option<TimeSpan>, ParseError> {
        if text.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(text.parse()?))
        }
    }
}

/// The Error type for a `TimeSpan` that couldn't be parsed.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum ParseError {
    /// An empty string is not a valid `TimeSpan`.
    Empty,
    /// The time is too large to be represented.
    Overflow,
    /// The fractional part contains characters that are not digits.
    FractionDigits,
    /// Couldn't parse the fractional part.
    Fraction {
        /// The underlying error.
        source: ParseIntError,
    },
    /// Couldn't parse the seconds, minutes, or hours.
    Time {
        /// The underlying error.
        source: ParseIntError,
    },
}

impl FromStr for TimeSpan {
    type Err = ParseError;

    fn from_str(mut text: &str) -> Result<Self, ParseError> {
        // It's faster to use `strip_prefix` with char literals if it's an ASCII
        // char, otherwise prefer using string literals.
        #[allow(clippy::single_char_pattern)]
        let negate =
            if let Some(remainder) = text.strip_prefix('-').or_else(|| text.strip_prefix("−")) {
                text = remainder;
                true
            } else {
                false
            };

        let (seconds_text, nanos) =
            if let Some((seconds, mut nanos)) = AsciiChar::DOT.split_once(text) {
                if nanos.len() > 9 {
                    nanos = nanos.get(..9).context(FractionDigits)?;
                }
                (
                    seconds,
                    nanos.parse::<u32>().context(Fraction)? * 10_u32.pow(9 - nanos.len() as u32),
                )
            } else {
                (text, 0u32)
            };

        ensure!(!seconds_text.is_empty(), Empty);

        let mut seconds = 0u64;

        for split in AsciiChar::COLON.split_iter(seconds_text) {
            seconds = seconds
                .checked_mul(60)
                .context(Overflow)?
                .checked_add(split.parse::<u64>().context(Time)?)
                .context(Overflow)?;
        }

        let (mut seconds, mut nanos) = (
            i64::try_from(seconds).ok().context(Overflow)?,
            i32::try_from(nanos).ok().context(Overflow)?,
        );

        if negate {
            seconds = -seconds;
            nanos = -nanos;
        }

        Ok(Duration::new(seconds, nanos).into())
    }
}

impl Default for TimeSpan {
    fn default() -> Self {
        TimeSpan(Duration::ZERO)
    }
}

impl From<Duration> for TimeSpan {
    fn from(duration: Duration) -> Self {
        TimeSpan(duration)
    }
}

impl From<TimeSpan> for Duration {
    fn from(time_span: TimeSpan) -> Self {
        time_span.0
    }
}

impl Add for TimeSpan {
    type Output = TimeSpan;
    fn add(self, rhs: TimeSpan) -> TimeSpan {
        TimeSpan(self.0.saturating_add(rhs.0))
    }
}

impl Sub for TimeSpan {
    type Output = TimeSpan;
    fn sub(self, rhs: TimeSpan) -> TimeSpan {
        TimeSpan(self.0.saturating_sub(rhs.0))
    }
}

impl AddAssign for TimeSpan {
    fn add_assign(&mut self, rhs: TimeSpan) {
        *self = *self + rhs;
    }
}

impl SubAssign for TimeSpan {
    fn sub_assign(&mut self, rhs: TimeSpan) {
        *self = *self - rhs;
    }
}

impl Neg for TimeSpan {
    type Output = TimeSpan;
    fn neg(self) -> TimeSpan {
        TimeSpan(-self.0)
    }
}

use core::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor};

impl<'de> Deserialize<'de> for TimeSpan {
    fn deserialize<D>(deserializer: D) -> Result<TimeSpan, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TimeSpanVisitor)
    }
}

struct TimeSpanVisitor;

impl Visitor<'_> for TimeSpanVisitor {
    type Value = TimeSpan;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string containing a time")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        v.parse()
            .map_err(|_| E::custom(format!("Not a valid time string: {v:?}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        TimeSpan::from_str("-12:37:30.12").unwrap();
        TimeSpan::from_str("-37:30.12").unwrap();
        TimeSpan::from_str("-30.12").unwrap();
        TimeSpan::from_str("-10:30").unwrap();
        TimeSpan::from_str("-30").unwrap();
        TimeSpan::from_str("-100").unwrap();
        TimeSpan::from_str("--30").unwrap_err();
        TimeSpan::from_str("-").unwrap_err();
        TimeSpan::from_str("").unwrap_err();
        TimeSpan::from_str("-10:-30").unwrap_err();
        TimeSpan::from_str("10:-30").unwrap_err();
        TimeSpan::from_str("10.5:30.5").unwrap_err();
        TimeSpan::from_str("NaN").unwrap_err();
        TimeSpan::from_str("Inf").unwrap_err();
        assert!(matches!(
            TimeSpan::from_str("1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1:1"),
            Err(ParseError::Overflow),
        ));
        assert_eq!(
            TimeSpan::from_str("10.123456789")
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (10, 123456789)
        );
        assert_eq!(
            TimeSpan::from_str("10.0987654321987654321")
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (10, 98765432)
        );
        assert_eq!(
            TimeSpan::from_str("10.000000000")
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (10, 0)
        );
        assert_eq!(
            TimeSpan::from_str("10")
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (10, 0)
        );
    }
}
