use crate::{
    component::splits::State,
    layout::LayoutState,
    rendering::{icon::Icon, Backend, RenderContext, MARGIN, TWO_ROW_HEIGHT},
    settings::{Gradient, ListGradient},
};

pub(in crate::rendering) fn render<B: Backend>(
    context: &mut RenderContext<'_, B>,
    [width, height]: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
    split_icons: &mut Vec<Option<Icon<B::Texture>>>,
) {
    let split_background = match component.background {
        ListGradient::Same(gradient) => {
            context.render_rectangle([0.0, 0.0], [width, height], &gradient);
            None
        }
        ListGradient::Alternating(even, odd) => Some((Gradient::Plain(even), Gradient::Plain(odd))),
    };

    let width = context.width;

    let split_height = if component.display_two_rows {
        TWO_ROW_HEIGHT
    } else {
        1.0
    };
    let transform = context.transform;

    for icon_change in &component.icon_changes {
        if icon_change.segment_index >= split_icons.len() {
            split_icons.extend((0..=icon_change.segment_index - split_icons.len()).map(|_| None));
        }
        let icon = &mut split_icons[icon_change.segment_index];
        if let Some(old_icon) = icon.take() {
            context.backend.free_texture(old_icon.texture);
        }
        *icon = context.create_icon(&icon_change.icon);
    }

    const COLUMN_WIDTH: f32 = 3.0;

    if let Some(column_labels) = &component.column_labels {
        let mut right_x = width - MARGIN;
        for label in column_labels {
            let left_x = right_x - COLUMN_WIDTH;
            context.render_text_right_align(
                label,
                [right_x, 0.7],
                0.8,
                [layout_state.text_color; 2],
            );
            right_x = left_x;
        }

        context.translate(0.0, 1.0);
        context.render_rectangle(
            [0.0, -0.05],
            [width, 0.05],
            &Gradient::Plain(layout_state.separators_color),
        );
    }

    let icon_size = split_height - 0.2;
    let icon_right = if component.has_icons { 2.0 * MARGIN + icon_size } else { MARGIN };

    for (i, split) in component.splits.iter().enumerate() {
        if component.show_thin_separators && i + 1 != component.splits.len() {
            context.render_rectangle(
                [0.0, split_height - 0.05],
                [width, split_height],
                &Gradient::Plain(layout_state.thin_separators_color),
            );
        }

        if split.is_current_split {
            context.render_rectangle(
                [0.0, 0.0],
                [width, split_height],
                &component.current_split_gradient,
            );
        } else if let Some((even, odd)) = &split_background {
            let color = if split.index % 2 == 0 { even } else { odd };
            context.render_rectangle([0.0, 0.0], [width, split_height - 0.05], color);
        }

        {
            if let Some(Some(icon)) = split_icons.get(split.index) {
                context.render_icon([MARGIN, 0.1 - 0.5 * 0.05], [icon_size, icon_size], icon);
            }

            let mut left_x = width - MARGIN;
            let mut right_x = left_x;
            for column in &split.columns {
                if !column.value.is_empty() {
                    left_x = context.render_numbers(
                        &column.value,
                        [right_x, split_height - 0.3],
                        0.8,
                        [column.visual_color; 2],
                    );
                }
                right_x -= COLUMN_WIDTH;
            }

            if component.display_two_rows {
                left_x = width;
            }

            context.render_text_ellipsis(
                &split.name,
                [icon_right, 0.7],
                0.8,
                [layout_state.text_color; 2],
                left_x - MARGIN,
            );
        }
        context.translate(0.0, split_height);
    }
    if component.show_final_separator {
        context.render_rectangle(
            [0.0, -split_height - 0.05],
            [width, -split_height + 0.05],
            &Gradient::Plain(layout_state.separators_color),
        );
    }
    context.transform = transform;
}
