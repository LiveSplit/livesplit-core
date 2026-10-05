//! A Carousel Component cycles through its child components one at a time,
//! periodically switching to the next child after a configurable interval.

use crate::component::OwnedComponent;
use livesplit_core::component::carousel::Component as CarouselComponent;

/// type
pub type OwnedCarouselComponent = Box<CarouselComponent>;

/// Creates a new empty Carousel Component.
#[unsafe(no_mangle)]
pub extern "C" fn CarouselComponent_new() -> OwnedCarouselComponent {
    Box::new(CarouselComponent::new())
}

/// drop
#[unsafe(no_mangle)]
pub extern "C" fn CarouselComponent_drop(this: OwnedCarouselComponent) {
    drop(this);
}

/// Converts the Carousel Component into a generic component suitable for using
/// with a layout.
#[unsafe(no_mangle)]
pub extern "C" fn CarouselComponent_into_generic(this: OwnedCarouselComponent) -> OwnedComponent {
    Box::new((*this).into())
}

/// Adds a component to the end of the carousel.
#[unsafe(no_mangle)]
pub extern "C" fn CarouselComponent_add_component(
    this: &mut CarouselComponent,
    component: OwnedComponent,
) {
    this.components.push(*component);
}

/// Returns the number of components in the carousel.
#[unsafe(no_mangle)]
pub extern "C" fn CarouselComponent_len(this: &CarouselComponent) -> usize {
    this.components.len()
}

/// Returns the size override of the carousel. 0xFFFFFFFF means automatic
/// sizing.
#[unsafe(no_mangle)]
pub extern "C" fn CarouselComponent_size(this: &CarouselComponent) -> u32 {
    this.size.unwrap_or(u32::MAX)
}

/// Sets the size override of the carousel. 0xFFFFFFFF means automatic
/// sizing.
#[unsafe(no_mangle)]
pub extern "C" fn CarouselComponent_set_size(this: &mut CarouselComponent, size: u32) {
    this.size = if size != u32::MAX { Some(size) } else { None };
}

/// Returns the interval in seconds between switching to the next child.
#[unsafe(no_mangle)]
pub extern "C" fn CarouselComponent_interval(this: &CarouselComponent) -> u64 {
    this.interval_seconds
}

/// Sets the interval in seconds between switching to the next child.
#[unsafe(no_mangle)]
pub extern "C" fn CarouselComponent_set_interval(this: &mut CarouselComponent, interval: u64) {
    this.interval_seconds = if interval > 0 { interval } else { 5 };
}
