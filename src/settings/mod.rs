//! The settings module provides the ability to customize components and various
//! other settings.

mod alignment;
mod color;
mod field;
mod font;
mod gradient;
mod image;
mod semantic_color;
mod settings_description;
mod value;

pub use self::{
    alignment::Alignment,
    color::Color,
    field::Field,
    font::{Font, Stretch as FontStretch, Style as FontStyle, Weight as FontWeight},
    gradient::{Gradient, ListGradient},
    image::{CachedImageId, Image, ImageData},
    semantic_color::SemanticColor,
    settings_description::SettingsDescription,
    value::{ColumnKind, Error as ValueError, Result as ValueResult, Value},
};
