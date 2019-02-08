use {
    crate::{
        component::title::State,
        layout::LayoutState,
        rendering::{Backend, IndexPair, RenderContext, MARGIN},
    },
    livesplit_title_abbreviations::abbreviate,
    ordered_float::OrderedFloat,
};

pub(in crate::rendering) fn render(
    context: &mut RenderContext<'_, impl Backend>,
    [width, height]: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
    game_icon: &mut Option<(IndexPair, f32)>,
) {
    context.render_rectangle([0.0, 0.0], [width, height], &component.background);
    let text_color = component.text_color.unwrap_or(layout_state.text_color);

    // TODO: For now let's just assume there's an icon.

    if let Some(url) = &component.icon_change {
        if let Some((old_texture, _)) = game_icon.take() {
            context.backend.free_texture(old_texture);
        }
        *game_icon = context.create_texture(url);
    }

    let left_bound = if let Some(icon) = *game_icon {
        let icon_size = 2.0 - 2.0 * MARGIN;
        context.render_image([MARGIN, MARGIN], [icon_size, icon_size], icon);
        2.0 * MARGIN + icon_size
    } else {
        MARGIN
    };

    let (x, align) = if component.is_centered {
        (0.5 * width, 0.5)
    } else {
        (left_bound, 0.0)
    };

    // TODO: Positioning for a single line
    // TODO: Abbreviations with single line are weird
    let abbreviations = abbreviate(&component.line1);
    let line1 = abbreviations
        .iter()
        .map(|line| (line.as_str(), context.measure_text(line, 0.8)))
        .filter(|&(_, len)| len < width - MARGIN - left_bound)
        .max_by_key(|&(_, len)| OrderedFloat(len))
        .map(|(line, _)| line)
        .unwrap_or(&component.line1);

    let attempts = match (component.finished_runs, component.attempts) {
        (Some(a), Some(b)) => format!("{}/{}", a, b),
        (Some(a), _) | (_, Some(a)) => a.to_string(),
        _ => String::new(),
    };
    let line2_end_x =
        context.render_numbers(&attempts, [width - MARGIN, 1.63], 0.8, [text_color; 2]);

    context.render_text_align(
        line1,
        left_bound,
        width + MARGIN, // TODO: Should be - MARGIN
        [x, 0.83],
        0.8,
        align,
        text_color,
    );
    if let Some(line2) = &component.line2 {
        context.render_text_align(
            line2,
            left_bound,
            line2_end_x - MARGIN,
            [x, 1.63],
            0.8,
            align,
            text_color,
        );
    }
}
