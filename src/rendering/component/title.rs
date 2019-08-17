use {
    crate::{
        component::title::State,
        layout::LayoutState,
        rendering::{
            icon::Icon, vertical_padding, Backend, RenderContext, BOTH_PADDINGS, DEFAULT_TEXT_SIZE,
            PADDING, TEXT_ALIGN_BOTTOM, TEXT_ALIGN_CENTER, TEXT_ALIGN_TOP,
        },
    },
    livesplit_title_abbreviations::abbreviate,
};

pub(in crate::rendering) fn render<B: Backend>(
    context: &mut RenderContext<'_, B>,
    [width, height]: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
    game_icon: &mut Option<Icon<B::Texture>>,
) {
    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
    let text_color = component.text_color.unwrap_or(layout_state.text_color);

    if let Some(icon) = &component.icon_change {
        if let Some(old_icon) = game_icon.take() {
            context.backend.free_texture(old_icon.texture);
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

    let (line_x, align) = if component.is_centered {
        (0.5 * width, 0.5)
    } else {
        (left_bound, 0.0)
    };

    // FIXME: For a single line the component provides both the game and category
    // in a single string, which makes it hard for us to properly abbreviate it.
    // We may want to rethink merging both values into a single string because
    // of that. https://github.com/LiveSplit/livesplit-core/issues/170
    let abbreviations = abbreviate(&component.line1);
    let line1 = context.choose_abbreviation(
        abbreviations.iter().map(String::as_str),
        DEFAULT_TEXT_SIZE,
        width - PADDING - left_bound,
    );

    let attempts = match (component.finished_runs, component.attempts) {
        (Some(a), Some(b)) => format!("{}/{}", a, b),
        (Some(a), _) | (_, Some(a)) => a.to_string(),
        _ => String::new(),
    };
    let line2_end_x = context.render_numbers(
        &attempts,
        [width - PADDING, height + TEXT_ALIGN_BOTTOM],
        DEFAULT_TEXT_SIZE,
        [text_color; 2],
    );

    let (line1_y, line1_end_x) = if let Some(line2) = &component.line2 {
        context.render_text_align(
            line2,
            left_bound,
            line2_end_x - PADDING,
            [line_x, height + TEXT_ALIGN_BOTTOM],
            DEFAULT_TEXT_SIZE,
            align,
            text_color,
        );
        (TEXT_ALIGN_TOP, width - PADDING)
    } else {
        (height / 2.0 + TEXT_ALIGN_CENTER, line2_end_x - PADDING)
    };

    context.render_text_align(
        line1,
        left_bound,
        line1_end_x,
        [line_x, line1_y],
        DEFAULT_TEXT_SIZE,
        align,
        text_color,
    );
}
