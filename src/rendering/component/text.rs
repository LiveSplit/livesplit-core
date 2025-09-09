use crate::{
    component::text::{State, TextState},
    layout::{LayoutDirection, LayoutState},
    rendering::{
        RenderContext,
        consts::{DEFAULT_TEXT_SIZE, PADDING, TEXT_ALIGN_TOP},
        font::{AbbreviatedLabel, CachedLabel},
        resource::ResourceAllocator,
        solid,
    },
};

pub struct Cache<L> {
    label1: AbbreviatedLabel<L>,
    label2: CachedLabel<L>,
}

impl<L> Cache<L> {
    pub const fn new() -> Self {
        Self {
            label1: AbbreviatedLabel::new(),
            label2: CachedLabel::new(),
        }
    }
}

pub(in crate::rendering) fn render<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<A>,
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
