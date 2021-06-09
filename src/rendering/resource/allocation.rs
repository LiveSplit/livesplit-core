use super::SharedOwnership;

/// A resource allocator provides all the paths and images necessary to place
/// [`Entities`](super::super::Entity) in a [`Scene`](super::super::Scene). This
/// is usually implemented by a specific renderer where the paths and images are
/// types that the renderer can directly render out.
pub trait ResourceAllocator {
    /// The type the renderer uses for building paths.
    type PathBuilder: PathBuilder<Self, Path = Self::Path>;
    /// The type the renderer uses for paths.
    type Path: SharedOwnership;
    /// The type the renderer uses for textures.
    type Image: SharedOwnership;

    /// Creates a new [`PathBuilder`] to build a new path.
    fn path_builder(&mut self) -> Self::PathBuilder;

    /// Builds a new circle. A default implementation that approximates the
    /// circle with 4 cubic bézier curves is provided. For more accuracy or
    /// performance you can change the implementation.
    fn build_circle(&mut self, x: f32, y: f32, r: f32) -> Self::Path {
        // Based on https://spencermortensen.com/articles/bezier-circle/
        const C: f64 = 0.551915024494;
        let c = (C * r as f64) as f32;
        let mut builder = self.path_builder();
        builder.move_to(x, y - r);
        builder.curve_to(x + c, y - r, x + r, y - c, x + r, y);
        builder.curve_to(x + r, y + c, x + c, y + r, x, y + r);
        builder.curve_to(x - c, y + r, x - r, y + c, x - r, y);
        builder.curve_to(x - r, y - c, x - c, y - r, x, y - r);
        builder.close();
        builder.finish(self)
    }

    /// Instructs the backend to create an image out of the image data provided.
    /// The image's resolution is provided as well. The data is an array of
    /// RGBA8 encoded pixels (red, green, blue, alpha with each channel being an
    /// u8).
    fn create_image(&mut self, width: u32, height: u32, data: &[u8]) -> Self::Image;
}

/// The [`ResourceAllocator`] provides a path builder that defines how to build
/// paths that can be used with the renderer.
pub trait PathBuilder<A: ?Sized> {
    /// The type of the path to build. This needs to be identical to the type of
    /// the path used by the [`ResourceAllocator`].
    type Path;

    /// Moves the cursor to a specific position and starts a new contour.
    fn move_to(&mut self, x: f32, y: f32);

    /// Adds a line from the previous position to the position specified, while
    /// also moving the cursor along.
    fn line_to(&mut self, x: f32, y: f32);

    /// Adds a quadratic bézier curve from the previous position to the position
    /// specified, while also moving the cursor along. (x1, y1) specifies the
    /// control point.
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32);

    /// Adds a cubic bézier curve from the previous position to the position
    /// specified, while also moving the cursor along. (x1, y1) and (x2, y2)
    /// specify the two control points.
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32);

    /// Closes the current contour. The current position and the initial
    /// position get connected by a line, forming a continuous loop. Nothing
    /// if the path is empty or already closed.
    fn close(&mut self);

    /// Finishes building the path.
    fn finish(self, allocator: &mut A) -> Self::Path;
}

pub struct MutPathBuilder<PB>(PB);

impl<A: ResourceAllocator> PathBuilder<&mut A> for MutPathBuilder<A::PathBuilder> {
    type Path = A::Path;

    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to(x, y)
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

    fn finish(self, allocator: &mut &mut A) -> Self::Path {
        self.0.finish(*allocator)
    }
}

impl<A: ResourceAllocator> ResourceAllocator for &mut A {
    type PathBuilder = MutPathBuilder<A::PathBuilder>;
    type Path = A::Path;
    type Image = A::Image;

    fn path_builder(&mut self) -> Self::PathBuilder {
        MutPathBuilder((*self).path_builder())
    }

    fn create_image(&mut self, width: u32, height: u32, data: &[u8]) -> Self::Image {
        (*self).create_image(width, height, data)
    }
}
