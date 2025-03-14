use crate::{
    component::key_value::State,
    layout::{LayoutDirection, LayoutState},
    rendering::{
        font::{AbbreviatedLabel, CachedLabel},
        resource::ResourceAllocator,
        RenderContext, FillShader
    },
};

pub struct Cache<L> {
    key: AbbreviatedLabel<L>,
    value: CachedLabel<L>,
}

impl<L> Cache<L> {
    pub const fn new() -> Self {
        Self {
            key: AbbreviatedLabel::new(),
            value: CachedLabel::new(),
        }
    }
}

pub(in crate::rendering) fn render<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<'_, A>,
    dim: [f32; 2],
    component: &State,
    layout_state: &LayoutState,
) {
    context.render_background(dim, &component.background);

    let shadow_offset = [0.05, 0.05];
    let shadow_color = FillShader::SolidColor([0.0, 0.0, 0.0, 0.5]);
    context.render_key_value_component(
        &component.key,
        &component.key_abbreviations,
        &mut cache.key,
        &component.value,
        &mut cache.value,
        component.updates_frequently,
        dim,
        shadow_offset,
        shadow_color,
        shadow_color,
        component.key_color.unwrap_or(layout_state.text_color),
        component.value_color.unwrap_or(layout_state.text_color),
        component.display_two_rows || layout_state.direction == LayoutDirection::Horizontal,
        layout_state
    );
}
