//! The translation contract for keyboard labels, independent of a language registry.
//!
//! Applications implement this for their own locale type. Closed message enums
//! make missing translations a compile error without duplicating language lists
//! or requiring this low-level crate to depend on its callers.

use core::fmt;

/// Translations needed to present key names and compose localized labels.
///
/// Formatting must preserve the supplied value and propagate writer failures.
/// The value is a key name or number; templates determine its position rather
/// than requiring callers to rearrange an already-formatted English string.
pub trait Localize: Copy {
    /// Return a localized keyboard label, without allocating.
    fn text(self, text: Text) -> &'static str;

    /// Write a localized template around its single dynamic value.
    fn format(
        self,
        template: Template,
        value: &dyn fmt::Display,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result;
}

/// The simple keyboard labels a localization provider must translate.
/// Adding a label requires all providers to supply its translation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Text {
    /// The label for ctrl.
    Control,
    /// The label for alt.
    Alt,
    /// The label for shift.
    Shift,
    /// The label for meta.
    Meta,
    /// The label for super.
    Super,
    /// The label for international backslash.
    InternationalBackslash,
    /// The label for backspace.
    Backspace,
    /// The label for caps lock.
    CapsLock,
    /// The label for context menu.
    ContextMenu,
    /// The label for enter.
    Enter,
    /// The label for space.
    Space,
    /// The label for tab.
    Tab,
    /// The label for convert.
    Convert,
    /// The label for kana mode.
    KanaMode,
    /// The label for hangul / kana.
    Lang1,
    /// The label for hanja / alphanumeric.
    Lang2,
    /// The label for katakana.
    Lang3,
    /// The label for hiragana.
    Lang4,
    /// The label for half-width / full-width.
    Lang5,
    /// The label for non-convert.
    NonConvert,
    /// The label for delete.
    Delete,
    /// The label for end.
    End,
    /// The label for help.
    Help,
    /// The label for home.
    Home,
    /// The label for insert.
    Insert,
    /// The label for page down.
    PageDown,
    /// The label for page up.
    PageUp,
    /// The label for num lock.
    NumLock,
    /// The label for clear.
    Clear,
    /// The label for clear entry.
    ClearEntry,
    /// The label for decimal.
    Decimal,
    /// The label for esc.
    Escape,
    /// The label for fn lock.
    FnLock,
    /// The label for print screen.
    PrintScreen,
    /// The label for scroll lock.
    ScrollLock,
    /// The label for pause / break.
    Pause,
    /// The label for browser back.
    BrowserBack,
    /// The label for browser favorites.
    BrowserFavorites,
    /// The label for browser forward.
    BrowserForward,
    /// The label for browser home.
    BrowserHome,
    /// The label for browser refresh.
    BrowserRefresh,
    /// The label for browser search.
    BrowserSearch,
    /// The label for browser stop.
    BrowserStop,
    /// The label for eject.
    Eject,
    /// The label for open mail.
    LaunchMail,
    /// The label for play / pause.
    MediaPlayPause,
    /// The label for select media.
    MediaSelect,
    /// The label for stop media.
    MediaStop,
    /// The label for next track.
    MediaTrackNext,
    /// The label for previous track.
    MediaTrackPrevious,
    /// The label for power.
    Power,
    /// The label for sleep.
    Sleep,
    /// The label for volume down.
    AudioVolumeDown,
    /// The label for mute.
    AudioVolumeMute,
    /// The label for volume up.
    AudioVolumeUp,
    /// The label for wake up.
    WakeUp,
    /// The label for again.
    Again,
    /// The label for copy.
    Copy,
    /// The label for cut.
    Cut,
    /// The label for find.
    Find,
    /// The label for open.
    Open,
    /// The label for paste.
    Paste,
    /// The label for properties.
    Properties,
    /// The label for select.
    Select,
    /// The label for undo.
    Undo,
    /// The label for brightness down.
    BrightnessDown,
    /// The label for brightness up.
    BrightnessUp,
    /// The label for switch internal / external display.
    DisplayToggle,
    /// The label for select keyboard layout.
    KeyboardLayout,
    /// The label for open assistant.
    LaunchAssistant,
    /// The label for open control panel.
    LaunchControlPanel,
    /// The label for start screen saver.
    LaunchScreenSaver,
    /// The label for forward mail.
    MailForward,
    /// The label for reply to mail.
    MailReply,
    /// The label for send mail.
    MailSend,
    /// The label for fast forward.
    MediaFastForward,
    /// The label for pause media.
    MediaPause,
    /// The label for play media.
    MediaPlay,
    /// The label for record media.
    MediaRecord,
    /// The label for rewind.
    MediaRewind,
    /// The label for mute microphone.
    MicrophoneMute,
    /// The label for toggle privacy screen.
    PrivacyScreen,
    /// The label for select task.
    SelectTask,
    /// The label for show all windows.
    ShowAllWindows,
    /// The label for toggle zoom.
    ZoomToggle,
}

/// Composite labels whose dynamic value must retain the locale's word order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Template {
    /// A numeric-keypad key. The value is a digit, operator or localized key name.
    NumpadKey,
    /// A gamepad button. The value is its zero-based technical button number.
    GamepadButton,
    /// A modifier on the left side of the keyboard. The value is its name.
    LeftKey,
    /// A modifier on the right side of the keyboard. The value is its name.
    RightKey,
    /// A numbered application-launch key. The value is its application number.
    LaunchApp,
}
