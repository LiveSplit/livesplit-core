use core::{
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

use crate::settings::Font;

use super::{Image, Label, PathBuilder, ResourceAllocator, SharedOwnership};

pub struct Handles<A> {
    next_id: usize,
    allocator: A,
}

impl<A> Handles<A> {
    pub const fn new(next_id: usize, allocator: A) -> Self {
        Self { next_id, allocator }
    }

    pub fn next<T>(&mut self, element: T) -> Handle<T> {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        Handle { id, inner: element }
    }

    /// Get the handles's next ID.
    pub fn into_next_id(self) -> usize {
        self.next_id
    }
}

pub struct HandlePathBuilder<PB: PathBuilder>(PB, usize);

impl<PB: PathBuilder> PathBuilder for HandlePathBuilder<PB> {
    type Path = Handle<PB::Path>;

    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to(x, y)
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.0.quad_to(x1, y1, x, y)
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.0.curve_to(x1, y1, x2, y2, x, y)
    }

    fn close(&mut self) {
        self.0.close()
    }

    fn finish(self) -> Self::Path {
        Handle::new(self.1, self.0.finish())
    }
}

impl<A: ResourceAllocator> ResourceAllocator for Handles<A> {
    type PathBuilder = HandlePathBuilder<A::PathBuilder>;
    type Path = Handle<A::Path>;
    type Image = Handle<A::Image>;
    type Font = Handle<A::Font>;
    type Label = LabelHandle<A::Label>;

    fn path_builder(&mut self) -> Self::PathBuilder {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        HandlePathBuilder(self.allocator.path_builder(), id)
    }

    fn build_circle(&mut self, x: f32, y: f32, r: f32) -> Self::Path {
        let circle = self.allocator.build_circle(x, y, r);
        self.next(circle)
    }

    fn build_square(&mut self) -> Self::Path {
        let square = self.allocator.build_square();
        self.next(square)
    }

    fn create_image(&mut self, data: &[u8]) -> Option<Self::Image> {
        let image = self.allocator.create_image(data)?;
        Some(self.next(image))
    }

    fn create_font(&mut self, font: Option<&Font>, kind: super::FontKind) -> Self::Font {
        let font = self.allocator.create_font(font, kind);
        self.next(font)
    }

    fn create_label(
        &mut self,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) -> Self::Label {
        let label = self.allocator.create_label(text, font, max_width);
        LabelHandle {
            update_counter: 0,
            handle: self.next(label),
        }
    }

    fn update_label(
        &mut self,
        label: &mut Self::Label,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) {
        self.allocator
            .update_label(&mut label.handle, text, font, max_width);
        label.update_counter += 1;
    }
}

/// A special handle meant to be used for text labels that also tracks how often
/// the label got updated.
pub struct LabelHandle<L> {
    update_counter: usize,
    handle: Handle<L>,
}

impl<T> Deref for LabelHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl<T> Hash for LabelHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.update_counter.hash(state);
        self.handle.hash(state);
    }
}

impl<L: Label> SharedOwnership for LabelHandle<L> {
    fn share(&self) -> Self {
        LabelHandle {
            update_counter: self.update_counter,
            handle: self.handle.share(),
        }
    }
}

impl<L: Label> Label for LabelHandle<L> {
    fn width(&self, scale: f32) -> f32 {
        self.handle.inner.width(scale)
    }

    fn width_without_max_width(&self, scale: f32) -> f32 {
        self.handle.inner.width_without_max_width(scale)
    }
}

impl<T> Eq for LabelHandle<T> {}

impl<T> PartialEq for LabelHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.update_counter.eq(&other.update_counter) && self.handle.eq(&other.handle)
    }
}

/// A handle can be used to uniquely identify the resource it wraps.
pub struct Handle<T> {
    pub(crate) id: usize,
    inner: T,
}

impl<T: Image> Image for Handle<T> {
    fn aspect_ratio(&self) -> f32 {
        self.inner.aspect_ratio()
    }
}

impl<T: SharedOwnership> SharedOwnership for Handle<T> {
    fn share(&self) -> Self {
        Self {
            id: self.id,
            inner: self.inner.share(),
        }
    }
}

impl<T: SharedOwnership> Handle<T> {
    /// Creates a handle based on some resource that it wraps and a unique ID.
    pub const fn new(id: usize, resource: T) -> Self {
        Self {
            id,
            inner: resource,
        }
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Handle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<T> Eq for Handle<T> {}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
