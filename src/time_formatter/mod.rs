mod accuracy;
mod complete;
mod delta;
mod regular;
mod short;
pub mod none_wrapper;
pub mod timer;

pub use self::short::Short;
pub use self::complete::Complete;
pub use self::delta::Delta;
pub use self::regular::Regular;
pub use self::accuracy::Accuracy;

use std::fmt::Display;
use TimeSpan;

pub trait TimeFormatter<'a> {
    type Inner: Display;

    fn format<T>(&'a self, time: T) -> Self::Inner where T: Into<Option<TimeSpan>>;
}
