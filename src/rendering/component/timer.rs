use crate::{
    component::timer::State,
    rendering::{decode_color, Backend, FillShader, RenderContext, PADDING},
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl Backend>,
    [width, height]: [f32; 2],
    component: &State,
) -> f32 {
    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
    let shader = FillShader::VerticalGradient(
        decode_color(&component.top_color),
        decode_color(&component.bottom_color),
    );
    let x = context.render_timer(
        &component.fraction,
        [width - PADDING, 0.85 * height],
        0.8 * height,
        shader,
    );
    context.render_timer(&component.time, [x, 0.85 * height], 1.2 * height, shader)
}
