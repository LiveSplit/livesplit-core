use crate::{
    component::text::{State, Text},
    layout::{LayoutDirection, LayoutState},
    rendering::{Backend, RenderContext, DEFAULT_TEXT_SIZE, PADDING, TEXT_ALIGN_TOP},
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl Backend>,
    [width, height]: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
) {
    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
    match &component.text {
        Text::Center(text) => context.render_text_centered(
            text,
            PADDING,
            width - PADDING,
            [0.5 * width, TEXT_ALIGN_TOP],
            DEFAULT_TEXT_SIZE,
            component
                .left_center_color
                .unwrap_or(layout_state.text_color),
        ),
        Text::Split(left, right) => context.render_info_text_component(
            &[&left],
            &right,
            [width, height],
            component
                .left_center_color
                .unwrap_or(layout_state.text_color),
            component.right_color.unwrap_or(layout_state.text_color),
            component.display_two_rows || layout_state.direction == LayoutDirection::Horizontal,
        ),
    }
}
