use chrono::Duration;
use platform::Duration as StdDuration;
use std::num::ParseFloatError;
use std::ops::{AddAssign, SubAssign};
use std::str::FromStr;

/// A Time Span represents a certain span of time.
#[derive(From, Add, Sub, Neg, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TimeSpan(Duration);

impl TimeSpan {
    /// Creates a new Time Span of zero length.
    pub fn zero() -> Self {
        Default::default()
    }

    /// Creates a new Time Span from a given amount of milliseconds.
    pub fn from_milliseconds(milliseconds: f64) -> Self {
        TimeSpan(Duration::microseconds((milliseconds * 1_000.0) as i64))
    }

    /// Creates a new Time Span from a given amount of seconds.
    pub fn from_seconds(seconds: f64) -> Self {
        TimeSpan(Duration::microseconds((seconds * 1_000_000.0) as i64))
    }

    /// Creates a new Time Span from a given amount of days.
    pub fn from_days(days: f64) -> Self {
        TimeSpan(Duration::microseconds(
            (days * 24.0 * 3600.0 * 1_000_000.0) as i64,
        ))
    }

    /// Converts the Time Span to a Duration from the `chrono` crate.
    pub fn to_duration(&self) -> Duration {
        self.0
    }

    /// Returns the total amount of seconds (including decimals) this Time Span
    /// represents.
    pub fn total_seconds(&self) -> f64 {
        self.0.num_microseconds().unwrap() as f64 / 1_000_000.0
    }

    /// Returns the total amount of milliseconds (including decimals) this Time
    /// Span represents.
    pub fn total_milliseconds(&self) -> f64 {
        self.0.num_microseconds().unwrap() as f64 / 1_000.0
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

quick_error! {
    /// The Error type for Time Spans that couldn't be parsed.
    #[derive(Debug)]
    pub enum ParseError {
        /// Couldn't parse as a floating point number.
        Float(err: ParseFloatError) {
            from()
        }
    }
}

impl FromStr for TimeSpan {
    type Err = ParseError;

    fn from_str(mut text: &str) -> Result<Self, ParseError> {
        let factor = if text.starts_with('-') {
            text = &text[1..];
            -1.0
        } else if text.starts_with('−') {
            text = &text[3..];
            -1.0
        } else {
            1.0
        };

        let mut seconds = 0.0;
        for split in text.split(':') {
            seconds = 60.0 * seconds + split.parse::<f64>()?;
        }

        Ok(TimeSpan::from_seconds(factor * seconds))
    }
}

impl Default for TimeSpan {
    fn default() -> Self {
        TimeSpan(Duration::nanoseconds(0))
    }
}

impl From<StdDuration> for TimeSpan {
    fn from(duration: StdDuration) -> Self {
        TimeSpan(Duration::from_std(duration).unwrap())
    }
}

impl AddAssign for TimeSpan {
    fn add_assign(&mut self, rhs: TimeSpan) {
        self.0 = self.0 + rhs.0;
    }
}

impl SubAssign for TimeSpan {
    fn sub_assign(&mut self, rhs: TimeSpan) {
        self.0 = self.0 - rhs.0;
    }
}
