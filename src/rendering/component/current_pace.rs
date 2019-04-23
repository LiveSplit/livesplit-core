use crate::{
    component::current_pace::State,
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
    let (a2, a3, a4);
    let abbreviations = match &*component.text {
        "Best Possible Time" => {
            a4 = [&component.text, "Best Poss. Time", "Best Time", "BPT"];
            &a4[..]
        }
        "Worst Possible Time" => {
            a3 = [&component.text, "Worst Poss. Time", "Worst Time"];
            &a3[..]
        }
        "Predicted Time" => {
            a2 = [&component.text, "Pred. Time"];
            &a2[..]
        }
        _ => {
            a4 = [&component.text, "Current Pace", "Cur. Pace", "Pace"];
            &a4[..]
        }
    };
    context.render_info_time_component(
        abbreviations,
        &component.time,
        dim,
        component.label_color.unwrap_or(layout_state.text_color),
        component.value_color.unwrap_or(layout_state.text_color),
        component.display_two_rows || layout_state.direction == LayoutDirection::Horizontal,
    );
}
