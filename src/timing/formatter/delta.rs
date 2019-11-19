use super::{Accuracy, TimeFormatter, DASH, MINUS, PLUS};
use crate::TimeSpan;
use core::fmt::{Display, Formatter, Result};

pub struct Inner {
    time: Option<TimeSpan>,
    drop_decimals: bool,
    accuracy: Accuracy,
}

/// The Delta Time Formatter formats Time Spans as a comparison of two
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
pub struct Delta(bool, Accuracy);

impl Delta {
    /// Creates a new default Delta Time Formatter that drops the fractional
    /// part and uses tenths when showing the fractional part.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new custom Delta Time Formatter where you can specify whether
    /// the fractional part should be dropped for deltas that are larger than 1
    /// minute and how many digits to show for the fractional part.
    pub fn custom(drop_decimals: bool, accuracy: Accuracy) -> Self {
        Delta(drop_decimals, accuracy)
    }

    /// Creates a new Delta Time Formatter that drops the fractional part and
    /// uses tenths when showing the fractional part.
    pub fn with_decimal_dropping() -> Self {
        Delta(true, Accuracy::Tenths)
    }
}

impl Default for Delta {
    fn default() -> Self {
        Delta(true, Accuracy::Tenths)
    }
}

impl TimeFormatter<'_> for Delta {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        Inner {
            time: time.into(),
            drop_decimals: self.0,
            accuracy: self.1,
        }
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(time) = self.time {
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "{}", MINUS)?;
            } else {
                write!(f, "{}", PLUS)?;
            }
            let seconds = total_seconds % 60.0;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let hours = total_minutes / 60;
            if hours > 0 {
                if self.drop_decimals {
                    write!(f, "{}:{:02}:{:02}", hours, minutes, seconds as u8)
                } else {
                    write!(
                        f,
                        "{}:{:02}:{}",
                        hours,
                        minutes,
                        self.accuracy.format_seconds(seconds, true)
                    )
                }
            } else if total_minutes > 0 {
                if self.drop_decimals {
                    write!(f, "{}:{:02}", minutes, seconds as u8)
                } else {
                    write!(
                        f,
                        "{}:{}",
                        minutes,
                        self.accuracy.format_seconds(seconds, true)
                    )
                }
            } else {
                write!(f, "{}", self.accuracy.format_seconds(seconds, false))
            }
        } else {
            write!(f, "{}", DASH)
        }
    }
}
