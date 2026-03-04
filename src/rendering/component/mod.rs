use crate::layout::{ComponentState, LayoutDirection, LayoutState};

use super::{
    RenderContext,
    consts::{DEFAULT_COMPONENT_HEIGHT, PSEUDO_PIXELS, SEPARATOR_THICKNESS, TWO_ROW_HEIGHT},
    resource::ResourceAllocator,
};

pub mod blank_space;
pub mod detailed_timer;
pub mod graph;
pub mod group;
pub mod key_value;
pub mod separator;
pub mod splits;
pub mod text;
pub mod timer;
pub mod title;

pub enum Cache<L> {
    Empty,
    DetailedTimer(detailed_timer::Cache<L>),
    KeyValue(key_value::Cache<L>),
    Splits(splits::Cache<L>),
    Text(text::Cache<L>),
    Timer(timer::Cache<L>),
    Title(title::Cache<L>),
    Group(group::Cache<L>),
}

macro_rules! accessors {
    ($($variant:ident $module:ident),*) => {
        $(
            fn $module(&mut self) -> &mut $module::Cache<L> {
                match self {
                    Self::$variant(c) => c,
                    _ => {
                        *self = Self::$variant($module::Cache::new());
                        self.$module()
                    }
                }
            }
        )*
    };
}

impl<L> Cache<L> {
    pub const fn new(component: &ComponentState) -> Self {
        match component {
            ComponentState::DetailedTimer(_) => Self::DetailedTimer(detailed_timer::Cache::new()),
            ComponentState::KeyValue(_) => Self::KeyValue(key_value::Cache::new()),
            ComponentState::Splits(_) => Self::Splits(splits::Cache::new()),
            ComponentState::Text(_) => Self::Text(text::Cache::new()),
            ComponentState::Timer(_) => Self::Timer(timer::Cache::new()),
            ComponentState::Title(_) => Self::Title(title::Cache::new()),
            ComponentState::Group(_) => Self::Group(group::Cache::new()),
            ComponentState::BlankSpace(_)
            | ComponentState::Graph(_)
            | ComponentState::Separator(_) => Self::Empty,
        }
    }

    fn make_empty(&mut self) {
        *self = Self::Empty;
    }

    accessors! {
        DetailedTimer detailed_timer,
        KeyValue key_value,
        Splits splits,
        Text text,
        Timer timer,
        Title title,
        Group group
    }
}

pub fn layout_width(layout: &LayoutState) -> f32 {
    layout
        .components
        .iter()
        .map(|c| width(c, LayoutDirection::Horizontal))
        .sum()
}

pub fn layout_height(layout: &LayoutState) -> f32 {
    layout
        .components
        .iter()
        .map(|c| height(c, LayoutDirection::Vertical))
        .sum()
}

/// Returns the width of a component given the effective layout direction.
///
/// In horizontal mode this returns the component's "natural" horizontal width
/// (used for proportional distribution). In vertical mode it returns the
/// preferred width of a single column (relevant for computing a vertical
/// group's max width).
pub fn width(component: &ComponentState, direction: LayoutDirection) -> f32 {
    match component {
        ComponentState::BlankSpace(state) => state.size as f32 * PSEUDO_PIXELS,
        ComponentState::DetailedTimer(_) => 7.0,
        ComponentState::Graph(_) => 7.0,
        ComponentState::KeyValue(_) => 6.0,
        ComponentState::Separator(_) => SEPARATOR_THICKNESS,
        ComponentState::Splits(state) => {
            let column_count = 2.0; // FIXME: Not always 2.
            let column_width = 2.75; // FIXME: Not always 2.75; difficult to calculate without a renderer.
            let split_width = 2.0 + column_count * column_width;
            match direction {
                // In horizontal mode, splits are laid out side-by-side, so
                // the total width is the number of splits times per-split width.
                LayoutDirection::Horizontal => state.splits.len() as f32 * split_width,
                // In vertical mode, splits are stacked so one split's width
                // is the component's preferred width.
                LayoutDirection::Vertical => split_width,
            }
        }
        ComponentState::Text(_) => 6.0,
        ComponentState::Timer(_) => 8.25,
        ComponentState::Title(_) => 8.0,
        ComponentState::Group(group) => match direction {
            LayoutDirection::Vertical => group
                .components
                .iter()
                .map(|c| width(c, LayoutDirection::Horizontal))
                .sum(),
            LayoutDirection::Horizontal => {
                if let Some(size) = group.size {
                    size as f32 * PSEUDO_PIXELS
                } else {
                    group
                        .components
                        .iter()
                        .map(|c| width(c, LayoutDirection::Vertical))
                        .max_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal))
                        .unwrap_or(0.0)
                }
            }
        },
    }
}

/// Returns the height of a component given the effective layout direction.
///
/// In vertical mode this returns the component's stacked height. In
/// horizontal mode components render at a fixed row height since they are
/// placed side-by-side.
pub fn height(component: &ComponentState, direction: LayoutDirection) -> f32 {
    match component {
        ComponentState::BlankSpace(state) => state.size as f32 * PSEUDO_PIXELS,
        ComponentState::DetailedTimer(state) => {
            (state.timer.height + state.segment_timer.height) as f32 * PSEUDO_PIXELS
        }
        ComponentState::Graph(state) => state.height as f32 * PSEUDO_PIXELS,
        ComponentState::KeyValue(state) => {
            // In horizontal mode the renderer forces display_two_rows.
            if state.display_two_rows || direction == LayoutDirection::Horizontal {
                TWO_ROW_HEIGHT
            } else {
                DEFAULT_COMPONENT_HEIGHT
            }
        }
        ComponentState::Separator(_) => SEPARATOR_THICKNESS,
        ComponentState::Splits(state) => match direction {
            LayoutDirection::Vertical => {
                state.splits.len() as f32
                    * if state.display_two_rows {
                        TWO_ROW_HEIGHT
                    } else {
                        DEFAULT_COMPONENT_HEIGHT
                    }
                    + if state.column_labels.is_some() {
                        DEFAULT_COMPONENT_HEIGHT
                    } else {
                        0.0
                    }
            }
            // In horizontal mode all splits are rendered side-by-side at a
            // fixed height and column labels are not shown.
            LayoutDirection::Horizontal => TWO_ROW_HEIGHT,
        },
        ComponentState::Text(state) => {
            // In horizontal mode the renderer forces display_two_rows.
            if state.display_two_rows || direction == LayoutDirection::Horizontal {
                TWO_ROW_HEIGHT
            } else {
                DEFAULT_COMPONENT_HEIGHT
            }
        }
        ComponentState::Timer(state) => state.height as f32 * PSEUDO_PIXELS,
        ComponentState::Title(_) => TWO_ROW_HEIGHT,
        ComponentState::Group(group) => match direction {
            LayoutDirection::Horizontal => group
                .components
                .iter()
                .map(|c| height(c, LayoutDirection::Vertical))
                .sum(),
            LayoutDirection::Vertical => {
                if let Some(size) = group.size {
                    size as f32 * PSEUDO_PIXELS
                } else {
                    group
                        .components
                        .iter()
                        .map(|c| height(c, LayoutDirection::Horizontal))
                        .max_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal))
                        .unwrap_or(0.0)
                }
            }
        },
    }
}

pub(super) fn render<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<A>,
    component: &ComponentState,
    state: &LayoutState,
    dim: [f32; 2],
    selected: Option<usize>,
    flat_index: &mut usize,
) {
    let is_selected = selected == Some(*flat_index);
    *flat_index += 1;

    match component {
        ComponentState::BlankSpace(state) => {
            cache.make_empty();
            blank_space::render(context, dim, state)
        }
        ComponentState::DetailedTimer(component) => {
            detailed_timer::render(cache.detailed_timer(), context, dim, component, state)
        }
        ComponentState::Graph(component) => {
            cache.make_empty();
            graph::render(context, dim, component, state)
        }
        ComponentState::KeyValue(component) => {
            key_value::render(cache.key_value(), context, dim, component, state)
        }
        ComponentState::Separator(component) => {
            cache.make_empty();
            separator::render(context, dim, component, state)
        }
        ComponentState::Splits(component) => {
            splits::render(cache.splits(), context, dim, component, state)
        }
        ComponentState::Text(component) => {
            text::render(cache.text(), context, dim, component, state)
        }
        ComponentState::Timer(component) => {
            timer::render(cache.timer(), context, dim, component);
        }
        ComponentState::Title(component) => {
            title::render(cache.title(), context, dim, component, state)
        }
        ComponentState::Group(component) => group::render_group(
            cache.group(),
            context,
            component,
            state,
            dim,
            selected,
            flat_index,
        ),
    }

    if is_selected {
        context.render_selection_outline(dim);
    }
}
