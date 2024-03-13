//! Provides a renderer that emits vector images in the SVG format.

use core::{
    cell::{Cell, RefCell},
    fmt::{self, Write},
    hash::{BuildHasher, BuildHasherDefault},
};

use ahash::AHasher;
use alloc::rc::Rc;
use hashbrown::{HashSet, HashTable};

use crate::{
    layout::LayoutState,
    platform::prelude::*,
    settings::{Font, ImageCache, BLUR_FACTOR},
    util::xml::{AttributeWriter, DisplayAlreadyEscaped, Text, Value, Writer},
};

use super::{
    default_text_engine::{self, TextEngine},
    Background, Entity, FillShader, FontKind, ResourceAllocator, SceneManager, SharedOwnership,
    Transform,
};

type SvgImage = Rc<Image>;
type SvgFont = default_text_engine::Font;
type SvgLabel = default_text_engine::Label<SvgPath>;

/// The SVG renderer allows rendering layouts to vector images in the SVG
/// format.
pub struct Renderer {
    allocator: SvgAllocator,
    scene_manager: SceneManager<SvgPath, SvgImage, SvgFont, SvgLabel>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    /// Creates a new SVG renderer.
    pub fn new() -> Self {
        let mut allocator = SvgAllocator {
            text_engine: TextEngine::new(),
            defs: Rc::new(RefCell::new(Defs {
                ptr_lookup: HashSet::new(),
                gradients_lookup: HashTable::new(),
            })),
        };
        let scene_manager = SceneManager::new(&mut allocator);
        Self {
            allocator,
            scene_manager,
        }
    }

    /// Renders the layout state with the chosen dimensions to the writer
    /// provided. It may detect that the layout got resized. In that case it
    /// returns the new ideal size. This is just a hint and can be ignored
    /// entirely. The image is always rendered with the dimensions provided.
    pub fn render<W: fmt::Write>(
        &mut self,
        writer: W,
        layout_state: &LayoutState,
        image_cache: &ImageCache,
        [width, height]: [f32; 2],
    ) -> Result<Option<[f32; 2]>, fmt::Error> {
        let new_dims = self.scene_manager.update_scene(
            &mut self.allocator,
            [width, height],
            layout_state,
            image_cache,
        );

        let writer = &mut Writer::new_with_default_header(writer)?;

        writer.tag_with_content(
            "svg",
            [
                (
                    "viewBox",
                    DisplayAlreadyEscaped(format_args!("0 0 {width} {height}")),
                ),
                (
                    "xmlns",
                    DisplayAlreadyEscaped(format_args!("http://www.w3.org/2000/svg")),
                ),
            ],
            |writer| {
                let background_filter_id = writer.tag("defs", |writer| {
                    writer.content(|writer| self.write_defs(writer))
                })?;
                self.write_scene(writer, width, height, background_filter_id)?;

                Ok(())
            },
        )?;

        Ok(new_dims)
    }

    fn write_defs<W: Write>(
        &mut self,
        writer: &mut Writer<W>,
    ) -> Result<Option<usize>, fmt::Error> {
        let current_id = &mut 0;
        let mut background_filter_id = None;

        let scene = self.scene_manager.scene();
        let defs = &mut *self.allocator.defs.borrow_mut();

        defs.ptr_lookup.clear();

        if let Some(background) = scene.background() {
            match background {
                Background::Shader(shader) => visit_shader(current_id, defs, writer, shader)?,
                Background::Image(image, transform) => {
                    visit_image(current_id, defs, writer, &image.image)?;

                    let needs_blur = image.blur != 0.0;
                    let needs_matrix = image.brightness != 1.0 || image.opacity != 1.0;
                    let needs_filter = needs_blur || needs_matrix;

                    if needs_filter {
                        let id = *background_filter_id.insert(*current_id);
                        *current_id += 1;

                        writer.tag_with_content(
                            "filter",
                            [("id", DisplayAlreadyEscaped(id))],
                            |writer| {
                                if needs_blur {
                                    writer.empty_tag(
                                        "feGaussianBlur",
                                        [(
                                            "stdDeviation",
                                            DisplayAlreadyEscaped(
                                                BLUR_FACTOR
                                                    * image.blur
                                                    * transform.scale_x.max(transform.scale_y),
                                            ),
                                        )],
                                    )?;
                                }

                                if needs_matrix {
                                    writer.empty_tag(
                                        "feColorMatrix",
                                        [(
                                            "values",
                                            DisplayAlreadyEscaped(format_args!(
                                                "{b} 0 0 0 0 \
                                             0 {b} 0 0 0 \
                                             0 0 {b} 0 0 \
                                             0 0 0 {o} 0",
                                                b = image.brightness,
                                                o = image.opacity,
                                            )),
                                        )],
                                    )?;
                                }

                                Ok(())
                            },
                        )?;
                    }
                }
            }
        }

        for entity in scene.bottom_layer().iter().chain(scene.top_layer()) {
            match entity {
                Entity::FillPath(path, shader, _) => {
                    visit_path(current_id, defs, writer, path)?;
                    visit_shader(current_id, defs, writer, shader)?;
                }
                Entity::StrokePath(path, _, _, _) => visit_path(current_id, defs, writer, path)?,
                Entity::Image(image, _) => visit_image(current_id, defs, writer, image)?,
                Entity::Label(label, shader, _) => {
                    for glyph in label.read().unwrap().glyphs() {
                        if glyph.color.is_none() {
                            visit_shader(current_id, defs, writer, shader)?;
                        }

                        visit_path(current_id, defs, writer, &glyph.path)?;
                    }
                }
            }
        }

        Ok(background_filter_id)
    }

    fn write_scene<W: Write>(
        &mut self,
        writer: &mut Writer<W>,
        width: f32,
        height: f32,
        background_filter_id: Option<usize>,
    ) -> Result<(), fmt::Error> {
        let scene = self.scene_manager.scene();

        if let Some(background) = scene.background() {
            match background {
                Background::Shader(shader) => {
                    if let Some((fill, opacity)) = convert_shader(shader, &self.allocator.defs) {
                        writer.tag("rect", |mut writer| {
                            writer.attribute("width", DisplayAlreadyEscaped(width))?;
                            writer.attribute("height", DisplayAlreadyEscaped(height))?;
                            writer.attribute("fill", fill)?;
                            if let Some(opacity) = opacity {
                                writer.attribute("fill-opacity", DisplayAlreadyEscaped(opacity))?;
                            }
                            Ok(())
                        })?;
                    }
                }
                Background::Image(image, transform) => {
                    writer.tag("use", |mut writer| {
                        writer.attribute(
                            "href",
                            DisplayAlreadyEscaped(format_args!("#{}", (*image.image).id.get())),
                        )?;
                        writer.attribute(
                            "transform",
                            TransformValue(
                                &transform.pre_scale(image.image.scale_x, image.image.scale_y),
                            ),
                        )?;

                        if let Some(id) = background_filter_id {
                            writer.attribute(
                                "filter",
                                DisplayAlreadyEscaped(format_args!("url(#{})", id)),
                            )?;
                        }

                        Ok(())
                    })?;
                }
            }
        }

        for entity in scene.bottom_layer().iter().chain(scene.top_layer()) {
            match entity {
                Entity::FillPath(path, shader, transform) => {
                    let Some((fill, opacity)) = convert_shader(shader, &self.allocator.defs) else {
                        continue;
                    };
                    path_with_transform(
                        writer,
                        path,
                        transform,
                        [("fill", fill.into()), ("fill-opacity", opacity.into())],
                    )?;
                }
                Entity::StrokePath(path, stroke_width, color, transform) => {
                    let Some((color, opacity)) = convert_color(color) else {
                        continue;
                    };
                    path_with_transform(
                        writer,
                        path,
                        transform,
                        [
                            ("stroke", Fill::Rgb(color).into()),
                            ("stroke-width", (*stroke_width * transform.scale_y).into()),
                            ("stroke-opacity", opacity.into()),
                        ],
                    )?;
                }
                Entity::Image(image, transform) => {
                    writer.empty_tag(
                        "use",
                        [
                            (
                                "href",
                                DisplayAlreadyEscaped(format_args!("#{}", (**image).id.get())),
                            ),
                            (
                                "transform",
                                DisplayAlreadyEscaped(format_args!(
                                    "{}",
                                    TransformValue(
                                        &transform.pre_scale(image.scale_x, image.scale_y)
                                    )
                                )),
                            ),
                        ],
                    )?;
                }
                Entity::Label(label, shader, transform) => {
                    for glyph in label.read().unwrap().glyphs() {
                        let (fill, opacity) = if let Some(color) = &glyph.color {
                            let Some((color, opacity)) = convert_color(color) else {
                                continue;
                            };
                            (Fill::Rgb(color), opacity)
                        } else {
                            let Some((fill, opacity)) =
                                convert_shader(shader, &self.allocator.defs)
                            else {
                                continue;
                            };
                            (fill, opacity)
                        };

                        path_with_transform(
                            writer,
                            &glyph.path,
                            &transform
                                .pre_translate(glyph.x, glyph.y)
                                .pre_scale(glyph.scale, glyph.scale),
                            [("fill", fill.into()), ("fill-opacity", opacity.into())],
                        )?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn visit_path<W: Write>(
    current_id: &mut usize,
    defs: &mut Defs,
    writer: &mut Writer<W>,
    path: &SvgPath,
) -> fmt::Result {
    if let SvgPath::Path(path) = path {
        if defs.ptr_lookup.insert(Rc::as_ptr(path) as usize) {
            path.id.set(*current_id);
            *current_id += 1;

            let (tag, attr) = match path.kind {
                PathKind::Polygon => ("polygon", "points"),
                PathKind::Polyline => ("polyline", "points"),
                PathKind::Path => ("path", "d"),
            };

            writer.empty_tag(
                tag,
                [
                    (
                        "id",
                        DisplayAlreadyEscaped(format_args!("{}", path.id.get())),
                    ),
                    (attr, DisplayAlreadyEscaped(format_args!("{}", path.data))),
                ],
            )?;
        }
    }
    Ok(())
}

fn visit_image<W: Write>(
    current_id: &mut usize,
    defs: &mut Defs,
    writer: &mut Writer<W>,
    image: &SvgImage,
) -> fmt::Result {
    if defs.ptr_lookup.insert(Rc::as_ptr(image) as usize) {
        image.id.set(*current_id);
        *current_id += 1;

        writer.empty_tag(
            "image",
            [
                (
                    "id",
                    DisplayAlreadyEscaped(format_args!("{}", image.id.get())),
                ),
                (
                    "href",
                    DisplayAlreadyEscaped(format_args!("{}", image.data)),
                ),
            ],
        )?;
    }
    Ok(())
}

fn visit_shader<W: Write>(
    current_id: &mut usize,
    defs: &mut Defs,
    writer: &mut Writer<W>,
    shader: &FillShader,
) -> fmt::Result {
    let (vertical, start, end) = match shader {
        FillShader::SolidColor(_) => return Ok(()),
        FillShader::VerticalGradient(top, bottom) => (true, top, bottom),
        FillShader::HorizontalGradient(left, right) => (false, left, right),
    };

    let gradient = defs.add_gradient(vertical, start, end);

    if defs.ptr_lookup.insert(Rc::as_ptr(&gradient) as usize) {
        gradient.id.set(*current_id);
        *current_id += 1;

        writer.tag_with_content(
            "linearGradient",
            [
                (
                    "id",
                    DisplayAlreadyEscaped(format_args!("{}", gradient.id.get())),
                ),
                (
                    "x2",
                    DisplayAlreadyEscaped(format_args!("{}", if vertical { "0" } else { "1" })),
                ),
                (
                    "y2",
                    DisplayAlreadyEscaped(format_args!("{}", if vertical { "1" } else { "0" })),
                ),
            ],
            |writer| {
                writer.tag("stop", |mut writer| {
                    let (start_rgb, start_a) = convert_color_or_transparent(start);
                    writer.attribute("stop-color", start_rgb)?;
                    if let Some(a) = start_a {
                        writer.attribute("stop-opacity", DisplayAlreadyEscaped(a))?;
                    }
                    Ok(())
                })?;
                writer.tag("stop", |mut writer| {
                    let (end_rgb, end_a) = convert_color_or_transparent(end);
                    writer.attribute("offset", Text::new_escaped("1"))?;
                    writer.attribute("stop-color", end_rgb)?;
                    if let Some(a) = end_a {
                        writer.attribute("stop-opacity", DisplayAlreadyEscaped(a))?;
                    }
                    Ok(())
                })
            },
        )?;
    }

    Ok(())
}

struct SvgAllocator {
    text_engine: TextEngine<SvgPath>,
    defs: Rc<RefCell<Defs>>,
}

struct Gradient {
    id: Cell<usize>,
    vertical: bool,
    start: [f32; 4],
    end: [f32; 4],
}

impl core::hash::Hash for Gradient {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.vertical.hash(state);
        self.start.map(f32::to_bits).hash(state);
        self.end.map(f32::to_bits).hash(state);
    }
}

impl PartialEq for Gradient {
    fn eq(&self, other: &Self) -> bool {
        self.vertical == other.vertical
            && self.start.map(f32::to_bits) == other.start.map(f32::to_bits)
            && self.end.map(f32::to_bits) == other.end.map(f32::to_bits)
    }
}

impl Eq for Gradient {}

struct Defs {
    ptr_lookup: HashSet<usize>,
    gradients_lookup: HashTable<Rc<Gradient>>,
}

impl Defs {
    fn add_gradient(&mut self, vertical: bool, start: &[f32; 4], end: &[f32; 4]) -> Rc<Gradient> {
        let hasher = BuildHasherDefault::<AHasher>::default();
        let hasher = |val: &Gradient| hasher.hash_one(val);
        let gradient = Gradient {
            id: Cell::new(0),
            vertical,
            start: *start,
            end: *end,
        };
        self.gradients_lookup
            .entry(hasher(&gradient), |g| gradient == **g, |g| hasher(g))
            .or_insert_with(|| Rc::new(gradient))
            .get()
            .clone()
    }
}

struct PathBuilder {
    segments: Vec<PathSegment>,
}

#[derive(Debug, Clone)]
enum SvgPath {
    Rectangle,
    Circle(f32, f32, f32),
    Line(Point, Point),
    Path(Rc<PathData>),
}

#[derive(Debug, Copy, Clone)]
enum PathKind {
    Polygon,
    Polyline,
    Path,
}

#[derive(Debug)]
struct PathData {
    id: Cell<usize>,
    kind: PathKind,
    data: String,
}

#[derive(Clone)]
struct Image {
    id: Cell<usize>,
    data: String,
    scale_x: f32,
    scale_y: f32,
}

impl SharedOwnership for SvgPath {
    fn share(&self) -> Self {
        self.clone()
    }
}

impl SharedOwnership for Image {
    fn share(&self) -> Self {
        self.clone()
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

enum PathSegment {
    MoveTo(Point),
    LineTo(Point),
    QuadTo(Point, Point),
    CurveTo(Point, Point, Point),
    Close,
}

impl super::PathBuilder for PathBuilder {
    type Path = SvgPath;

    fn move_to(&mut self, x: f32, y: f32) {
        self.segments.push(PathSegment::MoveTo(Point { x, y }));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.segments.push(PathSegment::LineTo(Point { x, y }));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.segments
            .push(PathSegment::QuadTo(Point { x: x1, y: y1 }, Point { x, y }));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.segments.push(PathSegment::CurveTo(
            Point { x: x1, y: y1 },
            Point { x: x2, y: y2 },
            Point { x, y },
        ));
    }

    fn close(&mut self) {
        self.segments.push(PathSegment::Close);
    }

    fn finish(self) -> Self::Path {
        if let [outer_rem @ .., PathSegment::Close] = &*self.segments {
            if let [PathSegment::MoveTo(_), rem @ ..] = outer_rem {
                if rem
                    .iter()
                    .all(|segment| matches!(segment, PathSegment::LineTo(_)))
                {
                    let point_iter = outer_rem.iter().map(|segment| match segment {
                        PathSegment::MoveTo(point) | PathSegment::LineTo(point) => *point,
                        _ => unreachable!(),
                    });

                    let mut data = String::new();
                    for point in point_iter {
                        if !data.is_empty() {
                            data.push(' ');
                        }
                        let _ = write!(data, "{},{}", point.x, point.y);
                    }
                    return SvgPath::Path(Rc::new(PathData {
                        id: Cell::new(0),
                        kind: PathKind::Polygon,
                        data,
                    }));
                }
            }
        }
        if let [PathSegment::MoveTo(start), rem @ ..] = &*self.segments {
            if let [PathSegment::LineTo(end)] = rem {
                return SvgPath::Line(*start, *end);
            }

            if rem
                .iter()
                .all(|segment| matches!(segment, PathSegment::LineTo(_)))
            {
                let point_iter = rem.iter().map(|segment| match segment {
                    PathSegment::MoveTo(point) | PathSegment::LineTo(point) => *point,
                    _ => unreachable!(),
                });

                let mut data = String::new();
                for point in point_iter {
                    if !data.is_empty() {
                        data.push(' ');
                    }
                    let _ = write!(data, "{},{}", point.x, point.y);
                }
                return SvgPath::Path(Rc::new(PathData {
                    id: Cell::new(0),
                    kind: PathKind::Polyline,
                    data,
                }));
            }
        }

        let mut data = String::new();

        for segment in self.segments {
            if !data.is_empty() {
                data.push(' ');
            }
            match segment {
                PathSegment::MoveTo(point) => {
                    let _ = write!(data, "M{},{}", point.x, point.y);
                }
                PathSegment::LineTo(point) => {
                    let _ = write!(data, "L{},{}", point.x, point.y);
                }
                PathSegment::QuadTo(point1, point2) => {
                    let _ = write!(data, "Q{},{} {},{}", point1.x, point1.y, point2.x, point2.y);
                }
                PathSegment::CurveTo(point1, point2, point3) => {
                    let _ = write!(
                        data,
                        "C{},{} {},{} {},{}",
                        point1.x, point1.y, point2.x, point2.y, point3.x, point3.y
                    );
                }
                PathSegment::Close => {
                    data.push('Z');
                }
            }
        }

        SvgPath::Path(Rc::new(PathData {
            id: Cell::new(0),
            kind: PathKind::Path,
            data,
        }))
    }
}

impl ResourceAllocator for SvgAllocator {
    type PathBuilder = PathBuilder;
    type Path = SvgPath;
    type Image = SvgImage;
    type Font = SvgFont;
    type Label = SvgLabel;

    fn path_builder(&mut self) -> Self::PathBuilder {
        PathBuilder {
            segments: Vec::new(),
        }
    }

    fn create_image(&mut self, _data: &[u8]) -> Option<(Self::Image, f32)> {
        #[cfg(feature = "image")]
        {
            let format = image::guess_format(_data).ok()?;

            let (width, height) = crate::util::image::get_dimensions(format, _data)?;
            let (width, height) = (width as f32, height as f32);
            let (rwidth, rheight) = (width.recip(), height.recip());

            let mut buf = String::new();
            buf.push_str("data:;base64,");

            // SAFETY: We encode Base64 to the end of the string, which is
            // always valid UTF-8. Once we've written it, we simply increase
            // the length of the buffer by the amount of bytes written.
            unsafe {
                let buf = buf.as_mut_vec();
                let encoded_len = base64_simd::STANDARD.encoded_length(_data.len());
                buf.reserve_exact(encoded_len);
                let additional_len = base64_simd::STANDARD
                    .encode(
                        _data,
                        base64_simd::Out::from_uninit_slice(buf.spare_capacity_mut()),
                    )
                    .len();
                buf.set_len(buf.len() + additional_len);
            }

            Some((
                Rc::new(Image {
                    id: Cell::new(0),
                    data: buf,
                    scale_x: rwidth,
                    scale_y: rheight,
                }),
                width * rheight,
            ))
        }
        #[cfg(not(feature = "image"))]
        {
            None
        }
    }

    fn create_font(&mut self, font: Option<&Font>, kind: FontKind) -> Self::Font {
        self.text_engine.create_font(font, kind)
    }

    fn create_label(
        &mut self,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) -> Self::Label {
        self.text_engine.create_label(
            || PathBuilder {
                segments: Vec::new(),
            },
            text,
            font,
            max_width,
        )
    }

    fn update_label(
        &mut self,
        label: &mut Self::Label,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) {
        self.text_engine.update_label(
            || PathBuilder {
                segments: Vec::new(),
            },
            label,
            text,
            font,
            max_width,
        )
    }

    fn build_circle(&mut self, x: f32, y: f32, r: f32) -> Self::Path {
        SvgPath::Circle(x, y, r)
    }

    fn build_square(&mut self) -> Self::Path {
        SvgPath::Rectangle
    }
}

enum AttrValue {
    Fill(Fill),
    F32(f32),
    OptionF32(Option<f32>),
}

impl From<Fill> for AttrValue {
    fn from(v: Fill) -> Self {
        Self::Fill(v)
    }
}

impl From<f32> for AttrValue {
    fn from(v: f32) -> Self {
        Self::F32(v)
    }
}

impl From<Option<f32>> for AttrValue {
    fn from(v: Option<f32>) -> Self {
        Self::OptionF32(v)
    }
}

fn path_with_transform<'a, W: fmt::Write>(
    writer: &mut Writer<W>,
    path: &SvgPath,
    transform: &Transform,
    attrs: impl IntoIterator<Item = (&'a str, AttrValue)>,
) -> fmt::Result {
    let add_attrs = |mut writer: AttributeWriter<'_, W>| {
        for (key, value) in attrs {
            match value {
                AttrValue::Fill(fill) => writer.attribute(key, fill)?,
                AttrValue::F32(value) => writer.attribute(key, DisplayAlreadyEscaped(value))?,
                AttrValue::OptionF32(value) => {
                    if let Some(value) = value {
                        writer.attribute(key, DisplayAlreadyEscaped(value))?;
                    }
                }
            }
        }
        Ok(())
    };

    match path {
        SvgPath::Circle(x, y, r) => {
            let [x, y] = Point { x: *x, y: *y }.transform(transform);
            let width = r * transform.scale_x;
            let height = r * transform.scale_y;
            if width == height {
                writer.tag("circle", |mut writer| {
                    writer.attribute("cx", DisplayAlreadyEscaped(x))?;
                    writer.attribute("cy", DisplayAlreadyEscaped(y))?;
                    writer.attribute("r", DisplayAlreadyEscaped(width))?;
                    add_attrs(writer)
                })?;
            } else {
                writer.tag("ellipse", |mut writer| {
                    writer.attribute("cx", DisplayAlreadyEscaped(x))?;
                    writer.attribute("cy", DisplayAlreadyEscaped(y))?;
                    writer.attribute("rx", DisplayAlreadyEscaped(width))?;
                    writer.attribute("ry", DisplayAlreadyEscaped(height))?;
                    add_attrs(writer)
                })?;
            }
        }
        SvgPath::Line(start, end) => {
            let [x1, y1] = start.transform(transform);
            let [x2, y2] = end.transform(transform);
            writer.tag("line", |mut writer| {
                writer.attribute("x1", DisplayAlreadyEscaped(x1))?;
                writer.attribute("y1", DisplayAlreadyEscaped(y1))?;
                writer.attribute("x2", DisplayAlreadyEscaped(x2))?;
                writer.attribute("y2", DisplayAlreadyEscaped(y2))?;
                add_attrs(writer)
            })?
        }
        SvgPath::Rectangle => writer.tag("rect", |mut writer| {
            if transform.x != 0.0 {
                writer.attribute("x", DisplayAlreadyEscaped(transform.x))?;
            }
            if transform.y != 0.0 {
                writer.attribute("y", DisplayAlreadyEscaped(transform.y))?;
            }
            writer.attribute("width", DisplayAlreadyEscaped(transform.scale_x))?;
            writer.attribute("height", DisplayAlreadyEscaped(transform.scale_y))?;
            add_attrs(writer)
        })?,
        SvgPath::Path(path) => writer.tag("use", |mut writer| {
            writer.attribute(
                "href",
                DisplayAlreadyEscaped(format_args!("#{}", path.id.get())),
            )?;
            writer.attribute("transform", TransformValue(transform))?;
            add_attrs(writer)
        })?,
    };

    Ok(())
}

impl Point {
    fn transform(&self, transform: &Transform) -> [f32; 2] {
        [
            self.x * transform.scale_x + transform.x,
            self.y * transform.scale_y + transform.y,
        ]
    }
}

enum Fill {
    Rgb(Rgb),
    Url(usize),
}

impl Value for Fill {
    fn write_escaped<T: fmt::Write>(self, sink: &mut T) -> fmt::Result {
        match self {
            Fill::Rgb(rgb) => rgb.write_escaped(sink),
            Fill::Url(id) => write!(sink, "url(#{id})"),
        }
    }

    fn is_empty(&self) -> bool {
        false
    }
}

fn convert_shader(shader: &FillShader, defs: &Rc<RefCell<Defs>>) -> Option<(Fill, Option<f32>)> {
    Some(match shader {
        FillShader::SolidColor(c) => {
            let (rgb, a) = convert_color(c)?;
            (Fill::Rgb(rgb), a)
        }
        FillShader::VerticalGradient(top, bottom) => {
            let gradient = defs.borrow_mut().add_gradient(true, top, bottom);
            (Fill::Url(gradient.id.get()), None)
        }
        FillShader::HorizontalGradient(left, right) => {
            let gradient = defs.borrow_mut().add_gradient(false, left, right);
            (Fill::Url(gradient.id.get()), None)
        }
    })
}

fn convert_color_or_transparent(&[r, g, b, a]: &[f32; 4]) -> (Rgb, Option<f32>) {
    convert_color(&[r, g, b, a]).unwrap_or((
        Rgb {
            r: 0xFF,
            b: 0xFF,
            g: 0xFF,
        },
        Some(0.0),
    ))
}

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Value for Rgb {
    fn write_escaped<T: fmt::Write>(self, sink: &mut T) -> fmt::Result {
        if self.r == 0xFF && self.g == 0xFF && self.b == 0xFF {
            sink.write_str("white")
        } else if self.r == 0x00 && self.g == 0x00 && self.b == 0x00 {
            sink.write_str("black")
        } else {
            write!(sink, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        }
    }

    fn is_empty(&self) -> bool {
        false
    }
}

fn convert_color(&[r, g, b, a]: &[f32; 4]) -> Option<(Rgb, Option<f32>)> {
    if a == 0.0 {
        return None;
    }
    let a = if a != 1.0 { Some(a) } else { None };
    Some((
        Rgb {
            r: (255.0 * r) as u8,
            g: (255.0 * g) as u8,
            b: (255.0 * b) as u8,
        },
        a,
    ))
}

#[derive(Copy, Clone)]
struct TransformValue<'a>(&'a Transform);

impl fmt::Display for TransformValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_escaped(f)
    }
}

impl Value for TransformValue<'_> {
    fn write_escaped<T: fmt::Write>(self, sink: &mut T) -> fmt::Result {
        write!(
            sink,
            "matrix({},0,0,{},{},{})",
            self.0.scale_x, self.0.scale_y, self.0.x, self.0.y
        )
    }

    fn is_empty(&self) -> bool {
        false
    }
}
