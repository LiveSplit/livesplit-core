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
        solid, RenderContext, FillShader
    },
    settings::{Gradient, ListGradient},
};

pub struct Cache<L> {
    splits: Vec<SplitCache<L>>,
    column_labels: Vec<CachedLabel<L>>,
    column_width_labels: Vec<(f32, CachedLabel<L>)>,
    longest_column_values: Vec<ShortLivedStr>,
}

#[derive(Copy, Clone)]
struct ShortLivedStr {
    str: *const str,
    char_count: usize,
}

impl ShortLivedStr {
    // We use this as the smallest possible "space" we use for each column. This
    // prevents the columns from changing size too much. We could bump this up
    // to include hours, but that's not ideal for delta based columns, which are
    // usually smaller.
    const MIN: Self = Self {
        str: "88:88",
        char_count: 5,
    };

    fn new(s: &str) -> Self {
        Self {
            str: s,
            char_count: s.chars().count(),
        }
    }

    /// # Safety
    /// Only call this function for a string that's still valid.
    const unsafe fn get(&self) -> &str {
        &*self.str
    }
}

// SAFETY: These strings are never actually kept across calls to render.
unsafe impl Send for ShortLivedStr {}
// SAFETY: These strings are never actually kept across calls to render.
unsafe impl Sync for ShortLivedStr {}

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
            column_width_labels: Vec::new(),
            longest_column_values: Vec::new(),
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
    // We measure the longest value in each column in terms of unicode scalar
    // values. This is not perfect, but gives a decent enough approximation for
    // now. Long term we probably want to shape all the texts first and then lay
    // them out.

    cache.longest_column_values.clear();

    let shadow_offset = [0.05, 0.05];
    let shadow_color = FillShader::SolidColor([0.0, 0.0, 0.0, 0.5]);
    for split in &component.splits {
        if split.columns.len() > cache.longest_column_values.len() {
            cache
                .longest_column_values
                .resize(split.columns.len(), ShortLivedStr::MIN);
        }
        for (column, longest_column_value) in
            split.columns.iter().zip(&mut cache.longest_column_values)
        {
            let column_value = ShortLivedStr::new(column.value.as_str());
            if column_value.char_count > longest_column_value.char_count {
                *longest_column_value = column_value;
            }
        }
    }

    cache.column_width_labels.resize_with(
        cache.longest_column_values.len().max(
            component
                .column_labels
                .as_ref()
                .map(|labels| labels.len())
                .unwrap_or_default(),
        ),
        || (0.0, CachedLabel::new()),
    );

    for (longest_column_value, (column_width, column_width_label)) in cache
        .longest_column_values
        .iter()
        .zip(&mut cache.column_width_labels)
    {
        // SAFETY: The longest_column_values vector is cleared on every render
        // call. We only store references to the column values in the vector and
        // a 'static default string. All of these are valid for the entire
        // duration of the render call.
        unsafe {
            *column_width = context.measure_numbers(
                longest_column_value.get(),
                column_width_label,
                DEFAULT_TEXT_SIZE,
            );
        }
    }

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
    let text_color = solid(&layout_state.text_color);

    if let Some(column_labels) = &component.column_labels {
        if layout_state.direction == LayoutDirection::Vertical {
            cache
                .column_labels
                .resize_with(column_labels.len(), CachedLabel::new);

            let mut right_x = width - PADDING;
            for ((label, column_cache), (max_width, _)) in column_labels
                .iter()
                .zip(&mut cache.column_labels)
                .zip(&mut cache.column_width_labels)
            {
                let left_x = context.render_text_right_align(
                    label,
                    column_cache,
                    Layer::Bottom,
                    [right_x, TEXT_ALIGN_TOP],
                    DEFAULT_TEXT_SIZE,
                    text_color,
                    shadow_offset,
                    shadow_color,
                    layout_state
                );
                let label_width = right_x - left_x;
                if label_width > *max_width {
                    *max_width = label_width;
                }
                right_x -= *max_width + PADDING;
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

            for ((column, column_cache), (max_width, _)) in split
                .columns
                .iter()
                .zip(&mut split_cache.columns)
                .zip(&cache.column_width_labels)
            {
                if !column.value.is_empty() {
                    left_x = context.render_numbers(
                        &column.value,
                        column_cache,
                        Layer::from_updates_frequently(column.updates_frequently),
                        [right_x, split_height + TEXT_ALIGN_BOTTOM],
                        DEFAULT_TEXT_SIZE,
                        solid(&column.visual_color),
                        shadow_offset,
                        shadow_color,
                        layout_state
                    );
                }
                right_x -= max_width + PADDING;
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
                shadow_offset,
                shadow_color,
                left_x - PADDING,
                layout_state
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
