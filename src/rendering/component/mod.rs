use crate::layout::{ComponentState, LayoutState};

use super::{
    consts::{DEFAULT_COMPONENT_HEIGHT, PSEUDO_PIXELS, SEPARATOR_THICKNESS, TWO_ROW_HEIGHT},
    resource::ResourceAllocator,
    RenderContext,
};

pub mod blank_space;
pub mod detailed_timer;
pub mod graph;
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
            _ => Self::Empty,
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
        Title title
    }
}

pub fn layout_width(layout: &LayoutState) -> f32 {
    layout.components.iter().map(width).sum()
}

pub fn layout_height(layout: &LayoutState) -> f32 {
    layout.components.iter().map(height).sum()
}

pub fn width(component: &ComponentState) -> f32 {
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
            state.splits.len() as f32 * split_width
        }
        ComponentState::Text(_) => 6.0,
        ComponentState::Timer(_) => 8.25,
        ComponentState::Title(_) => 8.0,
    }
}

pub fn height(component: &ComponentState) -> f32 {
    match component {
        ComponentState::BlankSpace(state) => state.size as f32 * PSEUDO_PIXELS,
        ComponentState::DetailedTimer(state) => {
            (state.timer.height + state.segment_timer.height) as f32 * PSEUDO_PIXELS
        }
        ComponentState::Graph(state) => state.height as f32 * PSEUDO_PIXELS,
        ComponentState::KeyValue(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                DEFAULT_COMPONENT_HEIGHT
            }
        }
        ComponentState::Separator(_) => SEPARATOR_THICKNESS,
        ComponentState::Splits(state) => {
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
        ComponentState::Text(state) => {
            if state.display_two_rows {
                TWO_ROW_HEIGHT
            } else {
                DEFAULT_COMPONENT_HEIGHT
            }
        }
        ComponentState::Timer(state) => state.height as f32 * PSEUDO_PIXELS,
        ComponentState::Title(_) => TWO_ROW_HEIGHT,
    }
}

pub(super) fn render<A: ResourceAllocator>(
    cache: &mut Cache<A::Label>,
    context: &mut RenderContext<'_, A>,
    component: &ComponentState,
    state: &LayoutState,
    dim: [f32; 2],
) {
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
            timer::render(cache.timer(), context, dim, component, state);
        }
        ComponentState::Title(component) => {
            title::render(cache.title(), context, dim, component, state)
        }
    }
}
