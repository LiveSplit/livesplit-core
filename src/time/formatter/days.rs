use std::fmt::{Result, Formatter, Display};
use TimeSpan;
use super::{TimeFormatter, MINUS};

pub struct Inner {
    time: Option<TimeSpan>,
}

#[derive(Default)]
pub struct Days;

impl Days {
    pub fn new() -> Self {
        Days
    }
}

impl<'a> TimeFormatter<'a> for Days {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
    {
        Inner { time: time.into() }
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.time {
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "{}", MINUS)?;
            }
            let total_seconds = total_seconds as u64;
            let seconds = total_seconds % 60;
            let total_minutes = total_seconds / 60;
            let minutes = total_minutes % 60;
            let total_hours = total_minutes / 60;
            let hours = total_hours % 24;
            let days = total_hours / 24;

            if days > 0 {
                write!(f, "{}d ", days)?;
            }

            if total_hours > 0 {
                write!(f, "{}:{:02}:{:02}", hours, minutes, seconds)
            } else {
                write!(f, "{}:{:02}", minutes, seconds)
            }
        } else {
            write!(f, "0")
        }
    }
}
