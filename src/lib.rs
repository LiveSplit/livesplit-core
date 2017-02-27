#![allow(unknown_lints)]

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate odds;
extern crate serde_json;
extern crate sxd_document;
#[macro_use]
extern crate quick_error;
extern crate base64;
extern crate byteorder;
extern crate image as imagelib;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate derive_new;
extern crate pdqsort;

mod atomic_date_time;
mod attempt;
mod color;
mod image;
mod run_metadata;
mod run;
mod segment_history;
mod segment;
mod time_stamp;
mod time;
mod timer_phase;
mod timer;
mod timing_method;
pub mod component;
pub mod comparison;
pub mod parser;
pub mod run_editor;
pub mod saver;
pub mod state_helper; // TODO Should maybe not be pub
pub mod sum_of_segments; // TODO Should maybe not be pub
pub mod time_formatter;
pub mod time_span;

pub use chrono::{DateTime, UTC};
pub use self::atomic_date_time::AtomicDateTime;
pub use self::attempt::Attempt;
pub use self::color::Color;
pub use self::image::Image;
pub use self::run::Run;
pub use self::run_metadata::RunMetadata;
pub use self::segment_history::SegmentHistory;
pub use self::segment::Segment;
pub use self::run_editor::RunEditor;
pub use self::time::{Time, RealTime, GameTime};
pub use self::time_span::TimeSpan;
pub use self::time_stamp::TimeStamp;
pub use self::timer::Timer;
pub use self::timer_phase::TimerPhase;
pub use self::timing_method::TimingMethod;
