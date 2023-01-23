use super::{
    format_padded, Accuracy, TimeFormatter, DASH, MINUS, SECONDS_PER_HOUR, SECONDS_PER_MINUTE,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(time) = self.time {
            let (total_seconds, nanoseconds) = time.to_seconds_and_subsec_nanoseconds();
            let (total_seconds, nanoseconds) = if (total_seconds | nanoseconds as i64) < 0 {
                f.write_str(MINUS)?;
                ((-total_seconds) as u64, (-nanoseconds) as u32)
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

#[test]
fn test() {
    let time = "4:20.69".parse::<TimeSpan>().unwrap();
    let formatted = SegmentTime::new().format(time).to_string();
    assert_eq!(formatted, "4:20.69");
}
