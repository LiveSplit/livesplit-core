use std::marker::PhantomData;

use crate::{
    component::text::{State, TextState},
    layout::{LayoutDirection, LayoutState},
    rendering::{
        consts::{DEFAULT_TEXT_SIZE, PADDING, TEXT_ALIGN_TOP},
        font::{AbbreviatedLabel, Label},
        resource::ResourceAllocator,
        solid, RenderContext,
    },
};

pub struct Cache<I> {
    label1: AbbreviatedLabel,
    label2: Label,
    _image: PhantomData<I>,
}

impl<I> Cache<I> {
    pub const fn new() -> Self {
        Self {
            label1: AbbreviatedLabel::new(),
            label2: Label::new(),
            _image: PhantomData,
        }
    }
}

pub(in crate::rendering) fn render<B: ResourceAllocator>(
    cache: &mut Cache<B::Image>,
    context: &mut RenderContext<'_, B>,
    [width, height]: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
) {
    context.render_background([width, height], &component.background);
    match &component.text {
        TextState::Center(text) => context.render_text_centered(
            text,
            &mut cache.label2,
            PADDING,
            width - PADDING,
            [0.5 * width, TEXT_ALIGN_TOP],
            DEFAULT_TEXT_SIZE,
            solid(
                &component
                    .left_center_color
                    .unwrap_or(layout_state.text_color),
            ),
        ),
        TextState::Split(left, right) => context.render_key_value_component(
            left,
            &[],
            &mut cache.label1,
            right,
            &mut cache.label2,
            false,
            [width, height],
            component
                .left_center_color
                .unwrap_or(layout_state.text_color),
            component.right_color.unwrap_or(layout_state.text_color),
            component.display_two_rows || layout_state.direction == LayoutDirection::Horizontal,
        ),
    }
}
