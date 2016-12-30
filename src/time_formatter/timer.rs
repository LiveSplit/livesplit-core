use std::fmt::{Result, Formatter, Display};
use TimeSpan;

pub struct Time(Option<TimeSpan>);

impl Time {
    pub fn new<T: Into<Option<TimeSpan>>>(time: T) -> Self {
        Time(time.into())
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.0 {
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "−")?;
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

pub struct Fraction(Option<TimeSpan>);

impl Fraction {
    pub fn new<T: Into<Option<TimeSpan>>>(time: T) -> Self {
        Fraction(time.into())
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.0 {
            write!(f,
                   ".{:02}",
                   ((time.total_seconds().abs() % 1.0) * 100.0) as u8)
        } else {
            write!(f, ".00")
        }
    }
}
