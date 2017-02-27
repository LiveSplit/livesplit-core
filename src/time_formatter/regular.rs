use std::fmt::{Result, Formatter, Display};
use super::{Accuracy, TimeFormatter};
use TimeSpan;

pub struct Inner {
    time: Option<TimeSpan>,
    accuracy: Accuracy,
}

pub struct Regular {
    accuracy: Accuracy,
}

impl Regular {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_accuracy(accuracy: Accuracy) -> Self {
        Regular { accuracy: accuracy }
    }
}

impl Default for Regular {
    fn default() -> Self {
        Regular { accuracy: Accuracy::Seconds }
    }
}

impl<'a> TimeFormatter<'a> for Regular {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
        where T: Into<Option<TimeSpan>>
    {
        Inner {
            time: time.into(),
            accuracy: self.accuracy,
        }
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.time {
            let total_seconds = time.total_seconds();
            let seconds = total_seconds % 60.0;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let hours = total_minutes / 60;
            if hours > 0 {
                write!(f,
                       "{}:{:02}:{}",
                       hours,
                       minutes,
                       self.accuracy.format_seconds(seconds, true))
            } else {
                write!(f,
                       "{}:{}",
                       minutes,
                       self.accuracy.format_seconds(seconds, true))
            }
        } else {
            write!(f, "—")
        }
    }
}
