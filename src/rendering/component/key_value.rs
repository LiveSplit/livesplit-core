use core::marker::PhantomData;

use crate::{
    component::key_value::State,
    layout::{LayoutDirection, LayoutState},
    rendering::{
        font::{AbbreviatedLabel, CachedLabel},
        resource::ResourceAllocator,
        RenderContext,
    },
};

pub struct Cache<I, L> {
    key: AbbreviatedLabel<L>,
    value: CachedLabel<L>,
    _image: PhantomData<I>,
}

impl<I, L> Cache<I, L> {
    pub const fn new() -> Self {
        Self {
            key: AbbreviatedLabel::new(),
            value: CachedLabel::new(),
            _image: PhantomData,
        }
    }
}

pub(in crate::rendering) fn render<A: ResourceAllocator>(
    cache: &mut Cache<A::Image, A::Label>,
    context: &mut RenderContext<'_, A>,
    dim: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
) {
    context.render_background(dim, &component.background);
    context.render_key_value_component(
        &component.key,
        &component.key_abbreviations,
        &mut cache.key,
        &component.value,
        &mut cache.value,
        component.updates_frequently,
        dim,
        component.key_color.unwrap_or(layout_state.text_color),
        component.value_color.unwrap_or(layout_state.text_color),
        component.display_two_rows || layout_state.direction == LayoutDirection::Horizontal,
    );
}
