//! Provides a renderer for the web that renders into a canvas. The element can
//! then be attached anywhere in the DOM with any desired positioning and size.

use bytemuck::cast;
use hashbrown::HashMap;
use js_sys::{Array, JsString, Uint8Array};
use std::{
    array,
    cell::{Cell, RefCell},
    f64::consts::TAU,
    ops::Deref,
    rc::Rc,
    str,
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, Element, HtmlCanvasElement, ImageBitmap, Path2d, Window};

use crate::{
    layout::LayoutState,
    settings::{Font, FontStretch, FontStyle, FontWeight, ImageCache, BLUR_FACTOR},
};

use self::bindings::CanvasRenderingContext2d;

use super::{
    Background, Entity, FillShader, FontKind, Label, PathBuilder, ResourceAllocator, SceneManager,
    SharedOwnership, Transform,
};

mod bindings;

// FIXME: The fonts should really be sized 1px, because the actual positioning
// and scaling is done through the transformation matrix. However Safari and
// especially Firefox have a lot of issues rendering text that way. They assume
// the font is actually 1px and then scale it up, causing COLRv1 emojis to be
// rendered at 1x1 resolution. Additionally kerning might mess up on other
// instances of Firefox and Safari seems to force a minimum size on the emojis.
// This is why we specify the fonts to be at 100px, which is a decent resolution
// for the emojis and then scale everything back down. Check this fiddle to see
// if the problem still exists: (Firefox on Windows 11, iOS Safari)
// https://jsfiddle.net/7y4barmq/
const FONT_SCALE_FACTOR: f32 = 0.01;

struct CanvasPathBuilder {
    path: Path2d,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}

impl CanvasPathBuilder {
    fn update_x(&mut self, x: f32) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
    }

    fn update_y(&mut self, y: f32) {
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
    }

    fn circle(&mut self, x: f32, y: f32, r: f32) {
        self.update_x(x - r);
        self.update_x(x + r);
        self.update_y(y - r);
        self.update_y(y + r);
        let _ = self.path.arc(x as _, y as _, r as _, 0.0, TAU);
    }
}

struct CanvasAllocator {
    window: Window,
    force_redraw_all: Rc<Cell<bool>>,
    ctx_bottom: CanvasRenderingContext2d,
    ctx_top: CanvasRenderingContext2d,
    digits: [JsString; 10],
}

#[derive(Clone)]
struct Path(Rc<CanvasPathBuilder>);

#[derive(Clone)]
struct Image(Rc<RefCell<Option<(ImageBitmap, f32)>>>);

impl super::Image for Image {
    fn aspect_ratio(&self) -> f32 {
        self.0.borrow().as_ref().map_or(1.0, |(_, ratio)| *ratio)
    }
}

impl SharedOwnership for Image {
    fn share(&self) -> Self {
        self.clone()
    }
}

impl Deref for Path {
    type Target = Rc<CanvasPathBuilder>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SharedOwnership for Path {
    fn share(&self) -> Self {
        self.clone()
    }
}

impl PathBuilder for CanvasPathBuilder {
    type Path = Path;

    fn move_to(&mut self, x: f32, y: f32) {
        self.update_x(x);
        self.update_y(y);
        self.path.move_to(x as _, y as _);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.update_x(x);
        self.update_y(y);
        self.path.line_to(x as _, y as _);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        // FIXME: Calculate the actual bezier curve's bounds. Should affect the
        // other renderers too.
        self.update_x(x1);
        self.update_y(y1);
        self.update_x(x);
        self.update_y(y);
        self.path
            .quadratic_curve_to(x1 as _, y1 as _, x as _, y as _);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.update_x(x1);
        self.update_y(y1);
        self.update_x(x2);
        self.update_y(y2);
        self.update_x(x);
        self.update_y(y);
        self.path
            .bezier_curve_to(x1 as _, y1 as _, x2 as _, y2 as _, x as _, y as _);
    }

    fn close(&mut self) {
        self.path.close_path();
    }

    fn finish(self) -> Self::Path {
        Path(Rc::new(self))
    }
}

struct CanvasLabelInner {
    font: Rc<CanvasFont>,
    shape: LabelShape,
    width: f32,
    width_without_max_width: f32,
}

impl Default for CanvasLabelInner {
    fn default() -> Self {
        Self {
            font: Rc::new(CanvasFont {
                descriptor: "".into(),
                font_handling: FontHandling::Normal,
                font_kerning: "".into(),
                top: 0.0,
                bottom: 0.0,
            }),
            shape: LabelShape::Normal("".into()),
            width: 0.0,
            width_without_max_width: 0.0,
        }
    }
}

#[derive(Default)]
struct CanvasLabel(Rc<RefCell<CanvasLabelInner>>);

impl SharedOwnership for CanvasLabel {
    fn share(&self) -> Self {
        Self(self.0.share())
    }
}

impl Label for CanvasLabel {
    fn width(&self, scale: f32) -> f32 {
        self.0.borrow().width * scale
    }

    fn width_without_max_width(&self, scale: f32) -> f32 {
        self.0.borrow().width_without_max_width * scale
    }
}

struct CanvasFont {
    descriptor: JsString,
    font_kerning: JsString,
    font_handling: FontHandling,
    top: f32,
    bottom: f32,
}

enum FontHandling {
    Normal,
    MonospaceEmulation(MonospaceInfo),
}

struct MonospaceInfo {
    digit_offsets: [f32; 10],
    digit_width: f32,
}

impl ResourceAllocator for CanvasAllocator {
    type PathBuilder = CanvasPathBuilder;
    type Path = Path;
    type Image = Image;
    type Font = Rc<CanvasFont>;
    type Label = CanvasLabel;

    fn path_builder(&mut self) -> Self::PathBuilder {
        CanvasPathBuilder {
            path: Path2d::new().unwrap(),
            min_x: f32::INFINITY,
            max_x: f32::NEG_INFINITY,
            min_y: f32::INFINITY,
            max_y: f32::NEG_INFINITY,
        }
    }

    fn create_image(&mut self, data: &[u8]) -> Option<Self::Image> {
        if data.is_empty() {
            return None;
        }
        let slot = Rc::new(RefCell::new(None));

        // SAFETY: There is no allocation happening that would invalidate the
        // view. The view is immediately given to the blob, which creates an
        // internal copy of the data.
        let blob = unsafe {
            let parts = Array::of1(&Uint8Array::view(data).into());
            Blob::new_with_u8_array_sequence(&parts).ok()?
        };

        if let Ok(promise) = self.window.create_image_bitmap_with_blob(&blob) {
            let future = JsFuture::from(promise);
            let slot = slot.clone();
            let force_redraw_all = self.force_redraw_all.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(image) = future.await {
                    let image: ImageBitmap = image.unchecked_into();
                    let aspect_ratio = image.width() as f32 / image.height() as f32;
                    *slot.borrow_mut() = Some((image, aspect_ratio));
                    force_redraw_all.set(true);
                }
            });
        }
        Some(Image(slot))
    }

    fn create_font(&mut self, font: Option<&Font>, kind: FontKind) -> Self::Font {
        let mut descriptor = String::new();
        if let Some(font) = font {
            match font.style {
                FontStyle::Normal => {}
                FontStyle::Italic => descriptor.push_str("italic "),
                FontStyle::Oblique => descriptor.push_str("oblique "),
            }
            if font.weight != FontWeight::Normal {
                descriptor.push_str(weight_as_css_str(font.weight));
                descriptor.push(' ');
            }
            if font.stretch != FontStretch::Normal {
                descriptor.push_str(stretch_as_css_str(font.stretch));
                descriptor.push(' ');
            }
            descriptor.push_str("100px \"");
            descriptor.push_str(&font.family);
            descriptor.push_str("\", ");
        } else {
            if kind == FontKind::Times {
                descriptor.push_str("700 ");
            }
            descriptor.push_str("100px ");
        }
        match kind {
            FontKind::Timer => descriptor.push_str("\"timer\", monospace"),
            FontKind::Text => descriptor.push_str("\"fira\", sans-serif"),
            FontKind::Times => descriptor.push_str("\"fira\", monospace"),
        }

        let is_monospaced = kind.is_monospaced();
        let font_kerning = JsString::from(if is_monospaced { "none" } else { "auto" });
        let descriptor = JsString::from(descriptor);
        self.ctx_top.set_font(&descriptor);
        self.ctx_top.set_font_kerning(&font_kerning);

        // FIXME: We query this to position a gradient from the top to the
        // bottom of the font. Is the ascent and descent what we want here?
        // That's not what we do in our default text engine.
        let metrics = self.ctx_top.measure_text(&JsString::from("")).unwrap();
        let top = -metrics.font_bounding_box_ascent() as f32;
        let bottom = metrics.font_bounding_box_descent() as f32;

        let font_handling = if is_monospaced {
            let mut digit_offsets = [0.0; 10];
            let mut digit_width = 0.0;

            for (digit, glyph) in digit_offsets.iter_mut().enumerate() {
                let metrics = self.ctx_top.measure_text(&self.digits[digit]).unwrap();
                let width = metrics.width() as f32;
                *glyph = width;
                if width > digit_width {
                    digit_width = width;
                }
            }

            if digit_offsets.iter().all(|&v| v == digit_width) {
                // If all digits have the same width, there's no need to emulate
                // monospacing.
                FontHandling::Normal
            } else {
                for digit_offset in &mut digit_offsets {
                    *digit_offset = 0.5 * (digit_width - *digit_offset);
                }
                FontHandling::MonospaceEmulation(MonospaceInfo {
                    digit_offsets,
                    digit_width,
                })
            }
        } else {
            FontHandling::Normal
        };

        Rc::new(CanvasFont {
            descriptor,
            font_kerning,
            font_handling,
            top,
            bottom,
        })
    }

    fn create_label(
        &mut self,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) -> Self::Label {
        let mut label = CanvasLabel::default();
        self.update_label(&mut label, text, font, max_width);
        label
    }

    fn update_label(
        &mut self,
        label: &mut Self::Label,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) {
        let mut label = label.0.borrow_mut();
        let label = &mut *label;
        set_font(&self.ctx_top, font);

        let (shape, width) = font.font_handling.shape(text, &self.ctx_top);
        label.width_without_max_width = width;
        label.width = label.width_without_max_width;
        label.shape = shape;

        // FIXME: Pop from the existing shape instead.
        if let Some(max_width) = max_width {
            if label.width > max_width {
                let mut text = text.to_owned();
                while let Some((drain_index, _)) = text.char_indices().next_back() {
                    text.drain(drain_index..);
                    text.push('…');
                    let (shape, width) = font.font_handling.shape(&text, &self.ctx_top);
                    label.shape = shape;
                    label.width = width;
                    if label.width <= max_width {
                        break;
                    } else {
                        const ELLIPSIS_LEN: usize = '…'.len_utf8();
                        text.drain(text.len() - ELLIPSIS_LEN..);
                    }
                }
            }
        }

        label.font = font.clone();
    }

    fn build_circle(&mut self, x: f32, y: f32, r: f32) -> Self::Path {
        let mut builder = self.path_builder();
        builder.circle(x, y, r);
        builder.finish()
    }
}

enum MonospacePiece {
    Digit { digit: u8, offset: f32 },
    Chunk { chunk: JsString, offset: f32 },
}

enum LabelShape {
    Normal(JsString),
    // FIXME: Switch to tabular-nums if that ever becomes a thing for Canvas.
    MonospaceEmulation(Vec<MonospacePiece>),
}

impl FontHandling {
    fn shape(&self, text: &str, ctx: &CanvasRenderingContext2d) -> (LabelShape, f32) {
        match self {
            FontHandling::Normal => {
                let text = JsString::from(text);
                let metrics = ctx.measure_text(&text).unwrap();
                let width = metrics.width() as f32;
                (LabelShape::Normal(text), width * FONT_SCALE_FACTOR)
            }
            FontHandling::MonospaceEmulation(info) => {
                let mut shaped = Vec::new();
                let mut rem = text;
                let mut offset = 0.0;
                while !rem.is_empty() {
                    let digit_pos = rem
                        .bytes()
                        .position(|b| b.is_ascii_digit())
                        .unwrap_or(rem.len());

                    if digit_pos > 0 {
                        let chunk = JsString::from(&rem[..digit_pos]);
                        let metrics = ctx.measure_text(&chunk).unwrap();
                        let width = metrics.width() as f32;
                        shaped.push(MonospacePiece::Chunk { chunk, offset });
                        offset += width;
                    }

                    if digit_pos < rem.len() {
                        let digit = rem.as_bytes()[digit_pos];
                        let digit = digit - b'0';
                        shaped.push(MonospacePiece::Digit {
                            digit,
                            offset: offset + info.digit_offsets[digit as usize],
                        });
                        offset += info.digit_width;
                        rem = &rem[digit_pos + 1..];
                    } else {
                        rem = "";
                    }
                }

                (
                    LabelShape::MonospaceEmulation(shaped),
                    offset * FONT_SCALE_FACTOR,
                )
            }
        }
    }
}

/// The web renderer renders into a canvas element. The element can then be
/// attached anywhere in the DOM with any desired positioning and size.
pub struct Renderer {
    manager: SceneManager<Path, Image, Rc<CanvasFont>, CanvasLabel>,
    div: Element,
    allocator: CanvasAllocator,
    canvas_bottom: HtmlCanvasElement,
    canvas_top: HtmlCanvasElement,
    str_buf: String,
    top_layer_is_cleared: bool,
    cache: HashMap<HashShader, JsValue>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    /// Creates a new web renderer that renders into a canvas element. The
    /// element can then be attached anywhere in the DOM with any desired
    /// positioning and size. There are two CSS fonts that are used as the
    /// default fonts. They are called "timer" and "fira". Make sure they are
    /// fully loaded before creating the renderer as otherwise information about
    /// a fallback font is cached instead.
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let div = document.create_element("div").unwrap();

        let canvas_bottom: HtmlCanvasElement =
            document.create_element("canvas").unwrap().unchecked_into();

        let canvas_top: HtmlCanvasElement =
            document.create_element("canvas").unwrap().unchecked_into();

        canvas_bottom
            .set_attribute(
                "style",
                "position: absolute; width: inherit; height: inherit;",
            )
            .unwrap();
        canvas_top
            .set_attribute(
                "style",
                "position: absolute; width: inherit; height: inherit;",
            )
            .unwrap();

        div.append_with_node_2(&canvas_bottom, &canvas_top).unwrap();

        let ctx_bottom: CanvasRenderingContext2d = canvas_bottom
            .get_context("2d")
            .unwrap()
            .unwrap()
            .unchecked_into();

        let ctx_top: CanvasRenderingContext2d = canvas_top
            .get_context("2d")
            .unwrap()
            .unwrap()
            .unchecked_into();

        let force_redraw_all = Rc::new(Cell::new(false));

        let mut allocator = CanvasAllocator {
            window,
            force_redraw_all,
            ctx_bottom,
            ctx_top,
            digits: array::from_fn(|digit| JsString::from((digit as u8 + b'0') as char)),
        };

        Self {
            manager: SceneManager::new(&mut allocator),
            div,
            allocator,
            canvas_bottom,
            canvas_top,
            str_buf: String::new(),
            top_layer_is_cleared: true,
            cache: HashMap::new(),
        }
    }

    /// Returns the HTML element. This can be attached anywhere in the DOM with
    /// any desired positioning and size.
    pub const fn element(&self) -> &Element {
        &self.div
    }

    /// Renders the layout state into the canvas. The image cache is used to
    /// retrieve images that are used in the layout state.
    pub fn render(&mut self, state: &LayoutState, image_cache: &ImageCache) -> Option<[f32; 2]> {
        // Scaling is based on:
        // https://webglfundamentals.org/webgl/lessons/webgl-resizing-the-canvas.html

        let ratio = self.allocator.window.device_pixel_ratio();
        let bounding_rect = self.canvas_bottom.get_bounding_client_rect();
        let [bounding_width, bounding_height] = [bounding_rect.width(), bounding_rect.height()];

        // If the width or height is 0, the element likely isn't mounted into
        // the DOM. This usually isn't a problem, but if we end up resizing the
        // layout, we would resize it based on a size of 0, which would break
        // the entire layout. This happened here:
        //
        // https://github.com/LiveSplit/LiveSplitOne/issues/881
        //
        // Rendering the layout with a size of 0 is also a waste of time, so
        // this ends up benefiting us in multiple ways.
        if bounding_width == 0.0 || bounding_height == 0.0 {
            return None;
        }

        let [width, height] = [
            (ratio * bounding_width).round(),
            (ratio * bounding_height).round(),
        ];

        if (self.canvas_bottom.width(), self.canvas_bottom.height()) != (width as _, height as _) {
            self.canvas_bottom.set_width(width as _);
            self.canvas_bottom.set_height(height as _);
            self.canvas_top.set_width(width as _);
            self.canvas_top.set_height(height as _);
        }

        let new_dims = self.manager.update_scene(
            &mut self.allocator,
            [width as _, height as _],
            state,
            image_cache,
        );

        let scene = self.manager.scene();
        let str_buf = &mut self.str_buf;

        if scene.bottom_layer_changed() || self.allocator.force_redraw_all.take() {
            let ctx = &mut self.allocator.ctx_bottom;
            let _ = ctx.reset_transform();
            ctx.clear_rect(0.0, 0.0, width, height);
            if let Some(background) = scene.background() {
                match background {
                    Background::Shader(shader) => {
                        set_fill_style(shader, ctx, &mut self.cache, str_buf, &*scene.rectangle());
                        // Instead of scaling the rectangle we need to use a
                        // transform so that the gradient's endpoints are
                        // correct.
                        set_transform(
                            ctx,
                            &Transform {
                                x: 0.0,
                                y: 0.0,
                                scale_x: width as _,
                                scale_y: height as _,
                            },
                        );
                        ctx.fill_rect(0.0, 0.0, 1.0, 1.0);
                    }
                    Background::Image(background_image, transform) => {
                        let image = background_image.image.0.borrow();
                        if let Some((image, _)) = &*image {
                            str_buf.clear();
                            use std::fmt::Write;
                            if background_image.brightness != 1.0 {
                                let _ = write!(
                                    str_buf,
                                    "brightness({}%)",
                                    100.0 * background_image.brightness
                                );
                            }
                            if background_image.opacity != 1.0 {
                                if !str_buf.is_empty() {
                                    str_buf.push(' ');
                                }
                                let _ = write!(
                                    str_buf,
                                    "opacity({}%)",
                                    100.0 * background_image.opacity
                                );
                            }
                            if background_image.blur != 0.0 {
                                if !str_buf.is_empty() {
                                    str_buf.push(' ');
                                }
                                let _ = write!(
                                    str_buf,
                                    "blur({}px)",
                                    (BLUR_FACTOR as f64)
                                        * background_image.blur as f64
                                        * width.max(height)
                                );
                            }
                            if !str_buf.is_empty() {
                                // FIXME: Cache the string (and below for the none).
                                ctx.set_filter(&JsString::from(str_buf.as_str()));
                            }
                            let _ = ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                                image,
                                transform.x as _,
                                transform.y as _,
                                transform.scale_x as _,
                                transform.scale_y as _,
                            );
                            if !str_buf.is_empty() {
                                ctx.set_filter(&JsString::from("none"));
                            }
                        }
                    }
                }
            }

            render_layer(
                ctx,
                &mut self.cache,
                str_buf,
                scene.bottom_layer(),
                &self.allocator.digits,
            );
        }

        let layer = scene.top_layer();
        let ctx = &mut self.allocator.ctx_top;

        if layer.is_empty() <= !self.top_layer_is_cleared {
            let _ = ctx.reset_transform();
            ctx.clear_rect(0.0, 0.0, width, height);
        }
        self.top_layer_is_cleared = layer.is_empty();
        render_layer(ctx, &mut self.cache, str_buf, layer, &self.allocator.digits);

        new_dims.map(|[width, height]| {
            let ratio = (1.0 / ratio) as f32;
            [width * ratio, height * ratio]
        })
    }
}

#[derive(PartialEq, Eq, Hash)]
enum HashShader {
    SolidColor([u32; 4]),
    VerticalGradient([u32; 4], [u32; 4], [u32; 2]),
    HorizontalGradient([u32; 4], [u32; 4], [u32; 2]),
}

trait HasBounds {
    fn bounds_x(&self) -> [f32; 2];
    fn bounds_y(&self) -> [f32; 2];
}

impl HasBounds for Path {
    fn bounds_x(&self) -> [f32; 2] {
        [self.min_x, self.max_x]
    }

    fn bounds_y(&self) -> [f32; 2] {
        [self.min_y, self.max_y]
    }
}

impl HasBounds for CanvasLabel {
    fn bounds_x(&self) -> [f32; 2] {
        let label = self.0.borrow();
        [0.0, label.width]
    }

    fn bounds_y(&self) -> [f32; 2] {
        let label = self.0.borrow();
        [label.font.top, label.font.bottom]
    }
}

fn set_stroke_style(
    c: &[f32; 4],
    ctx: &CanvasRenderingContext2d,
    str_buf: &mut String,
    cache: &mut HashMap<HashShader, JsValue>,
) {
    let hash_shader = HashShader::SolidColor(cast(*c));
    let style = cache
        .entry(hash_shader)
        .or_insert_with(|| JsValue::from_str(color(str_buf, c)));
    ctx.set_stroke_style(style);
}

fn set_fill_style(
    shader: &FillShader,
    ctx: &CanvasRenderingContext2d,
    cache: &mut HashMap<HashShader, JsValue>,
    str_buf: &mut String,
    handle: &impl HasBounds,
) {
    let hash_shader = match *shader {
        FillShader::SolidColor(c) => HashShader::SolidColor(cast(c)),
        FillShader::VerticalGradient(t, b) => {
            HashShader::VerticalGradient(cast(t), cast(b), cast(handle.bounds_y()))
        }
        FillShader::HorizontalGradient(l, r) => {
            HashShader::HorizontalGradient(cast(l), cast(r), cast(handle.bounds_x()))
        }
    };
    let style = cache.entry(hash_shader).or_insert_with(|| match shader {
        FillShader::SolidColor(c) => JsValue::from_str(color(str_buf, c)),
        FillShader::VerticalGradient(t, b) => {
            let [min_y, max_y] = handle.bounds_y();
            let gradient = ctx.create_linear_gradient(0.0, min_y as _, 0.0, max_y as _);
            let _ = gradient.add_color_stop(0.0, color(str_buf, t));
            let _ = gradient.add_color_stop(1.0, color(str_buf, b));
            gradient.unchecked_into()
        }
        FillShader::HorizontalGradient(l, r) => {
            let [min_x, max_x] = handle.bounds_x();
            let gradient = ctx.create_linear_gradient(min_x as _, 0.0, max_x as _, 0.0);
            let _ = gradient.add_color_stop(0.0, color(str_buf, l));
            let _ = gradient.add_color_stop(1.0, color(str_buf, r));
            gradient.unchecked_into()
        }
    });
    ctx.set_fill_style(style);
}

fn render_layer(
    ctx: &CanvasRenderingContext2d,
    cache: &mut HashMap<HashShader, JsValue>,
    str_buf: &mut String,
    layer: &[Entity<Path, Image, CanvasLabel>],
    digits: &[JsString; 10],
) {
    for entity in layer {
        match entity {
            Entity::FillPath(path, shader, transform) => {
                set_fill_style(shader, ctx, cache, str_buf, &**path);
                set_transform(ctx, transform);
                ctx.fill_with_path_2d(&path.path);
            }
            Entity::StrokePath(path, stroke_width, color, transform) => {
                set_fill_style(
                    &FillShader::SolidColor([0.0; 4]),
                    ctx,
                    cache,
                    str_buf,
                    &**path,
                );
                ctx.set_line_width(*stroke_width as f64);
                set_stroke_style(color, ctx, str_buf, cache);
                set_transform(ctx, transform);
                ctx.stroke_with_path(&path.path);
            }
            Entity::Image(image, transform) => {
                let image = image.0.borrow();
                if let Some((image, _)) = &*image {
                    let _ = ctx.reset_transform();
                    let _ = ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                        image,
                        transform.x as _,
                        transform.y as _,
                        transform.scale_x as _,
                        transform.scale_y as _,
                    );
                }
            }
            Entity::Label(label, shader, transform) => {
                set_fill_style(shader, ctx, cache, str_buf, &**label);
                let label = label.0.borrow();
                let label = &*label;
                set_font(ctx, &label.font);

                set_transform(
                    ctx,
                    &transform.pre_scale(FONT_SCALE_FACTOR, FONT_SCALE_FACTOR),
                );

                match &label.shape {
                    LabelShape::Normal(text) => {
                        let _ = ctx.fill_text(text, 0.0, 0.0);
                    }
                    LabelShape::MonospaceEmulation(pieces) => {
                        for piece in pieces {
                            match piece {
                                MonospacePiece::Digit { digit, offset } => {
                                    let _ = ctx.fill_text(
                                        &digits[*digit as usize],
                                        *offset as f64,
                                        0.0,
                                    );
                                }
                                MonospacePiece::Chunk { chunk, offset } => {
                                    let _ = ctx.fill_text(chunk, *offset as f64, 0.0);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn set_font(ctx: &CanvasRenderingContext2d, font: &CanvasFont) {
    ctx.set_font(&font.descriptor);
    ctx.set_font_kerning(&font.font_kerning);
}

fn set_transform(ctx: &CanvasRenderingContext2d, transform: &Transform) {
    let (sx, sy, tx, ty, ky, kx) = (
        transform.scale_x as f64,
        transform.scale_y as f64,
        transform.x as f64,
        transform.y as f64,
        0.0,
        0.0,
    );

    let _ = ctx.set_transform(sx, ky, kx, sy, tx, ty);
}

fn color<'b>(buf: &'b mut String, &[r, g, b, a]: &[f32; 4]) -> &'b str {
    use core::fmt::Write;

    buf.clear();
    let _ = write!(
        buf,
        "rgba({}, {}, {}, {})",
        255.0 * r,
        255.0 * g,
        255.0 * b,
        a
    );
    buf
}

const fn weight_as_css_str(weight: FontWeight) -> &'static str {
    match weight {
        FontWeight::Thin => "100",
        FontWeight::ExtraLight => "200",
        FontWeight::Light => "300",
        FontWeight::SemiLight => "350",
        FontWeight::Normal => "400",
        FontWeight::Medium => "500",
        FontWeight::SemiBold => "600",
        FontWeight::Bold => "700",
        FontWeight::ExtraBold => "800",
        FontWeight::Black => "900",
        FontWeight::ExtraBlack => "950",
    }
}

const fn stretch_as_css_str(stretch: FontStretch) -> &'static str {
    match stretch {
        FontStretch::UltraCondensed => "ultra-condensed",
        FontStretch::ExtraCondensed => "extra-condensed",
        FontStretch::Condensed => "condensed",
        FontStretch::SemiCondensed => "semi-condensed",
        FontStretch::Normal => "normal",
        FontStretch::SemiExpanded => "semi-expanded",
        FontStretch::Expanded => "expanded",
        FontStretch::ExtraExpanded => "extra-expanded",
        FontStretch::UltraExpanded => "ultra-expanded",
    }
}
