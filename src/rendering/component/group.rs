use crate::{
    component::group::State,
    layout::{LayoutDirection, LayoutState},
    platform::prelude::*,
};

use super::{super::resource::ResourceAllocator, RenderContext, height, render, width};

pub struct Cache<L> {
    pub children: Vec<super::Cache<L>>,
}

impl<L> Cache<L> {
    pub const fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

pub fn render_group<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<A>,
    group: &State,
    state: &LayoutState,
    dim: [f32; 2],
    selected: Option<usize>,
    flat_index: &mut usize,
) {
    let caches = &mut cache.children;

    // Ensure we have exactly as many cached sub-components as the group has.
    if let Some(new_children) = group.components.get(caches.len()..) {
        caches.extend(new_children.iter().map(super::Cache::new));
    } else {
        caches.truncate(group.components.len());
    }

    let [total_w, total_h] = dim;

    // Set the effective direction to the opposite of the parent's direction
    // so that child components render themselves correctly (e.g.
    // display_two_rows when the group is horizontal). We restore the
    // previous direction after rendering all children.
    let parent_direction = context.direction;
    let parent_transform = context.transform;
    let group_direction = parent_direction.opposite();
    context.direction = group_direction;

    match group_direction {
        LayoutDirection::Vertical => {
            let total_child_height: f32 = group
                .components
                .iter()
                .map(|c| height(c, LayoutDirection::Vertical))
                .sum();
            let scale = if total_child_height > 0.0 {
                total_h / total_child_height
            } else {
                1.0
            };
            for (child, child_cache) in group.components.iter().zip(caches.iter_mut()) {
                let child_h = height(child, LayoutDirection::Vertical) * scale;
                let child_dim = [total_w, child_h];
                render(
                    child_cache,
                    context,
                    child,
                    state,
                    child_dim,
                    selected,
                    flat_index,
                );
                context.translate(0.0, child_h);
            }
        }
        LayoutDirection::Horizontal => {
            let total_child_width: f32 = group
                .components
                .iter()
                .map(|c| width(c, LayoutDirection::Horizontal))
                .sum();
            let width_scale = if total_child_width > 0.0 {
                total_w / total_child_width
            } else {
                1.0
            };
            for (child, child_cache) in group.components.iter().zip(caches.iter_mut()) {
                let child_w = width(child, LayoutDirection::Horizontal) * width_scale;
                let child_dim = [child_w, total_h];
                render(
                    child_cache,
                    context,
                    child,
                    state,
                    child_dim,
                    selected,
                    flat_index,
                );
                context.translate(child_w, 0.0);
            }
        }
    }

    context.direction = parent_direction;
    context.transform = parent_transform;
}
