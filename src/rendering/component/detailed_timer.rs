use crate::{
    component::detailed_timer::State,
    layout::LayoutState,
    rendering::{
        component::timer,
        consts::{vertical_padding, BOTH_PADDINGS, PADDING},
        font::CachedLabel,
        icon::Icon,
        resource::ResourceAllocator,
        scene::Layer,
        solid, RenderContext,
    },
};

pub struct Cache<I, L> {
    icon: Option<Icon<I>>,
    timer: timer::Cache<I, L>,
    segment_timer: timer::Cache<I, L>,
    segment_name: CachedLabel<L>,
    comparison1_name: CachedLabel<L>,
    comparison2_name: CachedLabel<L>,
    comparison1_time: CachedLabel<L>,
    comparison2_time: CachedLabel<L>,
}

impl<I, L> Cache<I, L> {
    pub const fn new() -> Self {
        Self {
            icon: None,
            timer: timer::Cache::new(),
            segment_timer: timer::Cache::new(),
            segment_name: CachedLabel::new(),
            comparison1_name: CachedLabel::new(),
            comparison2_name: CachedLabel::new(),
            comparison1_time: CachedLabel::new(),
            comparison2_time: CachedLabel::new(),
        }
    }
}

pub(in crate::rendering) fn render<A: ResourceAllocator>(
    cache: &mut Cache<A::Image, A::Label>,
    context: &mut RenderContext<'_, A>,
    [width, height]: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
) {
    context.render_background([width, height], &component.background);

    let text_color = solid(&layout_state.text_color);

    let vertical_padding = vertical_padding(height);
    let icon_size = height - 2.0 * vertical_padding;

    if let Some(icon) = &component.icon_change {
        cache.icon = context.create_icon(icon);
    }

    let left_side = if let Some(icon) = &cache.icon {
        context.render_icon([PADDING, vertical_padding], [icon_size, icon_size], icon);
        BOTH_PADDINGS + icon_size
    } else {
        PADDING
    };

    let top_height = 0.55 * height;
    let bottom_height = height - top_height;

    let timer_end = timer::render(
        &mut cache.timer,
        context,
        [width, top_height],
        &component.timer,
    );

    if let Some(segment_name) = &component.segment_name {
        context.render_text_ellipsis(
            segment_name,
            &mut cache.segment_name,
            [left_side, 0.6 * top_height],
            0.5 * top_height,
            text_color,
            timer_end,
        );
    }

    context.translate(0.0, top_height);

    let segment_timer_end = timer::render(
        &mut cache.segment_timer,
        context,
        [width, bottom_height],
        &component.segment_timer,
    );

    context.translate(0.0, -top_height);

    let mut name_end = 0.0;
    let comparison_text_scale = 0.5 * bottom_height;
    let comparison2_y = 0.8 * bottom_height + top_height;
    let mut time_width = 0.0;

    let comparison1_y = if let Some(comparison) = &component.comparison2 {
        name_end = context
            .render_text_ellipsis(
                &comparison.name,
                &mut cache.comparison2_name,
                [left_side, comparison2_y],
                comparison_text_scale,
                text_color,
                segment_timer_end,
            )
            .max(name_end);

        time_width = context
            .measure_numbers(
                &comparison.time,
                &mut cache.comparison2_time,
                comparison_text_scale,
            )
            .max(time_width);

        comparison2_y - comparison_text_scale
    } else {
        comparison2_y
    };

    if let Some(comparison) = &component.comparison1 {
        name_end = context
            .render_text_ellipsis(
                &comparison.name,
                &mut cache.comparison1_name,
                [left_side, comparison1_y],
                comparison_text_scale,
                text_color,
                segment_timer_end,
            )
            .max(name_end);

        time_width = context
            .measure_numbers(
                &comparison.time,
                &mut cache.comparison1_time,
                comparison_text_scale,
            )
            .max(time_width);
    }

    let time_x = name_end + PADDING + time_width;

    if let Some(comparison) = &component.comparison2 {
        context.render_numbers(
            &comparison.time,
            &mut cache.comparison2_time,
            Layer::Bottom,
            [time_x, comparison2_y],
            comparison_text_scale,
            text_color,
        );
    }
    if let Some(comparison) = &component.comparison1 {
        context.render_numbers(
            &comparison.time,
            &mut cache.comparison1_time,
            Layer::Bottom,
            [time_x, comparison1_y],
            comparison_text_scale,
            text_color,
        );
    }
}
