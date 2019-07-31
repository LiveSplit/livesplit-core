use crate::{
    component::sum_of_best::State,
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
    context.render_numerical_key_value_component(
        &[&component.text, "Sum of Best", "SoB"],
        &component.time,
        dim,
        component.label_color.unwrap_or(layout_state.text_color),
        component.value_color.unwrap_or(layout_state.text_color),
        component.display_two_rows || layout_state.direction == LayoutDirection::Horizontal,
    );
}
