use crate::{
    component::separator::State,
    layout::LayoutState,
    rendering::{Backend, RenderContext},
    settings::Gradient,
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl Backend>,
    dim: [f32; 2],
    _component: &State,
    layout_state: &LayoutState,
) {
    context.render_rectangle(
        [0.0, 0.0],
        dim,
        &Gradient::Plain(layout_state.separators_color),
    );
}
