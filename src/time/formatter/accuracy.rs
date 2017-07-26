#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum Accuracy {
    /// No Fractional Part
    Seconds,
    /// .2
    Tenths,
    /// .23
    Hundredths,
}

impl Accuracy {
    pub fn format_seconds(self, seconds: f64, zero_prefix: bool) -> FormattedSeconds {
        FormattedSeconds {
            accuracy: self,
            seconds: seconds,
            zero_prefix: zero_prefix,
        }
    }
}

use super::{extract_tenths, extract_hundredths};
use std::fmt::{Result, Formatter, Display};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct FormattedSeconds {
    accuracy: Accuracy,
    seconds: f64,
    zero_prefix: bool,
}

impl Display for FormattedSeconds {
    fn fmt(&self, f: &mut Formatter) -> Result {
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
        }
    }
}
