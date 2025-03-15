use crate::{
    component::detailed_timer::State,
    layout::LayoutState,
    rendering::{
        FillShader,
        component::timer,
        consts::{vertical_padding, BOTH_PADDINGS, PADDING},
        font::CachedLabel,
        resource::ResourceAllocator,
        scene::Layer,
        solid, RenderContext,
    },
};

pub struct Cache<L> {
    timer: timer::Cache<L>,
    segment_timer: timer::Cache<L>,
    segment_name: CachedLabel<L>,
    comparison1_name: CachedLabel<L>,
    comparison2_name: CachedLabel<L>,
    comparison1_time: CachedLabel<L>,
    comparison2_time: CachedLabel<L>,
}

impl<L> Cache<L> {
    pub const fn new() -> Self {
        Self {
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
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<'_, A>,
    [width, height]: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
) {
    context.render_background([width, height], &component.background);
    
    let shadow_offset = [0.05, 0.05];
    let shadow_color = FillShader::SolidColor([0.0, 0.0, 0.0, 0.5]);

    let vertical_padding = vertical_padding(height);
    let icon_size = height - 2.0 * vertical_padding;

    let left_side = if let Some(icon) = context.create_image(&component.icon) {
        context.render_image([PADDING, vertical_padding], [icon_size, icon_size], icon);
        BOTH_PADDINGS + icon_size
    } else {
        PADDING
    };

    let total_height = component.timer.height + component.segment_timer.height;
    let top_height = (component.timer.height as f32 / total_height as f32) * height;
    let bottom_height = height - top_height;

    let timer_end = timer::render(
        &mut cache.timer,
        context,
        [width, top_height],
        &component.timer,
        layout_state
    );

    if let Some(segment_name) = &component.segment_name {
        let segment_name_color = solid(
            &component
                .segment_name_color
                .unwrap_or(layout_state.text_color),
        );

        context.render_text_ellipsis(
            segment_name,
            &mut cache.segment_name,
            [left_side, 0.6 * top_height],
            0.5 * top_height,
            segment_name_color,
            shadow_offset,
            shadow_color,
            timer_end,
            layout_state
        );
    }

    context.translate(0.0, top_height);

    let segment_timer_end = timer::render(
        &mut cache.segment_timer,
        context,
        [width, bottom_height],
        &component.segment_timer,
        layout_state
    );

    context.translate(0.0, -top_height);

    let mut name_end = 0.0;
    let comparison_text_scale = 0.5 * bottom_height;
    let comparison2_y = 0.8 * bottom_height + top_height;
    let mut time_width = 0.0;

    let comparison_names_color = solid(
        &component
            .comparison_names_color
            .unwrap_or(layout_state.text_color),
    );

    let comparison1_y = if let Some(comparison) = &component.comparison2 {
        name_end = context
            .render_text_ellipsis(
                &comparison.name,
                &mut cache.comparison2_name,
                [left_side, comparison2_y],
                comparison_text_scale,
                comparison_names_color,
                shadow_offset,
                shadow_color,
                segment_timer_end,
                layout_state
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
                comparison_names_color,
                shadow_offset,
                shadow_color,
                segment_timer_end,
                layout_state
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

    let comparison_times_color = solid(
        &component
            .comparison_times_color
            .unwrap_or(layout_state.text_color),
    );
    if let Some(comparison) = &component.comparison2 {
        context.render_numbers(
            &comparison.time,
            &mut cache.comparison2_time,
            Layer::Bottom,
            [time_x, comparison2_y],
            comparison_text_scale,
            comparison_times_color,
            shadow_offset,
            shadow_color,
            layout_state
        );
    }
    if let Some(comparison) = &component.comparison1 {
        context.render_numbers(
            &comparison.time,
            &mut cache.comparison1_time,
            Layer::Bottom,
            [time_x, comparison1_y],
            comparison_text_scale,
            comparison_times_color,
            shadow_offset,
            shadow_color,
            layout_state
        );
    }
}
