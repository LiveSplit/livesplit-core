use crate::component::{
    blank_space, detailed_timer, graph, key_value, separator, splits, text, timer, title,
};
use crate::platform::prelude::*;
use serde::{Deserialize, Serialize};

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
}
