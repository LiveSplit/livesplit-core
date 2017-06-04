mod accuracy;
mod complete;
mod days;
mod delta;
mod possible_time_save;
mod regular;
mod short;
pub mod none_wrapper;
pub mod timer;

pub use self::accuracy::Accuracy;
pub use self::complete::Complete;
pub use self::days::Days;
pub use self::delta::Delta;
pub use self::possible_time_save::PossibleTimeSave;
pub use self::regular::Regular;
pub use self::short::Short;

use std::fmt::Display;
use TimeSpan;
use std::cmp::min;

pub trait TimeFormatter<'a> {
    type Inner: Display;

    fn format<T>(&'a self, time: T) -> Self::Inner where T: Into<Option<TimeSpan>>;
}

const EPSILON: f64 = 0.0000001;

fn extract_tenths(seconds: f64) -> u8 {
    min(9, ((seconds.abs() % 1.0) * 10.0 + EPSILON).floor() as u8)
}

fn extract_hundredths(seconds: f64) -> u8 {
    min(99, ((seconds.abs() % 1.0) * 100.0 + EPSILON).floor() as u8)
}
