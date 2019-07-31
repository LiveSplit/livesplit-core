use crate::{
    component::previous_segment::State,
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
    let (a, b);
    let abbreviations = if component.text.starts_with("Previous Segment") {
        a = [
            &component.text,
            "Previous Segment",
            "Prev. Segment",
            "Prev. Seg.",
        ];
        &a[..]
    } else {
        b = [&component.text, "Live Segment", "Live Seg."];
        &b[..]
    };
    context.render_numerical_key_value_component(
        abbreviations,
        &component.time,
        dim,
        component.label_color.unwrap_or(layout_state.text_color),
        component.visual_color,
        component.display_two_rows || layout_state.direction == LayoutDirection::Horizontal,
    );
}
