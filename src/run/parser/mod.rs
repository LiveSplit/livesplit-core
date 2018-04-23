//! The parser module provides all the different parsers available for parsing
//! splits files into Runs. If the file type of the splits file is not known,
//! the composite parser can be used, which tries to figure out which splits
//! file format is used and parses it with the parser for that format.
//!
//! # Examples
//!
//! Using the composite parser to parse a splits file of an unknown file format.
//!
//! ```no_run
//! use livesplit_core::run::parser::composite;
//! use std::fs::File;
//! use std::io::BufReader;
//! use std::path::PathBuf;
//!
//! // Load the file.
//! let path = PathBuf::from("path/to/splits_file");
//! let file = BufReader::new(File::open(&path).expect("File not found"));
//!
//! // We want to load additional files from the file system, like segment icons.
//! let load_files = true;
//!
//! // Actually parse the file.
//! let result = composite::parse(file, Some(path), load_files);
//! let parsed = result.expect("Not a valid splits file");
//!
//! // Print out the detected file format.
//! println!("Splits File Format: {}", parsed.kind);
//!
//! // Get out the Run object.
//! let run = parsed.run;
//! ```

pub mod composite;
#[cfg(feature = "face_split_parsing")]
pub mod face_split;
pub mod livesplit;
#[cfg(feature = "llanfair_parsing")]
pub mod llanfair;
#[cfg(feature = "llanfair2_parsing")]
pub mod llanfair2;
#[cfg(feature = "llanfair_gered_parsing")]
pub mod llanfair_gered;
#[cfg(feature = "portal2_live_timer_parsing")]
pub mod portal2_live_timer;
#[cfg(feature = "shit_split_parsing")]
pub mod shit_split;
#[cfg(feature = "source_live_timer_parsing")]
pub mod source_live_timer;
#[cfg(feature = "splitterz_parsing")]
pub mod splitterz;
#[cfg(feature = "splitty_parsing")]
pub mod splitty;
#[cfg(feature = "time_split_tracker_parsing")]
pub mod time_split_tracker;
#[cfg(feature = "urn_parsing")]
pub mod urn;
#[cfg(feature = "worstrun_parsing")]
pub mod worstrun;
#[cfg(feature = "wsplit_parsing")]
pub mod wsplit;

mod timer_kind;
mod xml_util;

pub use self::timer_kind::TimerKind;
