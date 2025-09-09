use crate::{
    component::blank_space::State,
    rendering::{RenderContext, resource::ResourceAllocator},
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<impl ResourceAllocator>,
    dim: [f32; 2],
    component: &State,
) {
    context.render_background(dim, &component.background);
}
