use std::fmt::{Result, Formatter, Display};
use TimeSpan;
use super::{TimeFormatter, extract_hundredths, MINUS};

pub struct TimeInner(Option<TimeSpan>);
pub struct Time;

impl<'a> TimeFormatter<'a> for Time {
    type Inner = TimeInner;

    fn format<T>(&self, time: T) -> Self::Inner
        where T: Into<Option<TimeSpan>>
    {
        TimeInner(time.into())
    }
}

impl Display for TimeInner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.0 {
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "{}", MINUS)?;
            }
            let seconds = (total_seconds % 60.0) as u8;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let hours = total_minutes / 60;
            if hours > 0 {
                write!(f, "{}:{:02}:{:02}", hours, minutes, seconds)
            } else if total_minutes > 0 {
                write!(f, "{}:{:02}", minutes, seconds)
            } else {
                write!(f, "{}", seconds)
            }
        } else {
            write!(f, "0")
        }
    }
}

pub struct FractionInner(Option<TimeSpan>);
pub struct Fraction;

impl<'a> TimeFormatter<'a> for Fraction {
    type Inner = FractionInner;

    fn format<T>(&self, time: T) -> Self::Inner
        where T: Into<Option<TimeSpan>>
    {
        FractionInner(time.into())
    }
}

impl Display for FractionInner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.0 {
            write!(f, ".{:02}", extract_hundredths(time.total_seconds()))
        } else {
            write!(f, ".00")
        }
    }
}

#[test]
fn test() {
    let time = "4:20.999999".parse::<TimeSpan>().unwrap();
    assert_eq!(FractionInner(Some(time)).to_string(), ".99");
}
