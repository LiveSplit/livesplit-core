use crate::{
    component::timer::State,
    rendering::{
        consts::PADDING, resource::ResourceAllocator, scene::Layer, FillShader, RenderContext,
    },
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl ResourceAllocator>,
    [width, height]: [f32; 2],
    component: &State,
) -> f32 {
    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
    let shader = FillShader::VerticalGradient(
        component.top_color.to_array(),
        component.bottom_color.to_array(),
    );
    let render_target = Layer::from_updates_frequently(component.updates_frequently);
    let x = context.render_timer(
        &component.fraction,
        render_target,
        [width - PADDING, 0.85 * height],
        0.8 * height,
        shader,
    );
    context.render_timer(
        &component.time,
        render_target,
        [x, 0.85 * height],
        1.2 * height,
        shader,
    )
}
