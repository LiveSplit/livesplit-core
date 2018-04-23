//! The run module provides everything necessary for working with Runs, like
//! parsing and saving or editing them.
//!
//! # Examples
//!
//! ```
//! use livesplit_core::run::{Run, Segment};
//!
//! let mut run = Run::new();
//!
//! run.set_game_name("Super Mario Odyssey");
//! run.set_category_name("Darker Side");
//!
//! run.push_segment(Segment::new("Cap Kingdom"));
//! run.push_segment(Segment::new("Cascade Kingdom"));
//! ```

mod attempt;
#[cfg(feature = "editing")]
pub mod editor;
#[cfg(feature = "parsing")]
pub mod parser;
mod run;
mod run_metadata;
pub mod saver;
mod segment;
mod segment_history;

#[cfg(test)]
mod tests;

pub use self::attempt::Attempt;
#[cfg(feature = "editing")]
pub use self::editor::{Editor, RenameError};
pub use self::run::{ComparisonError, ComparisonsIter, Run};
pub use self::run_metadata::RunMetadata;
pub use self::segment::Segment;
pub use self::segment_history::SegmentHistory;
