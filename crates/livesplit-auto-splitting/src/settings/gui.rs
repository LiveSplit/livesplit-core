use std::sync::Arc;

/// A setting widget that is meant to be shown to and modified by the user.
#[non_exhaustive]
#[derive(Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
    FileSelect {
        /// The filters that are used to filter the files that can be selected.
        filters: Arc<Vec<FileFilter>>,
    },
}

/// A filter for a file selection setting.
#[derive(Clone, Debug, PartialEq)]
pub enum FileFilter {
    /// A filter that matches on the name of the file.
    Name {
        /// The description is what's shown to the user for the specific filter.
        description: Option<Arc<str>>,
        /// The pattern is a [glob
        /// pattern](https://en.wikipedia.org/wiki/Glob_(programming)) that is
        /// used to filter the files. The pattern generally only supports `*`
        /// wildcards, not `?` or brackets. This may however differ between
        /// frontends. Additionally `;` can't be used in Windows's native file
        /// dialog if it's part of the pattern. Multiple patterns may be
        /// specified by separating them with ASCII space characters. There are
        /// operating systems where glob patterns are not supported. A best
        /// effort lookup of the fitting MIME type may be used by a frontend on
        /// those operating systems instead. The
        /// [`mime_guess`](https://docs.rs/mime_guess) crate offers such a
        /// lookup.
        pattern: Arc<str>,
    },
    /// A filter that matches on the MIME type of the file. Most operating
    /// systems do not support MIME types, but the frontends are encouraged to
    /// look up the file extensions that are associated with the MIME type and
    /// use those as a filter in those cases. You may also use wildcards as part
    /// of the MIME types such as `image/*`. The support likely also varies
    /// between frontends however. The
    /// [`mime_guess`](https://docs.rs/mime_guess) crate offers such a lookup.
    MimeType(Arc<str>),
}

/// An option for a choice setting.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ChoiceOption {
    /// The unique identifier of the option. This is not meant to be shown to
    /// the user and is only used to keep track of the option. This key is used
    /// to store and retrieve the value of the option from the main settings
    /// [`Map`](super::Map).
    pub key: Arc<str>,
    /// The name of the option that is shown to the user.
    pub description: Arc<str>,
}
