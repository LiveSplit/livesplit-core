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
    let text_width = measure(cache, context, height, component);
    let available_width = (width - BOTH_PADDINGS).max(0.0);
    let scale = if text_width > available_width {
        height * (available_width / text_width)
    } else {
        height
    };
    // Keep the timer vertically centered as both parts shrink together.
    let y = 0.85 * height - 0.35 * (height - scale);

    render_fitted(cache, context, [width, height], component, scale, y)
}

pub(in crate::rendering) fn render_fitted<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<A>,
    [width, height]: [f32; 2],
    component: &State,
    scale: f32,
    y: f32,
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

/// Measure the displayed time only for its horizontal edge. Vertical placement
/// must use a fixed reference covering every digit, not these changing glyphs.
pub(in crate::rendering) fn measure_current<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<A>,
    component: &State,
) -> f32 {
    context.measure_timer(&component.time, &mut cache.time, 1.0)
        + context.measure_timer(&component.fraction, &mut cache.fraction, 0.7)
}

/// Measure the fraction hint at the same relative scale used to render it.
pub(in crate::rendering) fn measure_fraction_size_hint<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<A>,
    height: f32,
    component: &State,
) -> f32 {
    context.measure_timer(
        &component.fraction_size_hint,
        &mut cache.fraction_size_hint,
        0.7 * height,
    )
}

pub(in crate::rendering) fn measure<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<A>,
    height: f32,
    component: &State,
) -> f32 {
    context.measure_timer(&component.time_size_hint, &mut cache.time_size_hint, height)
        + measure_fraction_size_hint(cache, context, height, component)
}
