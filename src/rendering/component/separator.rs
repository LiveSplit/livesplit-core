use crate::{
    component::separator::State,
    layout::LayoutState,
    rendering::{resource::ResourceAllocator, RenderContext},
    settings::Gradient,
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl ResourceAllocator>,
    dim: [f32; 2],
    _component: &State,
    layout_state: &LayoutState,
) {
    context.render_background(dim, &Gradient::Plain(layout_state.separators_color));
}
