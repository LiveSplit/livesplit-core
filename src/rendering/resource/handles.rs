use std::{
    hash::{Hash, Hasher},
    ops::Deref,
};

use super::{PathBuilder, ResourceAllocator, SharedOwnership};

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
        self.next_id += 1;
        Handle { id, inner: element }
    }

    /// Get a reference to the handles's next ID.
    #[allow(clippy::missing_const_for_fn)] // FIXME: Drop is unsupported.
    pub fn into_next_id(self) -> usize {
        self.next_id
    }
}

pub struct HandlePathBuilder<A: ResourceAllocator>(A::PathBuilder);

impl<A: ResourceAllocator> PathBuilder<Handles<A>> for HandlePathBuilder<A> {
    type Path = Handle<A::Path>;

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

    fn finish(self, handles: &mut Handles<A>) -> Self::Path {
        let path = self.0.finish(&mut handles.allocator);
        handles.next(path)
    }
}

impl<A: ResourceAllocator> ResourceAllocator for Handles<A> {
    type PathBuilder = HandlePathBuilder<A>;
    type Path = Handle<A::Path>;
    type Image = Handle<A::Image>;

    fn path_builder(&mut self) -> Self::PathBuilder {
        HandlePathBuilder(self.allocator.path_builder())
    }

    fn build_circle(&mut self, x: f32, y: f32, r: f32) -> Self::Path {
        let circle = self.allocator.build_circle(x, y, r);
        self.next(circle)
    }

    fn create_image(&mut self, width: u32, height: u32, data: &[u8]) -> Self::Image {
        let image = self.allocator.create_image(width, height, data);
        self.next(image)
    }
}

/// A handle can be used to uniquely identify the resource it wraps.
pub struct Handle<T> {
    id: usize,
    inner: T,
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
    pub fn new(id: usize, resource: T) -> Self {
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
