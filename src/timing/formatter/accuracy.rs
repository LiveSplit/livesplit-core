use serde::{Deserialize, Serialize};

/// The Accuracy describes how many digits to show for the fractional part of a
/// time.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum Accuracy {
    /// Don't show any fractional part.
    Seconds,
    /// Show tenths of a second (12:34.5).
    Tenths,
    /// Show hundredths of a second (12:34.56).
    Hundredths,
    /// Show thousandths of a second, also known as milliseconds (12:34.567).
    Milliseconds,
}

impl Accuracy {
    /// Formats the seconds provided with the chosen accuracy. If there should
    /// be a zero prefix, the seconds are prefixed with a 0, if they are less
    /// than 10s.
    pub fn format_seconds(self, seconds: f64, zero_prefix: bool) -> FormattedSeconds {
        FormattedSeconds {
            accuracy: self,
            seconds,
            zero_prefix,
        }
    }
}

use super::{extract_hundredths, extract_milliseconds, extract_tenths};
use core::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FormattedSeconds {
    accuracy: Accuracy,
    seconds: f64,
    zero_prefix: bool,
}

impl Display for FormattedSeconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = self.seconds as u8;
        if self.zero_prefix {
            write!(f, "{:02}", s)?;
        } else {
            write!(f, "{}", s)?;
        }
        match self.accuracy {
            Accuracy::Seconds => Ok(()),
            Accuracy::Tenths => write!(f, ".{}", extract_tenths(self.seconds)),
            Accuracy::Hundredths => write!(f, ".{:02}", extract_hundredths(self.seconds)),
            Accuracy::Milliseconds => write!(f, ".{:03}", extract_milliseconds(self.seconds)),
        }
    }
}
