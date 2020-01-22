//! Provides a software renderer that can be used without a GPU. The rendering
//! is much slower than with a normal GPU, but might be sufficient for
//! situations where you want to create a screenshot of the layout.

#[cfg(test)]
mod tests;

use {
    crate::{
        layout::LayoutState,
        rendering::{Backend, Mesh, Renderer, Rgba as LSColor, Transform},
    },
    derive_more::{Add, Mul},
    euc::{
        buffer::Buffer2d,
        rasterizer::{self, BackfaceCullingDisabled, DepthStrategy},
        Interpolate, Pipeline, Target,
    },
    vek::{Mat3, Rgba, Vec2, Vec3},
};

pub use image::{self, RgbaImage};

struct SoftwareBackend {
    dims: [usize; 2],
    color: AlphaBlended,
}

impl Backend for SoftwareBackend {
    type Mesh = Vec<Vertex>;
    type Texture = Texture;

    fn create_mesh(&mut self, mesh: &Mesh) -> Self::Mesh {
        let vertices = mesh.vertices();
        mesh.indices()
            .iter()
            .map(|&index| {
                let v = vertices[index as usize];
                Vertex {
                    position: Vec2::new(v.x, v.y),
                    texcoord: Vec2::new(v.u, v.v),
                }
            })
            .collect()
    }

    fn render_mesh(
        &mut self,
        mesh: &Self::Mesh,
        transform: Transform,
        [tl, tr, br, bl]: [LSColor; 4],
        texture: Option<&Self::Texture>,
    ) {
        let [x1, y1, z1, x2, y2, z2] = transform.to_column_major_array();
        MyPipeline {
            transform: Mat3::new(x1, x2, 0.0, y1, y2, 0.0, z1, z2, 0.0),
            color_tl: Rgba::new(tl[0], tl[1], tl[2], tl[3]),
            color_tr: Rgba::new(tr[0], tr[1], tr[2], tr[3]),
            color_bl: Rgba::new(bl[0], bl[1], bl[2], bl[3]),
            color_br: Rgba::new(br[0], br[1], br[2], br[3]),
            texture,
        }
        .draw::<rasterizer::Triangles<'_, _, BackfaceCullingDisabled>, _>(
            mesh,
            &mut self.color,
            &mut NoDepth(self.dims),
        );
    }

    fn free_mesh(&mut self, _: Self::Mesh) {}

    fn create_texture(&mut self, width: u32, height: u32, data: &[u8]) -> Self::Texture {
        Texture {
            data: data.to_owned(),
            width: width as f32,
            height: height as f32,
            stride: width as usize * 4,
        }
    }
    fn free_texture(&mut self, _: Self::Texture) {}

    fn resize(&mut self, _: f32, _: f32) {}
}

struct NoDepth([usize; 2]);

impl Target for NoDepth {
    type Item = f32;

    fn size(&self) -> [usize; 2] {
        self.0
    }

    unsafe fn set(&mut self, _pos: [usize; 2], _item: Self::Item) {}

    unsafe fn get(&self, _pos: [usize; 2]) -> Self::Item {
        1.0
    }

    fn clear(&mut self, _fill: Self::Item) {}
}

struct AlphaBlended(Buffer2d<Rgba<f32>>);

impl Target for AlphaBlended {
    type Item = Rgba<f32>;

    fn size(&self) -> [usize; 2] {
        self.0.size()
    }

    unsafe fn set(&mut self, pos: [usize; 2], src: Self::Item) {
        debug_assert!(!src.r.is_nan());
        debug_assert!(!src.g.is_nan());
        debug_assert!(!src.b.is_nan());
        debug_assert!(!src.a.is_nan());

        let dst = self.0.get(pos);
        self.0.set(
            pos,
            Rgba::new(
                src.a * src.r + (1.0 - src.a) * dst.r,
                src.a * src.g + (1.0 - src.a) * dst.g,
                src.a * src.b + (1.0 - src.a) * dst.b,
                src.a + (1.0 - src.a) * dst.a,
            ),
        );
    }

    unsafe fn get(&self, pos: [usize; 2]) -> Self::Item {
        self.0.get(pos)
    }

    fn clear(&mut self, fill: Self::Item) {
        self.0.clear(fill)
    }
}

struct Texture {
    data: Vec<u8>,
    width: f32,
    height: f32,
    stride: usize,
}

struct MyPipeline<'a> {
    transform: Mat3<f32>,
    color_tl: Rgba<f32>,
    color_tr: Rgba<f32>,
    color_bl: Rgba<f32>,
    color_br: Rgba<f32>,
    texture: Option<&'a Texture>,
}

struct Vertex {
    position: Vec2<f32>,
    texcoord: Vec2<f32>,
}

#[derive(Clone, Mul, Add)]
struct VsOut {
    color: Rgba<f32>,
    texcoord: Vec2<f32>,
}

impl Interpolate for VsOut {
    #[inline(always)]
    fn lerp2(a: Self, b: Self, x: f32, y: f32) -> Self {
        a * x + b * y
    }
    #[inline(always)]
    fn lerp3(a: Self, b: Self, c: Self, x: f32, y: f32, z: f32) -> Self {
        a * x + b * y + c * z
    }
}

impl Pipeline for MyPipeline<'_> {
    type Vertex = Vertex;
    type VsOut = VsOut;
    type Pixel = Rgba<f32>;

    fn vert(&self, vertex: &Self::Vertex) -> ([f32; 4], Self::VsOut) {
        let left = self.color_tl * (1.0 - vertex.texcoord.y) + self.color_bl * vertex.texcoord.y;
        let right = self.color_tr * (1.0 - vertex.texcoord.y) + self.color_br * vertex.texcoord.y;
        let color = left * (1.0 - vertex.texcoord.x) + right * vertex.texcoord.x;

        let pos = Vec3::new(vertex.position.x, vertex.position.y, 1.0) * self.transform;

        (
            [2.0 * pos.x - 1.0, -2.0 * pos.y + 1.0, 0.0, 1.0],
            VsOut {
                color,
                texcoord: vertex.texcoord,
            },
        )
    }

    fn frag(&self, vsout: &Self::VsOut) -> Self::Pixel {
        if let Some(texture) = self.texture {
            let x = vsout.texcoord.x * texture.width;
            let y = vsout.texcoord.y * texture.height;
            let pixel = &texture.data[texture.stride * y as usize + x as usize * 4..];
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let a = pixel[3];
            return Rgba::new(
                f32::from(r) / 255.0,
                f32::from(g) / 255.0,
                f32::from(b) / 255.0,
                f32::from(a) / 255.0,
            ) * vsout.color;
        }
        vsout.color
    }

    fn get_depth_strategy(&self) -> DepthStrategy {
        DepthStrategy::None
    }
}

/// Renders the layout state provided into an image of the selected resolution.
/// The final render will have pixelated edges as there is not going to be any
/// anti aliasing. Use [`render_anti_aliased`] if you want it to be anti
/// aliased. Note that this is software rendered and thus will be much slower
/// than rendering on the GPU.
///
/// [`render_anti_aliased`]: fn.render_anti_aliased.html
pub fn render(state: &LayoutState, [width, height]: [usize; 2]) -> RgbaImage {
    let mut backend = SoftwareBackend {
        color: AlphaBlended(Buffer2d::new(
            [width, height],
            Rgba::new(0.0, 0.0, 0.0, 0.0),
        )),
        dims: [width, height],
    };

    Renderer::new().render(&mut backend, (width as _, height as _), &state);

    let mut buf = Vec::with_capacity(width * height * 4);
    for pixel in backend.color.0.as_ref() {
        buf.extend(
            pixel
                .map(|e| (e * 255.0) as u8)
                .into_array()
                .iter()
                .cloned(),
        )
    }

    RgbaImage::from_raw(width as _, height as _, buf).unwrap()
}

/// Renders the layout state provided into an image of the selected resolution.
/// The `samples_sqrt` argument is the square root of the amount of samples used
/// for anti aliasing the final image. Note that this is software rendered and
/// thus will be much slower than rendering on the GPU.
pub fn render_anti_aliased(
    state: &LayoutState,
    [width, height]: [usize; 2],
    samples_sqrt: usize,
) -> RgbaImage {
    let image = render(state, [width * samples_sqrt, height * samples_sqrt]);
    image::imageops::thumbnail(&image, width as u32, height as u32)
}
