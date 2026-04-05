use core::{
    hash::{Hash, Hasher},
    mem,
};

use serde_derive::{Deserialize, Serialize};

use crate::{
    component::{
        blank_space, carousel, detailed_timer, graph, group, key_value, separator, splits, text,
        timer, title,
    },
    platform::prelude::*,
    util::FxHasher,
};

/// The state object for one of the components available.
#[derive(Serialize, Deserialize)]
pub enum ComponentState {
    /// The state object for the Blank Space Component.
    BlankSpace(blank_space::State),
    /// The state object for the Detailed Timer Component.
    DetailedTimer(Box<detailed_timer::State>),
    /// The state object for the Graph Component.
    Graph(graph::State),
    /// The state object for a key value based component.
    KeyValue(key_value::State),
    /// The state object for the Separator Component.
    Separator(separator::State),
    /// The state object for the Splits Component.
    Splits(splits::State),
    /// The state object for the Text Component.
    Text(text::State),
    /// The state object for the Timer Component.
    Timer(timer::State),
    /// The state object for the Title Component.
    Title(title::State),
    /// The state object for a Component Group.
    Group(group::State),
    /// The state object for a Carousel Component.
    Carousel(carousel::State),
}

impl ComponentState {
    /// Returns the child states if this is a container component.
    pub fn children(&self) -> Option<&[ComponentState]> {
        match self {
            Self::Group(state) => Some(&state.components),
            Self::Carousel(state) => Some(&state.components),
            _ => None,
        }
    }

    /// Returns the child states if this is a container component.
    pub const fn children_mut(&mut self) -> Option<&mut Vec<ComponentState>> {
        match self {
            Self::Group(state) => Some(&mut state.components),
            Self::Carousel(state) => Some(&mut state.components),
            _ => None,
        }
    }

    /// Computes a fingerprint of the meaningful content in this state. Two
    /// states with the same fingerprint are very likely to have the same
    /// meaningful content. This is used for efficient change detection in the
    /// carousel without needing to clone or store full states.
    pub(crate) fn content_fingerprint(&self) -> u64 {
        let mut h = FxHasher::new();
        mem::discriminant(self).hash(&mut h);
        match self {
            Self::BlankSpace(state) => state.content_fingerprint(&mut h),
            Self::DetailedTimer(state) => state.content_fingerprint(&mut h),
            Self::Graph(state) => state.content_fingerprint(&mut h),
            Self::KeyValue(state) => state.content_fingerprint(&mut h),
            Self::Separator(state) => state.content_fingerprint(&mut h),
            Self::Splits(state) => state.content_fingerprint(&mut h),
            Self::Text(state) => state.content_fingerprint(&mut h),
            Self::Timer(state) => state.content_fingerprint(&mut h),
            Self::Title(state) => state.content_fingerprint(&mut h),
            Self::Group(state) => state.content_fingerprint(&mut h),
            Self::Carousel(state) => state.content_fingerprint(&mut h),
        }
        h.finish()
    }

    /// Returns `true` if this component state indicates that it updates
    /// frequently (e.g. every frame). Such components are typically timers or
    /// live graphs whose values continuously change.
    pub(crate) fn updates_frequently(&self) -> bool {
        match self {
            Self::BlankSpace(s) => s.updates_frequently(),
            Self::DetailedTimer(s) => s.updates_frequently(),
            Self::Graph(s) => s.updates_frequently(),
            Self::KeyValue(s) => s.updates_frequently(),
            Self::Separator(s) => s.updates_frequently(),
            Self::Splits(s) => s.updates_frequently(),
            Self::Text(s) => s.updates_frequently(),
            Self::Timer(s) => s.updates_frequently(),
            Self::Title(s) => s.updates_frequently(),
            Self::Group(s) => s.updates_frequently(),
            Self::Carousel(s) => s.updates_frequently(),
        }
    }

    /// Returns the total number of nodes in this state's subtree, including
    /// the state itself.
    pub(crate) fn subtree_size(&self) -> usize {
        self.children().map_or(1, |children| {
            1 + children.iter().map(Self::subtree_size).sum::<usize>()
        })
    }
}
