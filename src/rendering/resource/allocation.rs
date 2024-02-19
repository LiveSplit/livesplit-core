use crate::settings::Font;

use super::SharedOwnership;

/// The kind of text that the font is going to be used for.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FontKind {
    /// The font is going to be used for the timer.
    Timer,
    /// The font is going to be used for times.
    Times,
    /// The font is going to be used for regular text.
    Text,
}

impl FontKind {
    /// Returns whether this kind of font is intended to be monospaced. This
    /// usually means the `tabular-nums` variant of the font is supposed to be
    /// activated. If that variant is not available, then it may be necessary to
    /// emulate monospaced digits instead.
    pub fn is_monospaced(self) -> bool {
        self != FontKind::Text
    }
}

/// A resource allocator provides all the paths and images necessary to place
/// [`Entities`](super::super::Entity) in a [`Scene`](super::super::Scene). This
/// is usually implemented by a specific renderer where the paths and images are
/// types that the renderer can directly render out.
pub trait ResourceAllocator {
    /// The type the renderer uses for building paths.
    type PathBuilder: PathBuilder<Path = Self::Path>;
    /// The type the renderer uses for paths.
    type Path: SharedOwnership;
    /// The type the renderer uses for images.
    type Image: SharedOwnership;
    /// The type the renderer uses for fonts.
    type Font;
    /// The type the renderer uses for text labels.
    type Label: Label;

    /// Creates a new [`PathBuilder`] to build a new path.
    fn path_builder(&mut self) -> Self::PathBuilder;

    /// Builds a new circle. A default implementation that approximates the
    /// circle with 4 cubic bézier curves is provided. For more accuracy or
    /// performance you can change the implementation.
    fn build_circle(&mut self, x: f32, y: f32, r: f32) -> Self::Path {
        // Based on https://spencermortensen.com/articles/bezier-circle/
        const A: f64 = 1.00005519;
        const B: f64 = 0.55342686;
        const C: f64 = 0.99873585;

        let a = (A * r as f64) as f32;
        let b = (B * r as f64) as f32;
        let c = (C * r as f64) as f32;

        let mut builder = self.path_builder();
        builder.move_to(x, y + a);
        builder.curve_to(x + b, y + c, x + c, y + b, x + a, y);
        builder.curve_to(x + c, y - b, x + b, y - c, x, y - a);
        builder.curve_to(x - b, y - c, x - c, y - b, x - a, y);
        builder.curve_to(x - c, y + b, x - b, y + c, x, y + a);
        builder.close();
        builder.finish()
    }

    /// Creates an image out of the image data provided. The data represents the
    /// image in its original file format. It needs to be parsed in order to be
    /// visualized. The parsed image as well as the aspect ratio (width /
    /// height) are returned in case the image was parsed successfully.
    fn create_image(&mut self, data: &[u8]) -> Option<(Self::Image, f32)>;

    /// Creates a font from the font description provided. It is expected that
    /// the the font description is used in a font matching algorithm to find
    /// the most suitable font as described in [CSS Fonts Module Level
    /// 3](https://drafts.csswg.org/css-fonts-3/#font-matching-algorithm). The
    /// [`FontKind`] is used to provide additional information about the kind of
    /// font we are looking for. If the font is [`None`] or the font can't be
    /// found, then the default font to use is derived from the [`FontKind`].
    /// Also the timer and times fonts are meant to be monospaced fonts, so
    /// `tabular-nums` are supposed to be used if available and ideally some
    /// sort of emulation should happen if that's not the case. The default text
    /// and times font is provided as [`TEXT_FONT`](super::super::TEXT_FONT) and
    /// the timer's default font is provided as
    /// [`TIMER_FONT`](super::super::TIMER_FONT).
    fn create_font(&mut self, font: Option<&Font>, kind: FontKind) -> Self::Font;

    /// Creates a new text label with the text and font provided. An optional
    /// maximum width is provided as well. If the width of the text measured at
    /// a size of 1 is greater than the maximum width, then it is expected to be
    /// truncated with an ellipsis such that it fits within the maximum width.
    fn create_label(
        &mut self,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) -> Self::Label;

    /// Updates an existing text label with the new text and font provided. An
    /// optional maximum width is provided as well. If the width of the text
    /// measured at a size of 1 is greater than the maximum width, then it is
    /// expected to be truncated with an ellipsis such that it fits within the
    /// maximum width.
    fn update_label(
        &mut self,
        label: &mut Self::Label,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    );
}

/// A text label created by a [`ResourceAllocator`].
pub trait Label: SharedOwnership {
    /// The width of the current text scaled by the scale factor provided.
    fn width(&self, scale: f32) -> f32;

    /// The width of the current text scaled by the scale factor provided as if
    /// it wasn't truncated by the maximum width that was provided.
    fn width_without_max_width(&self, scale: f32) -> f32;
}

/// The [`ResourceAllocator`] provides a path builder that defines how to build
/// paths that can be used with the renderer.
pub trait PathBuilder {
    /// The type of the path to build. This needs to be identical to the type of
    /// the path used by the [`ResourceAllocator`].
    type Path: SharedOwnership;

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
    fn finish(self) -> Self::Path;
}

pub struct MutPathBuilder<PB>(PB);

impl<PB: PathBuilder> PathBuilder for MutPathBuilder<PB> {
    type Path = PB::Path;

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

    fn finish(self) -> Self::Path {
        self.0.finish()
    }
}

impl<A: ResourceAllocator> ResourceAllocator for &mut A {
    type PathBuilder = MutPathBuilder<A::PathBuilder>;
    type Path = A::Path;
    type Image = A::Image;
    type Font = A::Font;
    type Label = A::Label;

    fn path_builder(&mut self) -> Self::PathBuilder {
        MutPathBuilder((*self).path_builder())
    }

    fn create_image(&mut self, data: &[u8]) -> Option<(Self::Image, f32)> {
        (*self).create_image(data)
    }

    fn create_font(&mut self, font: Option<&Font>, kind: FontKind) -> Self::Font {
        (*self).create_font(font, kind)
    }

    fn create_label(
        &mut self,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) -> Self::Label {
        (*self).create_label(text, font, max_width)
    }

    fn update_label(
        &mut self,
        label: &mut Self::Label,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) {
        (*self).update_label(label, text, font, max_width)
    }
}
