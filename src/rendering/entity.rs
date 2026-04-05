use core::{
    hash::{Hash, Hasher},
    mem,
};

use crate::{settings::BackgroundImage, util::FxHasher};

use super::{
    Background, FillShader, Rgba, Transform,
    resource::{Handle, LabelHandle},
};

/// An entity describes an element positioned on a [`Scene's`](super::Scene)
/// [`Layer`](super::Layer) that is meant to be visualized.
pub enum Entity<P, I, L> {
    /// A path where the inside is filled with the [`FillShader`]. For
    /// determining what's inside the [non-zero fill
    /// rule](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule#nonzero)
    /// is supposed to be used.
    FillPath(Handle<P>, FillShader, Transform),
    /// A path where only the path lines themselves are supposed to be drawn.
    /// There is no notion of an inside region. The floating point number
    /// determines the thickness of the path.
    StrokePath(Handle<P>, f32, Rgba, Transform),
    /// An image.
    Image(Handle<I>, Transform),
    /// A text label with a [`FillShader`] that describes the text color and an
    /// optional text shadow.
    Label(LabelHandle<L>, FillShader, Option<Rgba>, Transform),
}

pub fn calculate_hash<P, I, L>(
    background: &Option<Background<I>>,
    entities: &[Entity<P, I, L>],
) -> u64 {
    let mut hasher = FxHasher::new();
    mem::discriminant(background).hash(&mut hasher);
    if let Some(background) = background {
        background.hash(&mut hasher);
    }
    entities.hash(&mut hasher);
    hasher.finish()
}

#[inline]
fn hash_float(f: f32, state: &mut impl Hasher) {
    u32::hash(&bytemuck::cast(f), state);
}

#[inline]
fn hash_transform(f: &Transform, state: &mut impl Hasher) {
    const {
        assert!(core::mem::size_of::<Transform>() == 16);
    }
    let [a, b]: [u64; 2] = bytemuck::cast(*f);
    u64::hash(&a, state);
    u64::hash(&b, state);
}

#[inline]
fn hash_floats(f: &[f32; 4], state: &mut impl Hasher) {
    let [a, b]: [u64; 2] = bytemuck::cast(*f);
    u64::hash(&a, state);
    u64::hash(&b, state);
}

fn hash_shader(shader: &FillShader, state: &mut impl Hasher) {
    mem::discriminant(shader).hash(state);
    match shader {
        FillShader::SolidColor(c) => hash_floats(c, state),
        FillShader::VerticalGradient(t, b) => {
            hash_floats(t, state);
            hash_floats(b, state);
        }
        FillShader::HorizontalGradient(l, r) => {
            hash_floats(l, state);
            hash_floats(r, state);
        }
    }
}

impl<P, I, L> Hash for Entity<P, I, L> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);
        match self {
            Entity::FillPath(path, shader, transform) => {
                path.hash(state);
                hash_shader(shader, state);
                hash_transform(transform, state);
            }
            Entity::StrokePath(path, stroke_width, color, transform) => {
                path.hash(state);
                hash_float(*stroke_width, state);
                hash_floats(color, state);
                hash_transform(transform, state);
            }
            Entity::Image(image, transform) => {
                image.hash(state);
                hash_transform(transform, state);
            }
            Entity::Label(label, shader, text_shadow, transform) => {
                label.hash(state);
                hash_shader(shader, state);
                mem::discriminant(text_shadow).hash(state);
                if let Some(text_shadow) = text_shadow {
                    hash_floats(text_shadow, state);
                }
                hash_transform(transform, state);
            }
        }
    }
}

impl<I> Hash for Background<I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);
        match self {
            Background::Shader(shader) => hash_shader(shader, state),
            Background::Image(image, transform) => {
                let BackgroundImage {
                    image,
                    brightness,
                    opacity,
                    blur,
                } = image;
                image.hash(state);
                hash_float(*brightness, state);
                hash_float(*opacity, state);
                hash_float(*blur, state);
                hash_transform(transform, state);
            }
        }
    }
}
