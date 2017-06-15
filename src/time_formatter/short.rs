use std::fmt::{Result, Formatter, Display};
use TimeSpan;
use super::{Accuracy, TimeFormatter, MINUS};

pub struct Inner {
    time: Option<TimeSpan>,
    accuracy: Accuracy,
}

pub struct Short {
    accuracy: Accuracy,
}

impl Short {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_accuracy(accuracy: Accuracy) -> Self {
        Short { accuracy: accuracy }
    }
}

impl Default for Short {
    fn default() -> Self {
        Short { accuracy: Accuracy::Hundredths }
    }
}

impl<'a> TimeFormatter<'a> for Short {
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
            match self.accuracy {
                Accuracy::Hundredths => write!(f, "0.00"),
                Accuracy::Tenths => write!(f, "0.0"),
                Accuracy::Seconds => write!(f, "0"),
            }
        }
    }
}

#[test]
fn test() {
    let time = "4:20.69".parse::<TimeSpan>().unwrap();
    assert_eq!(Short::new().format(time).to_string(), "4:20.69");
}
