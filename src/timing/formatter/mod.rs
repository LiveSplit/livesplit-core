//! The formatter module provides different Time Formatters that can be used to
//! format a [`TimeSpan`] Option in various ways.
//!
//! # Examples
//!
//! Using a [`SegmentTime`] [`TimeFormatter`] to format a [`TimeSpan`].
//!
//! ```
//! use livesplit_core::timing::formatter::{SegmentTime, TimeFormatter};
//! use livesplit_core::TimeSpan;
//!
//! // Create the SegmentTime TimeFormatter.
//! let formatter = SegmentTime::new();
//!
//! // Create a TimeSpan.
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
mod regular;
mod segment_time;
pub mod timer;

pub use self::{
    accuracy::Accuracy, complete::Complete, days::Days, delta::Delta, digits_format::DigitsFormat,
    regular::Regular, segment_time::SegmentTime,
};

use crate::TimeSpan;
use core::{fmt::Display, str};

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

/// The dash symbol to use for generic dashes in text.
pub const DASH: &str = "—";
/// The minus symbol to use for negative numbers.
pub const MINUS: &str = "−";
/// The minus symbol to use for negative numbers, where the minus symbol
/// specifically needs to be the ASCII minus.
pub const ASCII_MINUS: &str = "-";
/// The plus symbol to use for positive numbers.
pub const PLUS: &str = "+";

const SECONDS_PER_MINUTE: u64 = 60;
const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;
const SECONDS_PER_DAY: u64 = 24 * SECONDS_PER_HOUR;

const NANOS_PER_MILLI: u32 = 1_000_000;
const NANOS_PER_HUNDREDTH: u32 = 10_000_000;
const NANOS_PER_TENTH: u32 = 100_000_000;

#[rustfmt::skip]
static LOOKUP: [[u8; 2]; 100] = [
    *b"00", *b"01", *b"02", *b"03", *b"04", *b"05", *b"06", *b"07", *b"08", *b"09",
    *b"10", *b"11", *b"12", *b"13", *b"14", *b"15", *b"16", *b"17", *b"18", *b"19",
    *b"20", *b"21", *b"22", *b"23", *b"24", *b"25", *b"26", *b"27", *b"28", *b"29",
    *b"30", *b"31", *b"32", *b"33", *b"34", *b"35", *b"36", *b"37", *b"38", *b"39",
    *b"40", *b"41", *b"42", *b"43", *b"44", *b"45", *b"46", *b"47", *b"48", *b"49",
    *b"50", *b"51", *b"52", *b"53", *b"54", *b"55", *b"56", *b"57", *b"58", *b"59",
    *b"60", *b"61", *b"62", *b"63", *b"64", *b"65", *b"66", *b"67", *b"68", *b"69",
    *b"70", *b"71", *b"72", *b"73", *b"74", *b"75", *b"76", *b"77", *b"78", *b"79",
    *b"80", *b"81", *b"82", *b"83", *b"84", *b"85", *b"86", *b"87", *b"88", *b"89",
    *b"90", *b"91", *b"92", *b"93", *b"94", *b"95", *b"96", *b"97", *b"98", *b"99",
];

#[inline(always)]
fn format_padded(x: u8) -> &'static str {
    // SAFETY: The lookup table is always initialized with valid UTF-8.
    unsafe { str::from_utf8_unchecked(&LOOKUP[x as usize]) }
}

#[inline(always)]
fn format_unpadded(x: u8) -> &'static str {
    // SAFETY: The lookup table is always initialized with valid UTF-8.
    unsafe { str::from_utf8_unchecked(&LOOKUP[x as usize][(x < 10) as usize..]) }
}
