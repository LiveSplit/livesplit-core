//! Interacting with the settings of an auto splitter.
//!
//! # Overview
//!
//! Settings consist of two parts. One part is the settings [`Widgets`](Widget),
//! that are used to let the user configure the settings. The other part is the
//! settings [`Values`](Value) that are actually stored in the splits file.
//! Those settings don't necessarily correlate entirely with the settings
//! [`Widgets`](Widget), because the stored splits might either be from a
//! different version of the auto splitter or contain additional information
//! such as the version of the settings, that the user doesn't necessarily
//! directly interact with. These stored settings are available as the global
//! settings [`Map`], which can be loaded, modified and stored freely. The keys
//! used for the settings widgets directly correlate with the keys used in the
//! settings [`Map`]. Any changes in the settings [`Widgets`](Widget) will
//! automatically be reflected in the global settings [`Map`] and vice versa.

mod gui;
mod list;
mod map;
mod value;

pub use gui::*;
pub use list::*;
pub use map::*;
pub use value::*;
