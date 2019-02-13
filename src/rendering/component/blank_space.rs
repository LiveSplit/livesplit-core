use crate::{
    component::blank_space::State,
    rendering::{Backend, RenderContext},
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl Backend>,
    dim: [f32; 2],
    component: &State,
) {
    context.render_rectangle([0.0, 0.0], dim, &component.background);
}
