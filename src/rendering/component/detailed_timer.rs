use crate::{
    component::detailed_timer::State,
    layout::LayoutState,
    rendering::{
        component::timer, icon::Icon, vertical_padding, Backend, RenderContext, BOTH_PADDINGS,
        PADDING,
    },
};

pub(in crate::rendering) fn render<B: Backend>(
    context: &mut RenderContext<'_, B>,
    [width, height]: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
    detailed_timer_icon: &mut Option<Icon<B::Texture>>,
) {
    context.render_rectangle([0.0, 0.0], [width, height], &component.background);

    let vertical_padding = vertical_padding(height);
    let icon_size = height - 2.0 * vertical_padding;

    if let Some(icon) = &component.icon_change {
        if let Some(old_icon) = detailed_timer_icon.take() {
            context.backend.free_texture(old_icon.texture);
        }
        *detailed_timer_icon = context.create_icon(&icon);
    }

    let left_side = if let Some(icon) = detailed_timer_icon {
        context.render_icon([PADDING, vertical_padding], [icon_size, icon_size], icon);
        BOTH_PADDINGS + icon_size
    } else {
        PADDING
    };

    let top_height = 0.55 * height;
    let bottom_height = height - top_height;

    let timer_end = timer::render(context, [width, top_height], &component.timer);

    if let Some(segment_name) = &component.segment_name {
        context.render_text_ellipsis(
            &segment_name,
            [left_side, 0.6 * top_height],
            0.5 * top_height,
            [layout_state.text_color; 2],
            timer_end,
        );
    }

    context.translate(0.0, top_height);

    let segment_timer_end =
        timer::render(context, [width, bottom_height], &component.segment_timer);

    context.translate(0.0, -top_height);

    let mut name_end = 0.0;
    let comparison_text_scale = 0.5 * bottom_height;
    let comparison2_y = 0.8 * bottom_height + top_height;
    let mut time_width = 0.0;

    let comparison1_y = if let Some(comparison) = &component.comparison2 {
        name_end = context
            .render_text_ellipsis(
                &comparison.name,
                [left_side, comparison2_y],
                comparison_text_scale,
                [layout_state.text_color; 2],
                segment_timer_end,
            )
            .max(name_end);

        time_width = context
            .measure_numbers(&comparison.time, comparison_text_scale)
            .max(time_width);

        comparison2_y - comparison_text_scale
    } else {
        comparison2_y
    };

    if let Some(comparison) = &component.comparison1 {
        name_end = context
            .render_text_ellipsis(
                &comparison.name,
                [left_side, comparison1_y],
                comparison_text_scale,
                [layout_state.text_color; 2],
                segment_timer_end,
            )
            .max(name_end);

        time_width = context
            .measure_numbers(&comparison.time, comparison_text_scale)
            .max(time_width);
    }

    let time_x = name_end + PADDING + time_width;

    if let Some(comparison) = &component.comparison2 {
        context.render_numbers(
            &comparison.time,
            [time_x, comparison2_y],
            comparison_text_scale,
            [layout_state.text_color; 2],
        );
    }
    if let Some(comparison) = &component.comparison1 {
        context.render_numbers(
            &comparison.time,
            [time_x, comparison1_y],
            comparison_text_scale,
            [layout_state.text_color; 2],
        );
    }
}
