//! The formatter module provides different Time Formatters that can be used to
//! format optional Time Spans in various ways.
//!
//! # Examples
//!
//! Using a Short Time Formatter to format a Time Span.
//!
//! ```
//! use livesplit_core::time::formatter::{Short, TimeFormatter};
//! use livesplit_core::TimeSpan;
//!
//! // Create the Short Time Formatter.
//! let formatter = Short::new();
//!
//! // Create a Time Span.
//! let time = TimeSpan::from_seconds(-(4.0 * 60.0 + 23.5));
//!
//! // Format it with the formatter.
//! let formatted = formatter.format(Some(time)).to_string();
//! assert_eq!(formatted, "−4:23.50");
//! ```

mod accuracy;
mod complete;
mod days;
mod delta;
mod digits_format;
pub mod none_wrapper;
mod possible_time_save;
mod regular;
mod short;
pub mod timer;

pub use self::accuracy::Accuracy;
pub use self::complete::Complete;
pub use self::days::Days;
pub use self::delta::Delta;
pub use self::digits_format::DigitsFormat;
pub use self::possible_time_save::PossibleTimeSave;
pub use self::regular::Regular;
pub use self::short::Short;

use std::cmp::min;
use std::fmt::Display;
use TimeSpan;

/// Time Formatters can be used to format optional Time Spans in various ways.
pub trait TimeFormatter<'a> {
    /// The actual type that can be displayed.
    type Inner: Display;

    /// Constructs an object that displays the provided time span in a certain
    /// way.
    fn format<T>(&'a self, time: T) -> Self::Inner
    where
        T: Into<Option<TimeSpan>>;
}

const EPSILON: f64 = 0.0000001;
/// The dash symbol to use for generic dashes in text.
pub const DASH: &str = "—";
/// The minus symbol to use for negative numbers.
pub const MINUS: &str = "−";
/// The minus symbol to use for negative numbers, where the minus symbol
/// specifically needs to be the ASCII minus.
pub const ASCII_MINUS: &str = "-";
/// The plus symbol to use for positive numbers.
pub const PLUS: &str = "+";

fn extract_tenths(seconds: f64) -> u8 {
    min(9, ((seconds.abs() % 1.0) * 10.0 + EPSILON).floor() as u8)
}

fn extract_hundredths(seconds: f64) -> u8 {
    min(99, ((seconds.abs() % 1.0) * 100.0 + EPSILON).floor() as u8)
}
