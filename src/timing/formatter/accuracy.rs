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
    /// Formats the nanoseconds provided with the chosen accuracy.
    pub const fn format_nanoseconds(self, nanoseconds: u32) -> FormattedSeconds {
        FormattedSeconds {
            accuracy: self,
            nanoseconds,
        }
    }
}

use core::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct FormattedSeconds {
    accuracy: Accuracy,
    nanoseconds: u32,
}

impl Display for FormattedSeconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.accuracy {
            Accuracy::Seconds => Ok(()),
            Accuracy::Tenths => write!(f, ".{}", self.nanoseconds / 100_000_000),
            Accuracy::Hundredths => write!(f, ".{:02}", self.nanoseconds / 10_000_000),
            Accuracy::Milliseconds => write!(f, ".{:03}", self.nanoseconds / 1_000_000),
        }
    }
}
