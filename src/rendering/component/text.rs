use crate::{
    component::text::{State, Text},
    layout::LayoutState,
    rendering::{Backend, RenderContext, MARGIN},
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
            MARGIN,
            width - MARGIN,
            [0.5 * width, 0.7],
            0.8,
            component
                .left_center_color
                .unwrap_or(layout_state.text_color),
        ),
        Text::Split(left, right) => context.render_info_text_component(
            &[&left],
            &right,
            component
                .left_center_color
                .unwrap_or(layout_state.text_color),
            component.right_color.unwrap_or(layout_state.text_color),
            component.display_two_rows,
        ),
    }
}
