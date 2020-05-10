//! The analysis module provides a variety of functions for calculating
//! information about runs.

pub mod current_pace;
pub mod delta;
pub mod pb_chance;
pub mod possible_time_save;
mod skill_curve;
pub mod state_helper;
pub mod sum_of_segments;
pub mod total_playtime;

pub use self::skill_curve::SkillCurve;
pub use self::state_helper::*;

#[cfg(test)]
mod tests;
