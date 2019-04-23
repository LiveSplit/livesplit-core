use crate::{
    comparison,
    component::delta::State,
    layout::{LayoutDirection, LayoutState},
    rendering::{Backend, RenderContext},
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl Backend>,
    dim: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
) {
    context.render_rectangle([0.0, 0.0], dim, &component.background);
    context.render_info_time_component(
        &[&component.text, comparison::shorten(&component.text)],
        &component.time,
        dim,
        component.label_color.unwrap_or(layout_state.text_color),
        component.visual_color,
        component.display_two_rows || layout_state.direction == LayoutDirection::Horizontal,
    );
}
