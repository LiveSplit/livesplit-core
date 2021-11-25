use crate::{
    component::blank_space::State,
    rendering::{resource::ResourceAllocator, RenderContext},
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl ResourceAllocator>,
    dim: [f32; 2],
    component: &State,
) {
    context.render_background(dim, &component.background);
}
