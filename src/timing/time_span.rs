use crate::{
    localization::Lang,
    platform::{Duration, prelude::*},
    util::ascii_char::AsciiChar,
};
use core::{
    num::ParseIntError,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};
use snafu::{OptionExt, ResultExt};

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
    pub const fn from_seconds(seconds: f64) -> Self {
        Self(Duration::seconds_f64(seconds))
    }

    /// Creates a new `TimeSpan` from a given amount of milliseconds.
    pub const fn from_milliseconds(milliseconds: f64) -> Self {
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
    pub const fn total_seconds(&self) -> f64 {
        self.0.as_seconds_f64()
    }

    /// Returns the total amount of milliseconds (including decimals) this
    /// `TimeSpan` represents.
    pub const fn total_milliseconds(&self) -> f64 {
        1_000.0 * self.total_seconds()
    }

    /// Parses an optional `TimeSpan` from a given textual representation of the
    /// `TimeSpan` using the specified language. If the given text consists
    /// entirely of whitespace or is empty, `None` is returned.
    pub fn parse_opt_with_lang(text: &str, lang: Lang) -> Result<Option<TimeSpan>, ParseError> {
        if text.trim().is_empty() {
            Ok(None)
        } else {
            Ok(Some(TimeSpan::parse(text, lang)?))
        }
    }

    /// Parses a `TimeSpan` from a given textual representation using the
    /// specified language.
    pub fn parse(text: &str, lang: Lang) -> Result<TimeSpan, ParseError> {
        parse_custom::<DefaultParser>(text, lang)
    }
}

/// The Error type for a `TimeSpan` that couldn't be parsed.
#[derive(Debug, snafu::Snafu, PartialEq, Eq)]
#[snafu(context(suffix(false)))]
pub enum ParseError {
    /// There are too many colons in the string.
    TooManyColons,
    /// A piece separate by a colon is too large.
    PieceOverflow,
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
    Piece {
        /// The underlying error.
        source: ParseIntError,
    },
}

pub(crate) trait CustomParser {
    const ASCII_ONLY: bool = false;
    const ALLOW_NEGATIVE: bool = true;
    const WITH_DAYS: bool = false;
}

struct DefaultParser;

impl CustomParser for DefaultParser {}

pub(crate) fn parse_custom<T: CustomParser>(
    text: &str,
    lang: Lang,
) -> Result<TimeSpan, ParseError> {
    let separator = lang.decimal_separator();
    parse_custom_with_decimal::<T>(text, separator)
}

fn parse_custom_with_decimal<T: CustomParser>(
    mut text: &str,
    decimal_separator: AsciiChar,
) -> Result<TimeSpan, ParseError> {
    // It's faster to use `strip_prefix` with char literals if it's an ASCII
    // char, otherwise prefer using string literals.
    let negate = if T::ALLOW_NEGATIVE {
        if let Some(remainder) = text.strip_prefix('-').or_else(|| {
            if T::ASCII_ONLY {
                None
            } else {
                text.strip_prefix("−")
            }
        }) {
            text = remainder;
            true
        } else {
            false
        }
    } else {
        false
    };

    let (mut rem, nanos) = if let Some((seconds, mut nanos)) = decimal_separator.split_once(text) {
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

    let mut seconds = 0u64;

    let mut factor = 1;
    let max_pieces = const { if T::WITH_DAYS { 4 } else { 3 } };
    for i in 0.. {
        if i == max_pieces {
            return Err(ParseError::TooManyColons);
        }
        match AsciiChar::COLON.rsplit_once(rem) {
            Some((new_rem, piece)) => {
                rem = new_rem;
                let piece = piece.parse::<u64>().context(Piece)?;
                let max = if i < 3 { 60 } else { 24 };
                if piece >= max {
                    return Err(ParseError::PieceOverflow);
                }
                seconds = piece
                    .checked_mul(factor)
                    .context(Overflow)?
                    .checked_add(seconds)
                    .context(Overflow)?;
                factor *= max;
            }
            None => {
                let piece = rem.parse::<u64>().context(Piece)?;
                seconds = piece
                    .checked_mul(factor)
                    .context(Overflow)?
                    .checked_add(seconds)
                    .context(Overflow)?;
                break;
            }
        }
    }

    let (mut seconds, mut nanos) = (
        i64::try_from(seconds).ok().context(Overflow)?,
        i32::try_from(nanos).ok().context(Overflow)?,
    );

    if T::ALLOW_NEGATIVE && negate {
        seconds = -seconds;
        nanos = -nanos;
    }

    Ok(Duration::new(seconds, nanos).into())
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

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing a time")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        TimeSpan::parse(v, Lang::English)
            .map_err(|_| E::custom(format!("Not a valid time string: {v:?}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        TimeSpan::parse("-12:37:30.12", Lang::English).unwrap();
        TimeSpan::parse("-37:30.12", Lang::English).unwrap();
        TimeSpan::parse("-30.12", Lang::English).unwrap();
        TimeSpan::parse("-10:30", Lang::English).unwrap();
        TimeSpan::parse("-30", Lang::English).unwrap();
        TimeSpan::parse("-100", Lang::English).unwrap();
        TimeSpan::parse("--30", Lang::English).unwrap_err();
        TimeSpan::parse("-", Lang::English).unwrap_err();
        TimeSpan::parse("-10:-30", Lang::English).unwrap_err();
        TimeSpan::parse("10:-30", Lang::English).unwrap_err();
        TimeSpan::parse("10.5:30.5", Lang::English).unwrap_err();
        TimeSpan::parse("NaN", Lang::English).unwrap_err();
        TimeSpan::parse("Inf", Lang::English).unwrap_err();
        assert!(matches!(
            TimeSpan::parse("", Lang::English),
            Err(ParseError::Piece {
                source: ParseIntError { .. }
            })
        ));
        assert_eq!(
            TimeSpan::parse("60", Lang::English)
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (60, 0)
        );
        assert_eq!(
            TimeSpan::parse("0:60", Lang::English),
            Err(ParseError::PieceOverflow)
        );
        assert_eq!(
            TimeSpan::parse("60:00", Lang::English)
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (3600, 0)
        );
        assert_eq!(
            TimeSpan::parse("0:60:00", Lang::English),
            Err(ParseError::PieceOverflow)
        );
        assert_eq!(
            TimeSpan::parse("24:00:00", Lang::English)
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (86400, 0)
        );
        assert_eq!(
            TimeSpan::parse("0:24:00:00", Lang::English),
            Err(ParseError::TooManyColons),
        );
        assert_eq!(
            TimeSpan::parse("10.123456789", Lang::English)
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (10, 123456789)
        );
        assert_eq!(
            TimeSpan::parse("10.0987654321987654321", Lang::English)
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (10, 98765432)
        );
        assert_eq!(
            TimeSpan::parse("10.000000000", Lang::English)
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (10, 0)
        );
        assert_eq!(
            TimeSpan::parse("10", Lang::English)
                .unwrap()
                .to_seconds_and_subsec_nanoseconds(),
            (10, 0)
        );
    }
}
