use super::{
    entity::{calculate_hash, Entity},
    resource::{Handle, SharedOwnership},
    FillShader,
};
use crate::platform::prelude::*;

/// Describes a layer of a [`Scene`] to place an [`Entity`] on.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Layer {
    /// The bottom layer is the layer where all the less frequently changing
    /// [`Entities`](Entity) are being placed on.
    Bottom,
    /// The top layer is the layer where all the [`Entities`](Entity) that are
    /// expected to frequently change are being placed on.
    Top,
}

impl Layer {
    /// Returns the appropriate layer to use depending on whether the [`Entity`]
    /// to place updates frequently or not.
    pub const fn from_updates_frequently(updates_frequently: bool) -> Self {
        match updates_frequently {
            false => Self::Bottom,
            true => Self::Top,
        }
    }
}

/// A scene describes all the [`Entities`](Entity) to visualize. It consists of
/// two [`Layers`](Layer) that are supposed to be composited on top of each
/// other. The bottom [`Layer`] changes infrequently and doesn't need to be
/// rerendered for most frames. The top [`Layer`] contains all the per frame
/// changes and needs to be rerendered for every frame. If however it is empty
/// and both the bottom layer didn't change, then no new frame needs to be
/// rendered. While the top [`Layer`] is inherently transparent, the bottom
/// [`Layer`] has a background that needs to be considered.
pub struct Scene<P, I, L> {
    rectangle: Handle<P>,
    background: Option<FillShader>,
    bottom_hash: u64,
    bottom_layer_changed: bool,
    bottom_layer: Vec<Entity<P, I, L>>,
    top_layer: Vec<Entity<P, I, L>>,
}

impl<P: SharedOwnership, I: SharedOwnership, L: SharedOwnership> Scene<P, I, L> {
    /// Creates a new scene with the rectangle provided to use for placing
    /// rectangle entities.
    pub fn new(rectangle: Handle<P>) -> Self {
        Self {
            rectangle,
            background: None,
            bottom_hash: calculate_hash::<P, I, L>(&None, &[]),
            bottom_layer_changed: false,
            bottom_layer: Vec::new(),
            top_layer: Vec::new(),
        }
    }

    /// Get a reference to the bottom [`Layer's`](Layer) background. While the
    /// top [`Layer`] is inherently transparent, the bottom [`Layer`] has a
    /// background that needs to be considered.
    pub fn background(&self) -> &Option<FillShader> {
        &self.background
    }

    /// Check if the scene's bottom [`Layer`] changed. Use this method to check
    /// if the bottom [`Layer`] needs to be rerendered. If the background of the
    /// bottom [`Layer`] changes this also returns `true`, so the background
    /// doesn't need to manually be compared.
    pub fn bottom_layer_changed(&self) -> bool {
        self.bottom_layer_changed
    }

    /// Get a reference to the scene's bottom [`Layer`]. This [`Layer`] is
    /// intended to infrequently change, so it doesn't need to be rerendered
    /// every frame.
    pub fn bottom_layer(&self) -> &[Entity<P, I, L>] {
        &self.bottom_layer
    }

    /// Get a reference to the scene's top [`Layer`].
    pub fn top_layer(&self) -> &[Entity<P, I, L>] {
        &self.top_layer
    }

    /// Get access to the rectangle resource the scene stores.
    pub fn rectangle(&self) -> Handle<P> {
        self.rectangle.share()
    }

    /// Set the bottom [`Layer's`](Layer) background.
    pub fn set_background(&mut self, background: Option<FillShader>) {
        self.background = background;
    }

    /// Get a mutable reference to the scene's bottom [`Layer`].
    pub fn bottom_layer_mut(&mut self) -> &mut Vec<Entity<P, I, L>> {
        &mut self.bottom_layer
    }

    /// Get a mutable reference to the scene's top [`Layer`].
    pub fn top_layer_mut(&mut self) -> &mut Vec<Entity<P, I, L>> {
        &mut self.top_layer
    }

    /// Clears all the [`Layers`](Layer) such that no [`Entities`](Entity) are
    /// left.
    pub fn clear(&mut self) {
        self.bottom_layer.clear();
        self.top_layer.clear();
    }

    /// Recalculates the hash of the bottom [`Layer`] and checks if it changed.
    /// The bottom [`Layer`] is intended to infrequently change, such that it
    /// doesn't need to be rerendered all the time.
    pub fn recalculate_if_bottom_layer_changed(&mut self) {
        let new_hash = calculate_hash(&self.background, &self.bottom_layer);
        self.bottom_layer_changed = new_hash != self.bottom_hash;
        self.bottom_hash = new_hash;
    }

    /// Accesses the [`Layer`] specified mutably.
    pub fn layer_mut(&mut self, layer: Layer) -> &mut Vec<Entity<P, I, L>> {
        match layer {
            Layer::Bottom => &mut self.bottom_layer,
            Layer::Top => &mut self.top_layer,
        }
    }
}
