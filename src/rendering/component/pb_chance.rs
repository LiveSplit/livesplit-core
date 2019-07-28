use crate::{
    component::pb_chance::State,
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
    let abbreviations = [component.text.as_str()];
    // TODO: It's not really a time though?!
    context.render_info_time_component(
        &abbreviations,
        &component.pb_chance,
        dim,
        component.label_color.unwrap_or(layout_state.text_color),
        component.value_color.unwrap_or(layout_state.text_color),
        component.display_two_rows || layout_state.direction == LayoutDirection::Horizontal,
    );
}
