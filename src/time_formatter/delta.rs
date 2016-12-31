use std::fmt::{Result, Formatter, Display};
use TimeSpan;
use super::TimeFormatter;

pub struct Inner {
    time: Option<TimeSpan>,
    drop_decimals: bool,
}
pub struct Delta(bool);

impl Delta {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_decimal_dropping() -> Self {
        Delta(true)
    }
}

impl Default for Delta {
    fn default() -> Self {
        Delta(false)
    }
}

impl<'a> TimeFormatter<'a> for Delta {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
        where T: Into<Option<TimeSpan>>
    {
        Inner {
            time: time.into(),
            drop_decimals: self.0,
        }
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.time {
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "−")?;
            } else {
                write!(f, "+")?;
            }
            let seconds = total_seconds % 60.0;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let hours = total_minutes / 60;
            if hours > 0 {
                write!(f, "{}:{:02}:{:05.2}", hours, minutes, seconds)
            } else if total_minutes > 0 {
                if self.drop_decimals {
                    write!(f, "{}:{:02}", minutes, seconds as u8)
                } else {
                    write!(f, "{}:{:05.2}", minutes, seconds)
                }
            } else {
                write!(f, "{:.2}", seconds)
            }
        } else {
            write!(f, "-")
        }
    }
}
