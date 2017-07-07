use std::fmt::{Result, Formatter, Display};
use TimeSpan;
use super::{TimeFormatter, extract_hundredths, extract_tenths, MINUS, DigitsFormat, Accuracy};

pub struct TimeInner {
    time: Option<TimeSpan>,
    digits_format: DigitsFormat,
}

pub struct Time {
    digits_format: DigitsFormat,
}

impl Time {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_digits_format(digits_format: DigitsFormat) -> Self {
        Time { digits_format }
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            digits_format: DigitsFormat::SingleDigitSeconds,
        }
    }
}

impl<'a> TimeFormatter<'a> for Time {
    type Inner = TimeInner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        TimeInner {
            time: time.into(),
            digits_format: self.digits_format,
        }
    }
}

impl Display for TimeInner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.time {
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "{}", MINUS)?;
            }
            let seconds = (total_seconds % 60.0) as u8;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let hours = total_minutes / 60;
            if self.digits_format == DigitsFormat::DoubleDigitHours {
                write!(f, "{:02}:{:02}:{:02}", hours, minutes, seconds)
            } else if hours > 0 || self.digits_format == DigitsFormat::SingleDigitHours {
                write!(f, "{}:{:02}:{:02}", hours, minutes, seconds)
            } else if self.digits_format == DigitsFormat::DoubleDigitMinutes {
                write!(f, "{:02}:{:02}", minutes, seconds)
            } else if total_minutes > 0 || self.digits_format == DigitsFormat::SingleDigitMinutes {
                write!(f, "{}:{:02}", minutes, seconds)
            } else if self.digits_format == DigitsFormat::DoubleDigitSeconds {
                write!(f, "{:02}", seconds)
            } else {
                write!(f, "{}", seconds)
            }
        } else {
            match self.digits_format {
                DigitsFormat::SingleDigitSeconds => write!(f, "0"),
                DigitsFormat::DoubleDigitSeconds => write!(f, "00"),
                DigitsFormat::SingleDigitMinutes => write!(f, "0:00"),
                DigitsFormat::DoubleDigitMinutes => write!(f, "00:00"),
                DigitsFormat::SingleDigitHours => write!(f, "0:00:00"),
                DigitsFormat::DoubleDigitHours => write!(f, "00:00:00"),
            }
        }
    }
}

pub struct FractionInner {
    time: Option<TimeSpan>,
    accuracy: Accuracy,
}

pub struct Fraction {
    accuracy: Accuracy,
}

impl Fraction {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_accuracy(accuracy: Accuracy) -> Self {
        Fraction { accuracy }
    }
}

impl Default for Fraction {
    fn default() -> Self {
        Fraction {
            accuracy: Accuracy::Hundredths,
        }
    }
}

impl<'a> TimeFormatter<'a> for Fraction {
    type Inner = FractionInner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        FractionInner {
            time: time.into(),
            accuracy: self.accuracy,
        }
    }
}

impl Display for FractionInner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.time {
            match self.accuracy {
                Accuracy::Hundredths => {
                    write!(f, ".{:02}", extract_hundredths(time.total_seconds()))
                }
                Accuracy::Tenths => write!(f, ".{}", extract_tenths(time.total_seconds())),
                Accuracy::Seconds => Ok(()),
            }
        } else {
            match self.accuracy {
                Accuracy::Hundredths => write!(f, ".00"),
                Accuracy::Tenths => write!(f, ".0"),
                Accuracy::Seconds => Ok(()),
            }
        }
    }
}

#[test]
fn test() {
    let time = "4:20.999999".parse::<TimeSpan>().unwrap();
    assert_eq!(Fraction::new().format(Some(time)).to_string(), ".99");
}
