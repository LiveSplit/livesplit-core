use std::ops::{Add, Sub, AddAssign, SubAssign};
use std::str::FromStr;
use std::time::Duration as StdDuration;
use std::num::ParseFloatError;
use std::fmt::{self, Display};
use chrono::Duration;
use time_formatter::{Complete, TimeFormatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TimeSpan(Duration);

impl TimeSpan {
    pub fn zero() -> Self {
        Default::default()
    }

    pub fn from_milliseconds(milliseconds: f64) -> Self {
        TimeSpan(Duration::microseconds((milliseconds * 1_000.0) as i64))
    }

    pub fn from_seconds(seconds: f64) -> Self {
        TimeSpan(Duration::microseconds((seconds * 1_000_000.0) as i64))
    }

    pub fn from_days(days: f64) -> Self {
        TimeSpan(Duration::microseconds((days * 24.0 * 3600.0 * 1_000_000.0) as i64))
    }

    pub fn option_op<F, R>(a: Option<TimeSpan>, b: Option<TimeSpan>, f: F) -> Option<R>
        where F: FnOnce(TimeSpan, TimeSpan) -> R
    {
        match (a, b) {
            (Some(a), Some(b)) => Some(f(a, b)),
            _ => None,
        }
    }

    pub fn to_duration(&self) -> Duration {
        self.0
    }

    pub fn total_seconds(&self) -> f64 {
        self.0.num_microseconds().unwrap() as f64 / 1_000_000.0
    }

    pub fn parse_opt(text: &str) -> Result<Option<TimeSpan>, ParseError> {
        if text.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(text.parse()?))
        }
    }
}

impl Display for TimeSpan {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", Complete.format(*self))
    }
}

#[derive(Debug)]
pub struct ParseError(ParseFloatError);

impl From<ParseFloatError> for ParseError {
    fn from(e: ParseFloatError) -> Self {
        ParseError(e)
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

impl From<Duration> for TimeSpan {
    fn from(duration: Duration) -> Self {
        TimeSpan(duration)
    }
}

impl Add for TimeSpan {
    type Output = TimeSpan;

    fn add(self, rhs: TimeSpan) -> TimeSpan {
        TimeSpan(self.0 + rhs.0)
    }
}

impl AddAssign for TimeSpan {
    fn add_assign(&mut self, rhs: TimeSpan) {
        self.0 = self.0 + rhs.0;
    }
}

impl Sub for TimeSpan {
    type Output = TimeSpan;

    fn sub(self, rhs: TimeSpan) -> TimeSpan {
        TimeSpan(self.0 - rhs.0)
    }
}

impl SubAssign for TimeSpan {
    fn sub_assign(&mut self, rhs: TimeSpan) {
        self.0 = self.0 - rhs.0;
    }
}
