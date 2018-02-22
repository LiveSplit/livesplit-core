//! The saver module provides all the different ways to save Runs as splits
//! files.
//!
//! # Examples
//!
//! Using the LiveSplit Saver to save a Run as a LiveSplit splits file.
//!
//! ```no_run
//! use livesplit_core::run::saver::livesplit;
//! use livesplit_core::{Run, Segment};
//! use std::fs::File;
//! use std::io::BufWriter;
//!
//! // Create a run object that we can use.
//! let mut run = Run::new();
//! run.set_game_name("Super Mario Odyssey");
//! run.set_category_name("Any%");
//! run.push_segment(Segment::new("Cap Kingdom"));
//!
//! // Create the splits file.
//! let file = File::create("path/to/splits_file.lss");
//! let writer = BufWriter::new(file.expect("Failed creating the file"));
//!
//! // Save the splits file as a LiveSplit splits file.
//! livesplit::save_run(&run, writer).expect("Couldn't save the splits file");
//! ```

pub mod livesplit;
