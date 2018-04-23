#![allow(unknown_lints)]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
// Necessary for some larger quick-error based errors.
#![recursion_limit = "128"]

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

extern crate base64;
extern crate byteorder;
#[macro_use]
extern crate derive_more;
#[cfg(feature = "imagelib")]
extern crate image as imagelib;
extern crate ordered_float;
#[macro_use]
extern crate quick_error;
extern crate quick_xml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate unicase;

pub extern crate indexmap;
#[cfg(feature = "hotkeys")]
pub extern crate livesplit_hotkey as hotkey;
pub extern crate palette;
pub extern crate parking_lot;

mod platform;

use platform::chrono;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use platform::*;

macro_rules! catch {
    ($($code:tt)*) => {
        (|| { Some({ $($code)* }) })()
    }
}

pub mod analysis;
pub mod comparison;
#[cfg(feature = "layout")]
pub mod component;
#[cfg(feature = "hotkeys")]
mod hotkey_config;
#[cfg(feature = "hotkeys")]
mod hotkey_system;
mod image;
#[cfg(feature = "layout")]
pub mod layout;
pub mod run;
pub mod settings;
pub mod time;

#[cfg(feature = "hotkeys")]
pub use self::hotkey_config::HotkeyConfig;
#[cfg(feature = "hotkeys")]
pub use self::hotkey_system::HotkeySystem;
pub use self::image::Image;
#[cfg(feature = "layout")]
pub use self::layout::{Component, Editor as LayoutEditor,
                       GeneralSettings as GeneralLayoutSettings, Layout};
#[cfg(feature = "editing")]
pub use self::run::Editor as RunEditor;
pub use self::run::{Attempt, Run, RunMetadata, Segment, SegmentHistory};
pub use self::time::{AtomicDateTime, GameTime, RealTime, SharedTimer, Time, TimeSpan, TimeStamp,
                     Timer, TimerPhase, TimingMethod};
pub use chrono::{DateTime, Utc};
