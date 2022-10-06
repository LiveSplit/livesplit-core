use super::{
    format_padded, Accuracy, TimeFormatter, DASH, MINUS, PLUS, SECONDS_PER_HOUR, SECONDS_PER_MINUTE,
};
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
    pub const fn new() -> Self {
        Delta(true, Accuracy::Tenths)
    }

    /// Creates a new custom Delta Time Formatter where you can specify whether
    /// the fractional part should be dropped for deltas that are larger than 1
    /// minute and how many digits to show for the fractional part.
    pub const fn custom(drop_decimals: bool, accuracy: Accuracy) -> Self {
        Delta(drop_decimals, accuracy)
    }

    /// Creates a new Delta Time Formatter that drops the fractional part and
    /// uses tenths when showing the fractional part.
    pub const fn with_decimal_dropping() -> Self {
        Delta(true, Accuracy::Tenths)
    }
}

impl Default for Delta {
    fn default() -> Self {
        Self::new()
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
            let (total_seconds, nanoseconds) = time.to_seconds_and_subsec_nanoseconds();
            let (total_seconds, nanoseconds) = if total_seconds < 0 {
                f.write_str(MINUS)?;
                ((-total_seconds) as u64, (-nanoseconds) as u32)
            } else {
                f.write_str(PLUS)?;
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
                return self.accuracy.format_nanoseconds(nanoseconds).fmt(f);
            }
            if !self.drop_decimals {
                self.accuracy.format_nanoseconds(nanoseconds).fmt(f)
            } else {
                Ok(())
            }
        } else {
            f.write_str(DASH)
        }
    }
}
