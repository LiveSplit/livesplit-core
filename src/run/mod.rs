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
mod run_metadata;
mod run;
mod segment_history;
mod segment;
pub mod editor;
pub mod parser;
pub mod saver;

#[cfg(test)]
mod tests;

pub use self::attempt::Attempt;
pub use self::run_metadata::RunMetadata;
pub use self::run::{ComparisonsIter, Run};
pub use self::segment_history::SegmentHistory;
pub use self::segment::Segment;
pub use self::editor::Editor;

/// Checks a given name against the current comparisons in the Run to
/// ensure that it is valid for use.
pub fn validate_comparison_name(run: &Run, new: &str) -> bool {
    !new.starts_with("[Race]") && !run.comparisons().any(|c| c == new)
}
