use crate::{
    component::separator::State,
    layout::LayoutState,
    rendering::{RenderContext, resource::ResourceAllocator},
    settings::Gradient,
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<impl ResourceAllocator>,
    dim: [f32; 2],
    _component: &State,
    layout_state: &LayoutState,
) {
    context.render_background(dim, &Gradient::Plain(layout_state.separators_color));
}
