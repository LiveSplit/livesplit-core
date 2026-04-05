use alloc::vec::Vec;

use crate::{component::carousel::State, layout::LayoutState};

use super::{super::resource::ResourceAllocator, RenderContext, render};

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

pub fn render_carousel<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<A>,
    carousel: &State,
    state: &LayoutState,
    dim: [f32; 2],
    selected: Option<usize>,
    flat_index: &mut usize,
) {
    let caches = &mut cache.children;

    // Ensure we have exactly as many cached sub-components as the carousel has.
    if let Some(new_children) = carousel.components.get(caches.len()..) {
        caches.extend(new_children.iter().map(super::Cache::new));
    } else {
        caches.truncate(carousel.components.len());
    }

    // Render only the currently visible child.
    if let Some(child) = carousel.components.get(carousel.current_index) {
        if let Some(child_cache) = caches.get_mut(carousel.current_index) {
            // Skip flat indices for all children before the current one.
            for earlier in &carousel.components[..carousel.current_index] {
                *flat_index += subtree_count(earlier);
            }

            render(
                child_cache,
                context,
                child,
                state,
                dim,
                selected,
                flat_index,
            );

            // Skip flat indices for all children after the current one.
            for later in &carousel.components[carousel.current_index + 1..] {
                *flat_index += subtree_count(later);
            }
        }
    }
}

/// Counts the total number of state nodes in the subtree of a component
/// (including the component itself).
fn subtree_count(component: &crate::layout::ComponentState) -> usize {
    match component {
        crate::layout::ComponentState::Group(g) => {
            1 + g.components.iter().map(|c| subtree_count(c)).sum::<usize>()
        }
        crate::layout::ComponentState::Carousel(c) => {
            1 + c.components.iter().map(|c| subtree_count(c)).sum::<usize>()
        }
        _ => 1,
    }
}
