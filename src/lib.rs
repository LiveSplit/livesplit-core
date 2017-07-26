#![allow(unknown_lints)]

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate odds;
extern crate serde;
extern crate serde_json;
extern crate sxd_document;
#[macro_use]
extern crate quick_error;
extern crate base64;
extern crate byteorder;
extern crate image as imagelib;
#[macro_use]
extern crate derive_more;
pub extern crate parking_lot;
pub extern crate livesplit_hotkey as hotkey;
extern crate unicase;
pub extern crate ordermap;
pub extern crate palette;

mod hotkey_config;
mod hotkey_system;
mod image;
pub mod analysis;
pub mod comparison;
pub mod component;
pub mod layout;
pub mod run;
pub mod settings;
pub mod time;

pub use chrono::{DateTime, Utc};
pub use self::hotkey_config::HotkeyConfig;
pub use self::hotkey_system::HotkeySystem;
pub use self::image::Image;
pub use self::layout::{Layout, Component, Editor as LayoutEditor,
                       GeneralSettings as GeneralLayoutSettings};
pub use self::run::{Attempt, Run, RunMetadata, SegmentHistory, Segment, Editor as RunEditor};
pub use self::time::{AtomicDateTime, Time, RealTime, GameTime, TimeSpan, TimeStamp, Timer,
                     SharedTimer, TimerPhase, TimingMethod};
