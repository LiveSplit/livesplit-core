use crate::{
    component::timer::State,
    rendering::{
        consts::PADDING, font::CachedLabel, resource::ResourceAllocator, scene::Layer, FillShader,
        RenderContext,
    },
};

pub struct Cache<L> {
    time: CachedLabel<L>,
    fraction: CachedLabel<L>,
}

impl<L> Cache<L> {
    pub const fn new() -> Self {
        Self {
            time: CachedLabel::new(),
            fraction: CachedLabel::new(),
        }
    }
}

pub(in crate::rendering) fn render<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<'_, A>,
    [width, height]: [f32; 2],
    component: &State,
) -> f32 {
    context.render_background([width, height], &component.background);
    let shader = FillShader::VerticalGradient(
        component.top_color.to_array(),
        component.bottom_color.to_array(),
    );
    let render_target = Layer::from_updates_frequently(component.updates_frequently);
    let x = context.render_timer(
        &component.fraction,
        &mut cache.fraction,
        render_target,
        [width - PADDING, 0.85 * height],
        0.7 * height,
        shader,
    );
    context.render_timer(
        &component.time,
        &mut cache.time,
        render_target,
        [x, 0.85 * height],
        height,
        shader,
    )
}
