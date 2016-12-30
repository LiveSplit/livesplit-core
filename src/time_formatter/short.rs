use std::fmt::{Result, Formatter, Display};
use TimeSpan;

pub struct Short(Option<TimeSpan>);

impl Short {
    pub fn new<T: Into<Option<TimeSpan>>>(time: T) -> Self {
        Short(time.into())
    }
}

impl Display for Short {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.0 {
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "−")?;
            }
            let seconds = total_seconds % 60.0;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let hours = total_minutes / 60;
            if hours > 0 {
                write!(f, "{}:{:02}:{:05.2}", hours, minutes, seconds)
            } else if total_minutes > 0 {
                write!(f, "{}:{:05.2}", minutes, seconds)
            } else {
                write!(f, "{:.2}", seconds)
            }
        } else {
            write!(f, "0.00")
        }
    }
}
