use std::{
    hash::{Hash, Hasher},
    mem,
};

use ahash::AHasher;

use super::{resource::Handle, FillShader, Rgba, Transform};

/// An entity describes an element positioned on a [`Scene's`](super::Scene)
/// [`Layer`](super::Layer) that is meant to be visualized.
pub enum Entity<P, I> {
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
}

pub fn calculate_hash<P, I>(background: &Option<FillShader>, entities: &[Entity<P, I>]) -> u64 {
    let mut hasher = AHasher::new_with_keys(1234, 5678);
    mem::discriminant(background).hash(&mut hasher);
    if let Some(background) = background {
        hash_shader(background, &mut hasher);
    }
    entities.hash(&mut hasher);
    hasher.finish()
}

fn hash_float(f: f32, state: &mut impl Hasher) {
    u32::hash(&bytemuck::cast(f), state);
}

fn hash_floats(f: &[f32], state: &mut impl Hasher) {
    u32::hash_slice(bytemuck::cast_slice(f), state);
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

impl<P, I> Hash for Entity<P, I> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state);
        match self {
            Entity::FillPath(path, shader, transform) => {
                path.hash(state);
                hash_shader(shader, state);
                hash_floats(&transform.to_array(), state);
            }
            Entity::StrokePath(path, stroke_width, color, transform) => {
                path.hash(state);
                hash_float(*stroke_width, state);
                hash_floats(color, state);
                hash_floats(&transform.to_array(), state);
            }
            Entity::Image(image, transform) => {
                image.hash(state);
                hash_floats(&transform.to_array(), state);
            }
        }
    }
}
