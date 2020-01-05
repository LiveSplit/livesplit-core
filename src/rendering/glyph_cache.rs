use super::mesh::{fill_builder, Mesh};
use super::Backend;
use lyon::{
    path::{math::point, Path},
    tessellation::{FillOptions, FillTessellator},
};
use rusttype::{Font, GlyphId, Scale, Segment};
use std::collections::HashMap;

pub struct GlyphCache<M> {
    glyphs: HashMap<GlyphId, M>,
}

impl<M> Default for GlyphCache<M> {
    fn default() -> Self {
        Self {
            glyphs: Default::default(),
        }
    }
}

impl<M> GlyphCache<M> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn lookup_or_insert(
        &mut self,
        font: &Font<'_>,
        glyph: GlyphId,
        backend: &mut impl Backend<Mesh = M>,
    ) -> &M {
        self.glyphs.entry(glyph).or_insert_with(|| {
            let metrics = font.v_metrics(Scale::uniform(1.0));
            let delta_h = metrics.ascent - metrics.descent;
            let offset_h = metrics.descent;

            let glyph = font.glyph(glyph).scaled(Scale::uniform(1.0));
            let mut builder = Path::builder();

            for contour in glyph.shape().into_iter().flatten() {
                for (i, segment) in contour.segments.into_iter().enumerate() {
                    match segment {
                        Segment::Line(line) => {
                            if i == 0 {
                                builder.move_to(point(line.p[0].x, -line.p[0].y));
                            }
                            builder.line_to(point(line.p[1].x, -line.p[1].y));
                        }
                        Segment::Curve(curve) => {
                            if i == 0 {
                                builder.move_to(point(curve.p[0].x, -curve.p[0].y));
                            }
                            builder.quadratic_bezier_to(
                                point(curve.p[1].x, -curve.p[1].y),
                                point(curve.p[2].x, -curve.p[2].y),
                            );
                        }
                    }
                }
                builder.close();
            }

            let path = builder.build();

            let mut glyph_mesh = Mesh::new();
            let mut tessellator = FillTessellator::new();
            tessellator
                .tessellate_path(
                    &path,
                    &FillOptions::tolerance(0.005),
                    &mut fill_builder(&mut glyph_mesh),
                )
                .unwrap();

            for vertex in glyph_mesh.vertices_mut() {
                vertex.v = (-vertex.y - offset_h) * delta_h;
            }

            backend.create_mesh(&glyph_mesh)
        })
    }
}
