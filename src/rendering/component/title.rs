use crate::{
    component::title::State,
    layout::LayoutState,
    rendering::{
        icon::Icon, solid, vertical_padding, Backend, RenderContext, BOTH_PADDINGS,
        DEFAULT_TEXT_SIZE, PADDING, TEXT_ALIGN_BOTTOM, TEXT_ALIGN_CENTER, TEXT_ALIGN_TOP,
    },
};

pub(in crate::rendering) fn render<B: Backend>(
    context: &mut RenderContext<'_, B>,
    [width, height]: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
    game_icon: &mut Option<Icon<B::Image>>,
) {
    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
    let text_color = component.text_color.unwrap_or(layout_state.text_color);
    let text_color = solid(&text_color);

    if let Some(icon) = &component.icon_change {
        if let Some(old_icon) = game_icon.take() {
            context.backend.free_image(old_icon.image);
        }
        *game_icon = context.create_icon(icon);
    }

    let left_bound = if let Some(icon) = game_icon {
        let vertical_padding = vertical_padding(height);
        let icon_size = height - 2.0 * vertical_padding;
        context.render_icon([PADDING, vertical_padding], [icon_size, icon_size], icon);
        BOTH_PADDINGS + icon_size
    } else {
        PADDING
    };

    let line_x = if component.is_centered {
        0.5 * width
    } else {
        left_bound
    };

    let attempts = match (component.finished_runs, component.attempts) {
        (Some(a), Some(b)) => format!("{}/{}", a, b),
        (Some(a), _) | (_, Some(a)) => a.to_string(),
        _ => String::new(),
    };
    let line2_end_x = context.render_numbers(
        &attempts,
        [width - PADDING, height + TEXT_ALIGN_BOTTOM],
        DEFAULT_TEXT_SIZE,
        text_color,
    ) - PADDING;

    let (line1_y, line1_end_x) = if !component.line2.is_empty() {
        let line2 = context.choose_abbreviation(
            component.line2.iter().map(|a| &**a),
            DEFAULT_TEXT_SIZE,
            line2_end_x - left_bound,
        );
        context.render_text_align(
            line2,
            left_bound,
            line2_end_x,
            [line_x, height + TEXT_ALIGN_BOTTOM],
            DEFAULT_TEXT_SIZE,
            component.is_centered,
            text_color,
        );
        (TEXT_ALIGN_TOP, width - PADDING)
    } else {
        (height / 2.0 + TEXT_ALIGN_CENTER, line2_end_x)
    };

    let line1 = context.choose_abbreviation(
        component.line1.iter().map(|a| &**a),
        DEFAULT_TEXT_SIZE,
        line1_end_x - left_bound,
    );

    context.render_text_align(
        line1,
        left_bound,
        line1_end_x,
        [line_x, line1_y],
        DEFAULT_TEXT_SIZE,
        component.is_centered,
        text_color,
    );
}
