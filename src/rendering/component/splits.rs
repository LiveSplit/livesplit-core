use core::iter;

use crate::{
    component::splits::State,
    layout::{LayoutDirection, LayoutState},
    platform::prelude::*,
    rendering::{
        consts::{
            vertical_padding, BOTH_PADDINGS, DEFAULT_COMPONENT_HEIGHT, DEFAULT_TEXT_SIZE, PADDING,
            TEXT_ALIGN_BOTTOM, TEXT_ALIGN_TOP, THIN_SEPARATOR_THICKNESS, TWO_ROW_HEIGHT,
        },
        font::CachedLabel,
        resource::ResourceAllocator,
        scene::Layer,
        solid, RenderContext,
    },
    settings::{Gradient, ListGradient},
};

pub struct Cache<L> {
    splits: Vec<SplitCache<L>>,
    column_labels: Vec<CachedLabel<L>>,
    column_width_label: CachedLabel<L>,
    column_label_widths: Vec<f32>,
}

struct SplitCache<L> {
    name: CachedLabel<L>,
    columns: Vec<CachedLabel<L>>,
}

impl<L> SplitCache<L> {
    const fn new() -> Self {
        Self {
            name: CachedLabel::new(),
            columns: Vec::new(),
        }
    }
}

impl<L> Cache<L> {
    pub const fn new() -> Self {
        Self {
            splits: Vec::new(),
            column_labels: Vec::new(),
            column_width_label: CachedLabel::new(),
            column_label_widths: Vec::new(),
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
    const COLUMN_PADDING: f32 = 0.2;
    let max_column_width =
        context.measure_numbers("88:88:88", &mut cache.column_width_label, DEFAULT_TEXT_SIZE);

    let text_color = solid(&layout_state.text_color);

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

    cache.column_label_widths.clear();

    if let Some(column_labels) = &component.column_labels {
        if layout_state.direction == LayoutDirection::Vertical {
            cache
                .column_labels
                .resize_with(column_labels.len(), CachedLabel::new);

            let mut right_x = width - PADDING;
            for (label, column_cache) in column_labels.iter().zip(&mut cache.column_labels) {
                // FIXME: The column width should depend on the column type too.
                let left_x = context.render_text_right_align(
                    label,
                    column_cache,
                    Layer::Bottom,
                    [right_x, TEXT_ALIGN_TOP],
                    DEFAULT_TEXT_SIZE,
                    text_color,
                );
                let label_width = right_x - left_x;
                cache.column_label_widths.push(right_x - left_x);
                right_x -= label_width.max(max_column_width) + COLUMN_PADDING;
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

    cache
        .splits
        .resize_with(component.splits.len(), SplitCache::new);

    for (i, (split, split_cache)) in component.splits.iter().zip(&mut cache.splits).enumerate() {
        if component.show_thin_separators && i + 1 != component.splits.len() {
            context.render_rectangle(
                separator_pos,
                [split_width, split_height],
                &Gradient::Plain(layout_state.thin_separators_color),
            );
        }

        if split.is_current_split {
            context.render_background(
                [split_width, split_height],
                &component.current_split_gradient,
            );
        } else if let Some((even, odd)) = &split_background {
            let color = if split.index % 2 == 0 { even } else { odd };
            context.render_background(split_background_bottom_right, color);
        }

        {
            if let Some(icon) = context.create_image(&split.icon) {
                context.render_image([PADDING, icon_y], [icon_size, icon_size], icon);
            }

            let mut left_x = split_width - PADDING;
            let mut right_x = left_x;

            split_cache
                .columns
                .resize_with(split.columns.len(), CachedLabel::new);

            for ((column, column_cache), column_label_width) in
                split.columns.iter().zip(&mut split_cache.columns).zip(
                    cache
                        .column_label_widths
                        .iter()
                        .cloned()
                        .chain(iter::repeat(max_column_width)),
                )
            {
                if !column.value.is_empty() {
                    left_x = context.render_numbers(
                        &column.value,
                        column_cache,
                        Layer::from_updates_frequently(column.updates_frequently),
                        [right_x, split_height + TEXT_ALIGN_BOTTOM],
                        DEFAULT_TEXT_SIZE,
                        solid(&column.visual_color),
                    );
                }
                right_x -= max_column_width.max(column_label_width) + COLUMN_PADDING;
            }

            if display_two_rows {
                left_x = split_width;
            }

            context.render_text_ellipsis(
                &split.name,
                &mut split_cache.name,
                [icon_right, TEXT_ALIGN_TOP],
                DEFAULT_TEXT_SIZE,
                text_color,
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
