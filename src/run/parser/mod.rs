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
pub mod face_split;
pub mod flitter;
pub mod livesplit;
pub mod llanfair;
pub mod llanfair2;
pub mod llanfair_gered;
pub mod portal2_live_timer;
pub mod shit_split;
pub mod source_live_timer;
pub mod splits_io;
pub mod splitterino;
pub mod splitterz;
pub mod splitty;
pub mod time_split_tracker;
pub mod urn;
pub mod worstrun;
pub mod wsplit;

mod timer_kind;

pub use self::timer_kind::TimerKind;

pub use composite::{parse, parse_and_fix};
