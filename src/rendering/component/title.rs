use core::fmt::Write;

use crate::{
    component::title::State,
    layout::LayoutState,
    platform::prelude::*,
    rendering::{
        consts::{
            vertical_padding, BOTH_PADDINGS, DEFAULT_TEXT_SIZE, PADDING, TEXT_ALIGN_BOTTOM,
            TEXT_ALIGN_CENTER, TEXT_ALIGN_TOP,
        },
        font::{AbbreviatedLabel, CachedLabel},
        resource::ResourceAllocator,
        solid, Layer, RenderContext, FillShader
    },
};

pub struct Cache<L> {
    line1: AbbreviatedLabel<L>,
    line2: AbbreviatedLabel<L>,
    attempts: CachedLabel<L>,
    attempts_buffer: String,
}

impl<L> Cache<L> {
    pub const fn new() -> Self {
        Self {
            line1: AbbreviatedLabel::new(),
            line2: AbbreviatedLabel::new(),
            attempts: CachedLabel::new(),
            attempts_buffer: String::new(),
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
    
    let text_color = component.text_color.unwrap_or(layout_state.text_color);
    let text_color = solid(&text_color);

    let left_bound = if let Some(icon) = context.create_image(&component.icon) {
        let vertical_padding = vertical_padding(height);
        let icon_size = height - 2.0 * vertical_padding;
        context.render_image([PADDING, vertical_padding], [icon_size, icon_size], icon);
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
        (Some(a), Some(b)) => {
            cache.attempts_buffer.clear();
            let _ = write!(cache.attempts_buffer, "{a}/{b}");
            cache.attempts_buffer.as_str()
        }
        (Some(a), _) | (_, Some(a)) => {
            cache.attempts_buffer.clear();
            let _ = write!(cache.attempts_buffer, "{a}");
            cache.attempts_buffer.as_str()
        }
        _ => "",
    };

    let line2_end_x = context.render_numbers(
        attempts,
        &mut cache.attempts,
        Layer::Bottom,
        [width - PADDING, height + TEXT_ALIGN_BOTTOM],
        DEFAULT_TEXT_SIZE,
        text_color,
        shadow_offset,
        shadow_color,
        layout_state
    ) - PADDING;

    let (line1_y, line1_end_x) = if !component.line2.is_empty() {
        context.render_abbreviated_text_align(
            component.line2.iter().map(|a| &**a),
            &mut cache.line2,
            left_bound,
            line2_end_x,
            [line_x, height + TEXT_ALIGN_BOTTOM],
            DEFAULT_TEXT_SIZE,
            component.is_centered,
            shadow_offset,
            shadow_color,
            text_color,
            layout_state
        );
        (TEXT_ALIGN_TOP, width - PADDING)
    } else {
        (0.5 * height + TEXT_ALIGN_CENTER, line2_end_x)
    };

    context.render_abbreviated_text_align(
        component.line1.iter().map(|a| &**a),
        &mut cache.line1,
        left_bound,
        line1_end_x,
        [line_x, line1_y],
        DEFAULT_TEXT_SIZE,
        component.is_centered,
        shadow_offset,
        shadow_color,
        text_color,
        layout_state
    );
}
