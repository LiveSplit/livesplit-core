use std::marker::PhantomData;

use crate::{
    component::key_value::State,
    layout::{LayoutDirection, LayoutState},
    rendering::{
        font::{AbbreviatedLabel, Label},
        resource::ResourceAllocator,
        RenderContext,
    },
};

pub struct Cache<I> {
    key: AbbreviatedLabel,
    value: Label,
    _image: PhantomData<I>,
}

impl<I> Cache<I> {
    pub const fn new() -> Self {
        Self {
            key: AbbreviatedLabel::new(),
            value: Label::new(),
            _image: PhantomData,
        }
    }
}

pub(in crate::rendering) fn render<B: ResourceAllocator>(
    cache: &mut Cache<B::Image>,
    context: &mut RenderContext<'_, B>,
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
