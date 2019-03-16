use crate::{
    component::possible_time_save::State,
    layout::LayoutState,
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
    let abbreviations = if component.text.starts_with("Total") {
        a = [
            &component.text,
            "Total Possible Time Save",
            "Possible Time Save",
            "Poss. Time Save",
            "Time Save",
        ];
        &a[..]
    } else {
        b = [
            &component.text,
            "Possible Time Save",
            "Poss. Time Save",
            "Time Save",
        ];
        &b[..]
    };
    context.render_info_time_component(
        abbreviations,
        &component.time,
        component.label_color.unwrap_or(layout_state.text_color),
        component.value_color.unwrap_or(layout_state.text_color),
        component.display_two_rows,
    );
}
