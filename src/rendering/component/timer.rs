use crate::{
    component::timer::State,
    rendering::{Backend, FillShader, RenderContext, PADDING},
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl Backend>,
    [width, height]: [f32; 2],
    component: &State,
) -> f32 {
    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
    let shader = FillShader::VerticalGradient(
        component.top_color.to_array(),
        component.bottom_color.to_array(),
    );
    let x = context.render_timer(
        &component.fraction,
        [width - PADDING, 0.85 * height],
        0.8 * height,
        shader,
    );
    context.render_timer(&component.time, [x, 0.85 * height], 1.2 * height, shader)
}
