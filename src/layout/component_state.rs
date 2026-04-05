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

    /// Returns `true` if the meaningful content of this state is the same as
    /// `other`'s. Only compares content-relevant values such as text, times,
    /// and structural data, ignoring cosmetic properties like colors and
    /// gradients. Returns `false` if the variants differ.
    pub fn has_same_content(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::BlankSpace(a), Self::BlankSpace(b)) => a.has_same_content(b),
            (Self::DetailedTimer(a), Self::DetailedTimer(b)) => a.has_same_content(b),
            (Self::Graph(a), Self::Graph(b)) => a.has_same_content(b),
            (Self::KeyValue(a), Self::KeyValue(b)) => a.has_same_content(b),
            (Self::Separator(a), Self::Separator(b)) => a.has_same_content(b),
            (Self::Splits(a), Self::Splits(b)) => a.has_same_content(b),
            (Self::Text(a), Self::Text(b)) => a.has_same_content(b),
            (Self::Timer(a), Self::Timer(b)) => a.has_same_content(b),
            (Self::Title(a), Self::Title(b)) => a.has_same_content(b),
            (Self::Group(a), Self::Group(b)) => a.has_same_content(b),
            (Self::Carousel(a), Self::Carousel(b)) => a.has_same_content(b),
            _ => false,
        }
    }

    /// Returns `true` if this component state indicates that it updates
    /// frequently (e.g. every frame). Such components are typically timers or
    /// live graphs whose values continuously change.
    pub fn updates_frequently(&self) -> bool {
        match self {
            Self::Timer(s) => s.updates_frequently,
            Self::KeyValue(s) => s.updates_frequently,
            Self::DetailedTimer(s) => {
                s.timer.updates_frequently || s.segment_timer.updates_frequently
            }
            Self::Graph(s) => s.updates_frequently,
            Self::Splits(s) => s
                .splits
                .iter()
                .any(|split| split.columns.iter().any(|col| col.updates_frequently)),
            Self::Group(s) => s.components.iter().any(|c| c.updates_frequently()),
            Self::Carousel(s) => s.components.iter().any(|c| c.updates_frequently()),
            _ => false,
        }
    }
}

impl ComponentState {
    /// Computes a fingerprint of the meaningful content in this state. Two
    /// states with the same fingerprint are very likely to have the same
    /// meaningful content. This is used for efficient change detection in the
    /// carousel without needing to clone or store full states.
    pub fn content_fingerprint(&self) -> u64 {
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
}
