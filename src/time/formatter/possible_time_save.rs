use std::fmt::{Display, Formatter, Result};
use TimeSpan;
use super::{Accuracy, TimeFormatter, DASH, MINUS};

pub struct Inner {
    time: Option<TimeSpan>,
    accuracy: Accuracy,
}

pub struct PossibleTimeSave {
    accuracy: Accuracy,
}

impl PossibleTimeSave {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_accuracy(accuracy: Accuracy) -> Self {
        PossibleTimeSave { accuracy: accuracy }
    }
}

impl Default for PossibleTimeSave {
    fn default() -> Self {
        PossibleTimeSave {
            accuracy: Accuracy::Hundredths,
        }
    }
}

impl<'a> TimeFormatter<'a> for PossibleTimeSave {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>,
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
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "{}", MINUS)?;
            }
            let seconds = total_seconds % 60.0;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let hours = total_minutes / 60;
            if hours > 0 {
                write!(
                    f,
                    "{}:{:02}:{}",
                    hours,
                    minutes,
                    self.accuracy.format_seconds(seconds, true)
                )
            } else if total_minutes > 0 {
                write!(
                    f,
                    "{}:{}",
                    minutes,
                    self.accuracy.format_seconds(seconds, true)
                )
            } else {
                write!(f, "{}", self.accuracy.format_seconds(seconds, false))
            }
        } else {
            write!(f, "{}", DASH)
        }
    }
}
