use crate::platform::{prelude::*, Duration};
use core::{
    num::{ParseFloatError, ParseIntError},
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
    str::FromStr,
};
use snafu::{ensure, OptionExt, ResultExt};

/// A Time Span represents a certain span of time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TimeSpan(Duration);

impl TimeSpan {
    /// Creates a new Time Span of zero length.
    pub fn zero() -> Self {
        Default::default()
    }

    /// Creates a new Time Span from a given amount of milliseconds.
    pub fn from_milliseconds(milliseconds: f64) -> Self {
        TimeSpan(Duration::seconds_f64(0.001 * milliseconds))
    }

    /// Creates a new Time Span from a given amount of seconds.
    pub fn from_seconds(seconds: f64) -> Self {
        TimeSpan(Duration::seconds_f64(seconds))
    }

    /// Creates a new Time Span from a given amount of days.
    pub fn from_days(days: f64) -> Self {
        TimeSpan(Duration::seconds_f64(days * (24.0 * 60.0 * 60.0)))
    }

    /// Converts the Time Span to a Duration from the `time` crate.
    pub const fn to_duration(&self) -> Duration {
        self.0
    }

    /// Returns the total amount of seconds (including decimals) this Time Span
    /// represents.
    pub fn total_seconds(&self) -> f64 {
        self.0.as_seconds_f64()
    }

    /// Returns the total amount of milliseconds (including decimals) this Time
    /// Span represents.
    pub fn total_milliseconds(&self) -> f64 {
        1_000.0 * self.total_seconds()
    }

    /// Parses an optional Time Span from a given textual representation of the
    /// Time Span. If the given text consists entirely of whitespace or is
    /// empty, `None` is returned.
    pub fn parse_opt<S>(text: S) -> Result<Option<TimeSpan>, ParseError>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();
        if text.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(text.parse()?))
        }
    }
}

/// The Error type for Time Spans that couldn't be parsed.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum ParseError {
    /// An empty string is not a valid Time Span.
    Empty,
    /// The seconds need to be a finite number.
    Finite,
    /// Couldn't parse the seconds.
    Seconds {
        /// The underlying error.
        source: ParseFloatError,
    },
    /// Couldn't parse the minutes or hours.
    MinutesOrHours {
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
        let factor =
            if let Some(remainder) = text.strip_prefix('-').or_else(|| text.strip_prefix("−")) {
                text = remainder;
                -1.0
            } else {
                1.0
            };

        let mut pieces = text.split(':');

        let last = pieces.next_back().context(Empty)?;
        let seconds = last.parse::<f64>().context(Seconds)?;

        ensure!(seconds.is_finite() && seconds >= 0.0, Finite);

        let mut minutes = 0.0;
        for split in pieces {
            minutes = minutes * 60.0 + split.parse::<u32>().context(MinutesOrHours)? as f64;
        }

        Ok(TimeSpan::from_seconds(factor * (seconds + minutes * 60.0)))
    }
}

impl Default for TimeSpan {
    fn default() -> Self {
        TimeSpan(Duration::nanoseconds(0))
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
        TimeSpan(self.0 + rhs.0)
    }
}

impl Sub for TimeSpan {
    type Output = TimeSpan;
    fn sub(self, rhs: TimeSpan) -> TimeSpan {
        TimeSpan(self.0 - rhs.0)
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
        "-12:37:30.12".parse::<TimeSpan>().unwrap();
        "-37:30.12".parse::<TimeSpan>().unwrap();
        "-30.12".parse::<TimeSpan>().unwrap();
        "-10:30".parse::<TimeSpan>().unwrap();
        "-30".parse::<TimeSpan>().unwrap();
        "-100".parse::<TimeSpan>().unwrap();
        "--30".parse::<TimeSpan>().unwrap_err();
        "-".parse::<TimeSpan>().unwrap_err();
        "".parse::<TimeSpan>().unwrap_err();
        "-10:-30".parse::<TimeSpan>().unwrap_err();
        "10:-30".parse::<TimeSpan>().unwrap_err();
        "10.5:30.5".parse::<TimeSpan>().unwrap_err();
        "NaN".parse::<TimeSpan>().unwrap_err();
        "Inf".parse::<TimeSpan>().unwrap_err();
    }
}
