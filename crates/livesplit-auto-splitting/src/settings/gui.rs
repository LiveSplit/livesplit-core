use std::sync::Arc;

/// A setting widget that is meant to be shown to and modified by the user.
#[non_exhaustive]
#[derive(Clone)]
pub struct Widget {
    /// A unique identifier for this setting. This is not meant to be shown to
    /// the user and is only used to keep track of the setting. This key is used
    /// to store and retrieve the value of the setting from the main settings
    /// [`Map`](super::Map).
    pub key: Arc<str>,
    /// The name of the setting that is shown to the user.
    pub description: Arc<str>,
    /// An optional tooltip that is shown to the user when hovering over the
    /// widget.
    pub tooltip: Option<Arc<str>>,
    /// The type of widget and additional information about it.
    pub kind: WidgetKind,
}

/// The type of a [`Widget`] and additional information about it.
#[derive(Clone)]
pub enum WidgetKind {
    /// A title that is shown to the user. It doesn't by itself store a value
    /// and is instead used to group settings together.
    Title {
        /// The heading level of the title. This is used to determine the size
        /// of the title and which other settings are grouped together with it.
        /// The top level titles use a heading level of 0.
        heading_level: u32,
    },
    /// A boolean setting. This could be shown as a checkbox or a toggle.
    Bool {
        /// The default value of the setting, if it's not available in the
        /// settings [`Map`](super::Map) yet.
        default_value: bool,
    },
    /// A choice setting. This could be shown as a dropdown or radio buttons.
    Choice {
        /// The default value of the setting, if it's not available in the
        /// settings [`Map`](super::Map) yet.
        default_option_key: Arc<str>,
        /// The available options for the setting.
        options: Arc<Vec<ChoiceOption>>,
    },
    /// A file selection. This could be a button that opens a File Dialog.
    FileSelection {
        /// A filter on which files are selectable,
        /// for example `"*.txt"` for text files.
        filter: Arc<str>,
    }
}

/// An option for a choice setting.
#[derive(Clone)]
pub struct ChoiceOption {
    /// The unique identifier of the option. This is not meant to be shown to
    /// the user and is only used to keep track of the option. This key is used
    /// to store and retrieve the value of the option from the main settings
    /// [`Map`](super::Map).
    pub key: Arc<str>,
    /// The name of the option that is shown to the user.
    pub description: Arc<str>,
}
