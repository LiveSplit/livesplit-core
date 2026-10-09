use crate::{
    component::timer::State,
    rendering::{
        FillShader, RenderContext,
        consts::{BOTH_PADDINGS, PADDING},
        font::CachedLabel,
        resource::ResourceAllocator,
        scene::Layer,
    },
};

pub struct Cache<L> {
    time: CachedLabel<L>,
    fraction: CachedLabel<L>,
    time_size_hint: CachedLabel<L>,
    fraction_size_hint: CachedLabel<L>,
}

impl<L> Cache<L> {
    pub const fn new() -> Self {
        Self {
            time: CachedLabel::new(),
            fraction: CachedLabel::new(),
            time_size_hint: CachedLabel::new(),
            fraction_size_hint: CachedLabel::new(),
        }
    }
}

pub(in crate::rendering) fn render<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<A>,
    [width, height]: [f32; 2],
    component: &State,
) -> f32 {
    context.render_background([width, height], &component.background);

    let shader = FillShader::VerticalGradient(
        component.top_color.to_array(),
        component.bottom_color.to_array(),
    );

    let render_target = Layer::from_updates_frequently(component.updates_frequently);

    let time_width =
        context.measure_timer(&component.time_size_hint, &mut cache.time_size_hint, height);
    let fraction_width = context.measure_timer(
        &component.fraction_size_hint,
        &mut cache.fraction_size_hint,
        0.7 * height,
    );

    let text_width = time_width + fraction_width;
    let available_width = (width - BOTH_PADDINGS).max(0.0);
    let scale = if text_width > available_width {
        height * (available_width / text_width)
    } else {
        height
    };
    // Keep the timer vertically centered as both parts shrink together.
    let y = 0.85 * height - 0.35 * (height - scale);

    let x = context.render_timer(
        &component.fraction,
        &mut cache.fraction,
        render_target,
        [(width - PADDING).max(0.0), y],
        0.7 * scale,
        shader,
    );

    context.render_timer(
        &component.time,
        &mut cache.time,
        render_target,
        [x, y],
        scale,
        shader,
    )
}
