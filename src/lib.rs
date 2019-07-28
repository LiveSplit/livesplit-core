#![warn(
    missing_docs,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::complexity,
    rust_2018_idioms
)]
// Clippy false positives
#![allow(
    clippy::block_in_if_condition_stmt,
    clippy::redundant_closure_call,
    clippy::new_ret_no_self
)]

//! livesplit-core is a library that provides a lot of functionality for creating a speedrun timer.
//!
//! # Examples
//!
//! ```
//! use livesplit_core::{Run, Segment, Timer, TimerPhase};
//!
//! // Create a run object that we can use with at least one segment.
//! let mut run = Run::new();
//! run.set_game_name("Super Mario Odyssey");
//! run.set_category_name("Any%");
//! run.push_segment(Segment::new("Cap Kingdom"));
//!
//! // Create the timer from the run.
//! let mut timer = Timer::new(run).expect("Run with at least one segment provided");
//!
//! // Start a new attempt.
//! timer.start();
//! assert_eq!(timer.current_phase(), TimerPhase::Running);
//!
//! // Create a split.
//! timer.split();
//!
//! // The run should be finished now.
//! assert_eq!(timer.current_phase(), TimerPhase::Ended);
//!
//! // Reset the attempt and confirm that we want to store the attempt.
//! timer.reset(true);
//!
//! // The attempt is now over.
//! assert_eq!(timer.current_phase(), TimerPhase::NotRunning);
//! ```

mod platform;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
pub use crate::platform::*;

macro_rules! catch {
    ($($code:tt)*) => {
        (|| { Some({ $($code)* }) })()
    }
}

pub mod analysis;
pub mod comparison;
pub mod component;
mod hotkey_config;
mod hotkey_system;
mod image;
#[cfg(feature = "image-shrinking")]
mod image_shrinking;
pub mod layout;
#[cfg(feature = "rendering")]
pub mod rendering;
pub mod run;
pub mod settings;
#[cfg(test)]
pub mod tests_helper;
pub mod timing;
mod xml_util;

pub use {
    crate::{
        hotkey_config::HotkeyConfig,
        hotkey_system::HotkeySystem,
        image::{CachedImageId, Image},
        layout::{
            Component, Editor as LayoutEditor, GeneralSettings as GeneralLayoutSettings, Layout,
        },
        run::{Attempt, Editor as RunEditor, Run, RunMetadata, Segment, SegmentHistory},
        timing::{
            AtomicDateTime, GameTime, RealTime, SharedTimer, Time, TimeSpan, TimeStamp, Timer,
            TimerPhase, TimingMethod,
        },
    },
    chrono::{DateTime, Utc},
    indexmap, livesplit_hotkey as hotkey, palette, parking_lot,
};
