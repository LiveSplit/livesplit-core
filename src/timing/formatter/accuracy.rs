use super::{format_padded, NANOS_PER_HUNDREDTH, NANOS_PER_MILLI, NANOS_PER_TENTH};
use core::{
    fmt::{Display, Formatter, Result},
    str,
};
use serde_derive::{Deserialize, Serialize};

/// The `Accuracy` describes how many digits to show for the fractional part of a
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
    pub const fn format_nanoseconds(self, nanoseconds: u32) -> FractionalPart {
        FractionalPart {
            accuracy: self,
            nanoseconds,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct FractionalPart {
    accuracy: Accuracy,
    nanoseconds: u32,
}

impl Display for FractionalPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.accuracy {
            Accuracy::Seconds => Ok(()),
            Accuracy::Tenths => {
                f.write_str(".")?;
                let v = (self.nanoseconds / NANOS_PER_TENTH) as u8;
                assert!(v < 10);
                // SAFETY: We ensured the value is between 0 and 10, so adding
                // that on top of ASCII '0' ensures us that we get an ASCII
                // digit.
                unsafe { f.write_str(str::from_utf8_unchecked(&[v + b'0'])) }
            }
            Accuracy::Hundredths => {
                f.write_str(".")?;
                f.write_str(format_padded(
                    (self.nanoseconds / NANOS_PER_HUNDREDTH) as u8,
                ))
            }
            Accuracy::Milliseconds => {
                f.write_str(".")?;
                let first = (self.nanoseconds / NANOS_PER_TENTH) as u8;
                let second_and_third =
                    ((self.nanoseconds % NANOS_PER_TENTH) / NANOS_PER_MILLI) as u8;
                assert!(first < 10);
                // SAFETY: We ensured the value is between 0 and 10, so adding
                // that on top of ASCII '0' ensures us that we get an ASCII
                // digit.
                unsafe {
                    f.write_str(str::from_utf8_unchecked(&[first + b'0']))?;
                }
                f.write_str(format_padded(second_and_third))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_seconds() {
        let acc = Accuracy::Seconds;
        assert_eq!(acc.format_nanoseconds(0).to_string(), "");
        assert_eq!(acc.format_nanoseconds(1).to_string(), "");
        assert_eq!(acc.format_nanoseconds(789_654_321).to_string(), "");
        assert_eq!(acc.format_nanoseconds(7_654_321).to_string(), "");
        assert_eq!(acc.format_nanoseconds(70_654_321).to_string(), "");
        assert_eq!(acc.format_nanoseconds(700_654_321).to_string(), "");
        assert_eq!(acc.format_nanoseconds(109_654_321).to_string(), "");
        assert_eq!(acc.format_nanoseconds(999_999_999).to_string(), "");
    }

    #[test]
    fn format_tenths() {
        let acc = Accuracy::Tenths;
        assert_eq!(acc.format_nanoseconds(0).to_string(), ".0");
        assert_eq!(acc.format_nanoseconds(1).to_string(), ".0");
        assert_eq!(acc.format_nanoseconds(789_654_321).to_string(), ".7");
        assert_eq!(acc.format_nanoseconds(7_654_321).to_string(), ".0");
        assert_eq!(acc.format_nanoseconds(70_654_321).to_string(), ".0");
        assert_eq!(acc.format_nanoseconds(700_654_321).to_string(), ".7");
        assert_eq!(acc.format_nanoseconds(109_654_321).to_string(), ".1");
        assert_eq!(acc.format_nanoseconds(999_999_999).to_string(), ".9");
    }

    #[test]
    fn format_hundredths() {
        let acc = Accuracy::Hundredths;
        assert_eq!(acc.format_nanoseconds(0).to_string(), ".00");
        assert_eq!(acc.format_nanoseconds(1).to_string(), ".00");
        assert_eq!(acc.format_nanoseconds(789_654_321).to_string(), ".78");
        assert_eq!(acc.format_nanoseconds(7_654_321).to_string(), ".00");
        assert_eq!(acc.format_nanoseconds(70_654_321).to_string(), ".07");
        assert_eq!(acc.format_nanoseconds(700_654_321).to_string(), ".70");
        assert_eq!(acc.format_nanoseconds(109_654_321).to_string(), ".10");
        assert_eq!(acc.format_nanoseconds(999_999_999).to_string(), ".99");
    }

    #[test]
    fn format_milliseconds() {
        let acc = Accuracy::Milliseconds;
        assert_eq!(acc.format_nanoseconds(0).to_string(), ".000");
        assert_eq!(acc.format_nanoseconds(1).to_string(), ".000");
        assert_eq!(acc.format_nanoseconds(789_654_321).to_string(), ".789");
        assert_eq!(acc.format_nanoseconds(7_654_321).to_string(), ".007");
        assert_eq!(acc.format_nanoseconds(70_654_321).to_string(), ".070");
        assert_eq!(acc.format_nanoseconds(700_654_321).to_string(), ".700");
        assert_eq!(acc.format_nanoseconds(109_654_321).to_string(), ".109");
        assert_eq!(acc.format_nanoseconds(999_999_999).to_string(), ".999");
    }
}
