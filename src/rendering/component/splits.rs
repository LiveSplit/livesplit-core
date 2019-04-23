use crate::{
    component::splits::State,
    layout::{LayoutDirection, LayoutState},
    rendering::{
        icon::Icon, vertical_padding, Backend, RenderContext, BOTH_PADDINGS,
        DEFAULT_COMPONENT_HEIGHT, DEFAULT_TEXT_SIZE, PADDING, TEXT_ALIGN_BOTTOM, TEXT_ALIGN_TOP,
        THIN_SEPARATOR_THICKNESS, TWO_ROW_HEIGHT,
    },
    settings::{Gradient, ListGradient},
};

pub const COLUMN_WIDTH: f32 = 3.0;

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

    let display_two_rows =
        component.display_two_rows || layout_state.direction == LayoutDirection::Horizontal;

    let split_height = if display_two_rows {
        TWO_ROW_HEIGHT
    } else {
        DEFAULT_COMPONENT_HEIGHT
    };

    let vertical_padding = vertical_padding(split_height);

    let (split_width, (delta_x, delta_y), separator_pos, split_background_bottom_right, icon_y) =
        if layout_state.direction == LayoutDirection::Horizontal {
            let split_width = width / component.splits.len() as f32;
            (
                split_width,
                (split_width, 0.0),
                [split_width - THIN_SEPARATOR_THICKNESS, 0.0],
                [split_width - THIN_SEPARATOR_THICKNESS, split_height],
                vertical_padding,
            )
        } else {
            (
                width,
                (0.0, split_height),
                [0.0, split_height - THIN_SEPARATOR_THICKNESS],
                [width, split_height - THIN_SEPARATOR_THICKNESS],
                vertical_padding - 0.5 * THIN_SEPARATOR_THICKNESS,
            )
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

    if let Some(column_labels) = &component.column_labels {
        if layout_state.direction == LayoutDirection::Vertical {
            let mut right_x = width - PADDING;
            for label in column_labels {
                let left_x = right_x - COLUMN_WIDTH;
                context.render_text_right_align(
                    label,
                    [right_x, TEXT_ALIGN_TOP],
                    DEFAULT_TEXT_SIZE,
                    [layout_state.text_color; 2],
                );
                right_x = left_x;
            }

            context.translate(0.0, DEFAULT_COMPONENT_HEIGHT);
            context.render_rectangle(
                [0.0, -THIN_SEPARATOR_THICKNESS],
                [width, THIN_SEPARATOR_THICKNESS],
                &Gradient::Plain(layout_state.separators_color),
            );
        }
    }

    let icon_size = split_height - 2.0 * vertical_padding;
    let icon_right = if component.has_icons {
        BOTH_PADDINGS + icon_size
    } else {
        PADDING
    };

    for (i, split) in component.splits.iter().enumerate() {
        if component.show_thin_separators && i + 1 != component.splits.len() {
            context.render_rectangle(
                separator_pos,
                [split_width, split_height],
                &Gradient::Plain(layout_state.thin_separators_color),
            );
        }

        if split.is_current_split {
            context.render_rectangle(
                [0.0, 0.0],
                [split_width, split_height],
                &component.current_split_gradient,
            );
        } else if let Some((even, odd)) = &split_background {
            let color = if split.index % 2 == 0 { even } else { odd };
            context.render_rectangle([0.0, 0.0], split_background_bottom_right, color);
        }

        {
            if let Some(Some(icon)) = split_icons.get(split.index) {
                context.render_icon([PADDING, icon_y], [icon_size, icon_size], icon);
            }

            let mut left_x = split_width - PADDING;
            let mut right_x = left_x;
            for column in &split.columns {
                if !column.value.is_empty() {
                    left_x = context.render_numbers(
                        &column.value,
                        [right_x, split_height + TEXT_ALIGN_BOTTOM],
                        DEFAULT_TEXT_SIZE,
                        [column.visual_color; 2],
                    );
                }
                right_x -= COLUMN_WIDTH;
            }

            if display_two_rows {
                left_x = split_width;
            }

            context.render_text_ellipsis(
                &split.name,
                [icon_right, TEXT_ALIGN_TOP],
                DEFAULT_TEXT_SIZE,
                [layout_state.text_color; 2],
                left_x - PADDING,
            );
        }
        context.translate(delta_x, delta_y);
    }
    if component.show_final_separator {
        let (pos, end) = if layout_state.direction == LayoutDirection::Horizontal {
            (
                [-split_width - THIN_SEPARATOR_THICKNESS, 0.0],
                [-split_width + THIN_SEPARATOR_THICKNESS, split_height],
            )
        } else {
            (
                [0.0, -split_height - THIN_SEPARATOR_THICKNESS],
                [split_width, -split_height + THIN_SEPARATOR_THICKNESS],
            )
        };
        context.render_rectangle(pos, end, &Gradient::Plain(layout_state.separators_color));
    }
    context.transform = transform;
}
