//! The `Settings` module provides the ability to customize a
//! [`Component`](crate::layout::Component) and various other settings.

mod alignment;
mod color;
mod field;
mod font;
mod gradient;
mod image;
mod layout_background;
mod semantic_color;
mod settings_description;
mod value;

pub use self::{
    alignment::Alignment,
    color::Color,
    field::Field,
    font::{Font, Stretch as FontStretch, Style as FontStyle, Weight as FontWeight},
    gradient::{Gradient, ListGradient},
    image::{HasImageId, Image, ImageCache, ImageId},
    layout_background::{BackgroundImage, LayoutBackground, BLUR_FACTOR},
    semantic_color::SemanticColor,
    settings_description::SettingsDescription,
    value::{ColumnKind, Error as ValueError, Result as ValueResult, Value},
};
