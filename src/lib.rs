#![allow(unknown_lints)]

extern crate base64;
extern crate byteorder;
extern crate chrono;
#[macro_use]
extern crate derive_more;
extern crate image as imagelib;
extern crate odds;
#[macro_use]
extern crate quick_error;
extern crate quick_xml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate unicase;
pub extern crate livesplit_hotkey as hotkey;
pub extern crate ordermap;
pub extern crate palette;
pub extern crate parking_lot;

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
pub use self::layout::{Component, Editor as LayoutEditor,
                       GeneralSettings as GeneralLayoutSettings, Layout};
pub use self::run::{Attempt, Editor as RunEditor, Run, RunMetadata, Segment, SegmentHistory};
pub use self::time::{AtomicDateTime, GameTime, RealTime, SharedTimer, Time, TimeSpan, TimeStamp,
                     Timer, TimerPhase, TimingMethod};
