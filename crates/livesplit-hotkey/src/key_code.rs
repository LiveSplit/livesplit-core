use alloc::borrow::Cow;
use core::str::FromStr;

// This is based on the web KeyboardEvent code Values specification and the
// individual mappings are based on the following sources:
//
// MDN, but it turns out to be wrong in many ways:
// https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code/code_values
//
// Chromium's sources:
// https://github.com/chromium/chromium/blob/5af3e41ce69e2e18b899589b46540e4360527733/ui/events/keycodes/dom/dom_code_data.inc
//
// Firefox's sources:
// https://github.com/mozilla/gecko-dev/blob/25002b534963ad95ff0c1a3dd0f906ba023ddc8e/widget/NativeKeyToDOMCodeName.h
//
// Safari's sources: Windows:
// https://github.com/WebKit/WebKit/blob/8afe31a018b11741abdf9b4d5bb973d7c1d9ff05/Source/WebCore/platform/win/WindowsKeyNames.cpp
// macOS:
// https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/mac/PlatformEventFactoryMac.mm
// Linux GTK:
// https://github.com/WebKit/WebKit/blob/8afe31a018b11741abdf9b4d5bb973d7c1d9ff05/Source/WebCore/platform/gtk/PlatformKeyboardEventGtk.cpp
// WPE:
// https://github.com/WebKit/WebKit/blob/8afe31a018b11741abdf9b4d5bb973d7c1d9ff05/Source/WebCore/platform/libwpe/PlatformKeyboardEventLibWPE.cpp

/// A key code represents a physical key on a keyboard and is independent of the
/// keyboard layout. The values are based on the [`UI Events KeyboardEvent code
/// Values`](https://www.w3.org/TR/uievents-code/) specification. There are some
/// additional values for Gamepad support and some browser specific values.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum KeyCode {
    /// `Backtick` and `~` on a US keyboard. This is the `半角/全角/漢字`
    /// (`hankaku/zenkaku/kanji`) key on Japanese keyboards
    ///
    /// USB HID:
    ///  - `Keyboard Grave Accent and Tilde` `Keyboard Page 0x35`
    Backquote,
    /// Used for both the US `\|` (on the 101-key layout) and also for the key
    /// located between the `"` and `Enter` keys on row C of the 102-, 104- and
    /// 106-key layouts. Labelled `#~` on a UK (102) keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard \ and |` `Keyboard Page 0x31`
    ///  - `Keyboard Non-US # and ~` `Keyboard Page 0x32`
    Backslash,
    /// `[{` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard [ and {` `Keyboard Page 0x2f`
    BracketLeft,
    /// `]}` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard ] and }` `Keyboard Page 0x30`
    BracketRight,
    /// `,<` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard , and <` `Keyboard Page 0x36`
    Comma,
    /// `0)` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard 0 and )` `Keyboard Page 0x27`
    Digit0,
    /// `1!` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard 1 and !` `Keyboard Page 0x1e`
    Digit1,
    /// `2@` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard 2 and @` `Keyboard Page 0x1f`
    Digit2,
    /// `3#` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard 3 and #` `Keyboard Page 0x20`
    Digit3,
    /// `4$` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard 4 and $` `Keyboard Page 0x21`
    Digit4,
    /// `5%` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard 5 and %` `Keyboard Page 0x22`
    Digit5,
    /// `6^` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard 6 and ^` `Keyboard Page 0x23`
    Digit6,
    /// `7&` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard 7 and &` `Keyboard Page 0x24`
    Digit7,
    /// `8*` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard 8 and *` `Keyboard Page 0x25`
    Digit8,
    /// `9(` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard 9 and (` `Keyboard Page 0x26`
    Digit9,
    /// `=+` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard = and +` `Keyboard Page 0x2e`
    Equal,
    /// Located between the left `Shift` and `Z` keys. Labelled `\|` on a UK
    /// keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard Non-US \ and |` `Keyboard Page 0x64`
    IntlBackslash,
    /// Located between the `/` and right `Shift` keys. Labelled `\ろ` (`ro`) on
    /// a Japanese keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard International1` `Keyboard Page 0x87`
    IntlRo,
    /// Located between the `=` and `Backspace` keys. Labelled `¥` (`yen`) on a
    /// Japanese keyboard. `\/` on a Russian keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard International3` `Keyboard Page 0x89`
    IntlYen,
    /// `a` on a US keyboard. Labelled `q` on an AZERTY (e.g., French) keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard a and A` `Keyboard Page 0x4`
    KeyA,
    /// `b` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard b and B` `Keyboard Page 0x5`
    KeyB,
    /// `c` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard c and C` `Keyboard Page 0x6`
    KeyC,
    /// `d` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard d and D` `Keyboard Page 0x7`
    KeyD,
    /// `e` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard e and E` `Keyboard Page 0x8`
    KeyE,
    /// `f` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard f and F` `Keyboard Page 0x9`
    KeyF,
    /// `g` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard g and G` `Keyboard Page 0xa`
    KeyG,
    /// `h` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard h and H` `Keyboard Page 0xb`
    KeyH,
    /// `i` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard i and I` `Keyboard Page 0xc`
    KeyI,
    /// `j` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard j and J` `Keyboard Page 0xd`
    KeyJ,
    /// `k` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard k and K` `Keyboard Page 0xe`
    KeyK,
    /// `l` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard l and L` `Keyboard Page 0xf`
    KeyL,
    /// `m` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard m and M` `Keyboard Page 0x10`
    KeyM,
    /// `n` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard n and N` `Keyboard Page 0x11`
    KeyN,
    /// `o` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard o and O` `Keyboard Page 0x12`
    KeyO,
    /// `p` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard p and P` `Keyboard Page 0x13`
    KeyP,
    /// `q` on a US keyboard. Labelled `a` on an AZERTY (e.g., French) keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard q and Q` `Keyboard Page 0x14`
    KeyQ,
    /// `r` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard r and R` `Keyboard Page 0x15`
    KeyR,
    /// `s` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard s and S` `Keyboard Page 0x16`
    KeyS,
    /// `t` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard t and T` `Keyboard Page 0x17`
    KeyT,
    /// `u` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard u and U` `Keyboard Page 0x18`
    KeyU,
    /// `v` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard v and V` `Keyboard Page 0x19`
    KeyV,
    /// `w` on a US keyboard. Labelled `z` on an AZERTY (e.g., French) keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard w and W` `Keyboard Page 0x1a`
    KeyW,
    /// `x` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard x and X` `Keyboard Page 0x1b`
    KeyX,
    /// `y` on a US keyboard. Labelled `z` on a QWERTZ (e.g., German) keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard y and Y` `Keyboard Page 0x1c`
    KeyY,
    /// `z` on a US keyboard. Labelled `w` on an AZERTY (e.g., French) keyboard,
    /// and `y` on a QWERTZ (e.g., German) keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard z and Z` `Keyboard Page 0x1d`
    KeyZ,
    /// `-_` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard - and (underscore)` `Keyboard Page 0x2d`
    Minus,
    /// `.>` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard . and >` `Keyboard Page 0x37`
    Period,
    /// `'"` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard ‘ and “` `Keyboard Page 0x34`
    Quote,
    /// `;:` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard ; and :` `Keyboard Page 0x33`
    Semicolon,
    /// `/?` on a US keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard / and ?` `Keyboard Page 0x38`
    Slash,

    /// `Alt`, `Option` or
    /// `⌥`.
    ///
    /// USB HID:
    ///  - `Keyboard LeftAlt` `Keyboard Page 0xe2`
    AltLeft,
    /// `Alt`, `Option` or `⌥`. This is labelled `AltGr` key on many keyboard
    /// layouts.
    ///
    /// USB HID:
    ///  - `Keyboard RightAlt` `Keyboard Page 0xe6`
    AltRight,
    /// `Backspace` or `⌫`. Labelled `Delete` on Apple keyboards.
    ///
    /// USB HID:
    ///  - `Keyboard DELETE (Backspace)` `Keyboard Page 0x2a`
    Backspace,
    /// `CapsLock` or `⇪`
    ///
    /// USB HID:
    ///  - `Keyboard Caps Lock` `Keyboard Page 0x39`
    CapsLock,
    /// The application context menu key, which is typically found between the
    /// right `Meta` key and the right `Control` key.
    ///
    /// USB HID:
    ///  - `Keyboard Application` `Keyboard Page 0x65`
    ContextMenu,
    /// `Control` or `⌃`
    ///
    /// USB HID:
    ///  - `Keyboard LeftControl` `Keyboard Page 0xe0`
    ControlLeft,
    /// `Control` or `⌃`
    ///
    /// USB HID:
    ///  - `Keyboard RightControl` `Keyboard Page 0xe4`
    ControlRight,
    /// `Enter` or `↵`. Labelled `Return` on Apple keyboards.
    ///
    /// USB HID:
    ///  - `Keyboard Return (ENTER)` `Keyboard Page 0x28`
    ///  - `Enter Channel` `Consumer Page 0x84`
    Enter,
    /// The Windows, `⌘`, `Command` or other OS symbol key.
    ///
    /// USB HID:
    ///  - `Keyboard Left GUI` `Keyboard Page 0xe3`
    MetaLeft,
    /// The Windows, `⌘`, `Command` or other OS symbol key.
    ///
    /// USB HID:
    ///  - `Keyboard Right GUI` `Keyboard Page 0xe7`
    MetaRight,
    /// `Shift` or `⇧`
    ///
    /// USB HID:
    ///  - `Keyboard LeftShift` `Keyboard Page 0xe1`
    ShiftLeft,
    /// `Shift` or `⇧`
    ///
    /// USB HID:
    ///  - `Keyboard RightShift` `Keyboard Page 0xe5`
    ShiftRight,
    /// ` ` (space)
    ///
    /// USB HID:
    ///  - `Keyboard Spacebar` `Keyboard Page 0x2c`
    Space,
    /// `Tab` or `⇥`
    ///
    /// USB HID:
    ///  - `Keyboard Tab` `Keyboard Page 0x2b`
    Tab,

    /// Japanese: `変換` (`henkan`)
    ///
    /// USB HID:
    ///  - `Keyboard International4` `Keyboard Page 0x8a`
    Convert,
    /// Japanese: `カタカナ/ひらがな/ローマ字` (`katakana/hiragana/romaji`)
    ///
    /// USB HID:
    ///  - `Keyboard International2` `Keyboard Page 0x88`
    KanaMode,
    /// Korean: HangulMode `한/영` (`han/yeong`)
    ///
    /// Japanese (Mac keyboard): `かな` (`kana`)
    ///
    /// USB HID:
    ///  - `Keyboard LANG1` `Keyboard Page 0x90`
    Lang1,
    /// Korean: Hanja `한자` (`hanja`)
    ///
    /// Japanese (Mac keyboard): `英数` (`eisu`)
    ///
    /// USB HID:
    ///  - `Keyboard LANG2` `Keyboard Page 0x91`
    Lang2,
    /// Japanese (word-processing keyboard): Katakana
    ///
    /// USB HID:
    ///  - `Keyboard LANG3` `Keyboard Page 0x92`
    Lang3,
    /// Japanese (word-processing keyboard): Hiragana
    ///
    /// USB HID:
    ///  - `Keyboard LANG4` `Keyboard Page 0x93`
    Lang4,
    /// Japanese (word-processing keyboard): Zenkaku/Hankaku
    ///
    /// USB HID:
    ///  - `Keyboard LANG5` `Keyboard Page 0x94`
    Lang5,
    /// Japanese: `無変換` (`muhenkan`)
    ///
    /// USB HID:
    ///  - `Keyboard International5` `Keyboard Page 0x8b`
    NonConvert,

    /// `⌦`. The forward delete key. Note that on Apple keyboards, the key
    /// labelled `Delete` on the main part of the keyboard should be encoded as
    /// `"Backspace"`.
    ///
    /// USB HID:
    ///  - `Keyboard Delete Forward` `Keyboard Page 0x4c`
    ///  - `Keyboard Clear` `Keyboard Page 0x9c`
    ///  - `Keypad Clear` `Keyboard Page 0xd8`
    ///  - `AC Delete` `Consumer Page 0x26a`
    Delete,
    /// `Page Down`, `End` or `↘`
    ///
    /// USB HID:
    ///  - `Keyboard End` `Keyboard Page 0x4d`
    End,
    /// `Help`. Not present on standard PC keyboards.
    ///
    /// USB HID:
    ///  - `Keyboard Help` `Keyboard Page 0x75`
    ///  - `Help` `Consumer Page 0x95`
    ///  - `AL Integrated Help Center` `Consumer Page 0x1a6`
    Help,
    /// `Home` or `↖`
    ///
    /// USB HID:
    ///  - `Keyboard Home` `Keyboard Page 0x4a`
    Home,
    /// `Insert` or `Ins`. Not present on Apple keyboards.
    ///
    /// USB HID:
    ///  - `Keyboard Insert` `Keyboard Page 0x49`
    ///  - `AC Insert Mode` `Consumer Page 0x269`
    Insert,
    /// `Page Down`, `PgDn` or `⇟`
    ///
    /// USB HID:
    ///  - `Keyboard PageDown` `Keyboard Page 0x4e`
    PageDown,
    /// `Page Up`, `PgUp` or
    /// `⇞`
    ///
    /// USB HID:
    ///  - `Keyboard PageUp` `Keyboard Page 0x4b`
    PageUp,

    /// `↓`
    ///
    /// USB HID:
    ///  - `Keyboard DownArrow` `Keyboard Page 0x51`
    ///  - `Menu Down` `Consumer Page 0x43`
    ArrowDown,
    /// `←`
    ///
    /// USB HID:
    ///  - `Keyboard LeftArrow` `Keyboard Page 0x50`
    ///  - `Menu Left` `Consumer Page 0x44`
    ArrowLeft,
    /// `→`
    ///
    /// USB HID:
    ///  - `Keyboard RightArrow` `Keyboard Page 0x4f`
    ///  - `Menu Right` `Consumer Page 0x45`
    ArrowRight,
    /// `↑`
    ///
    /// USB HID:
    ///  - `Keyboard UpArrow` `Keyboard Page 0x52`
    ///  - `Menu Up` `Consumer Page 0x42`
    ArrowUp,

    /// On the Mac, the `"NumLock"` code should be used for the numpad `Clear`
    /// key.
    ///
    /// USB HID:
    ///  - `Keypad Num Lock and Clear` `Keyboard Page 0x53`
    NumLock,
    /// `0 Ins` on a keyboard
    ///
    /// `0` on a phone or remote control
    ///
    /// USB HID:
    ///  - `Keypad 0 and Insert` `Keyboard Page 0x62`
    ///  - `Phone Key 0` `Telephony Device Page 0xb0`
    Numpad0,
    /// `1 End` on a keyboard
    ///
    /// `1` or `1 QZ` on a phone or remote control
    ///
    /// USB HID:
    ///  - `Keypad 1 and End` `Keyboard Page 0x59`
    ///  - `Phone Key 1` `Telephony Device Page 0xb1`
    Numpad1,
    /// `2 ↓` on a keyboard
    ///
    /// `2 ABC` on a phone or remote control
    ///
    /// USB HID:
    ///  - `Keypad 2 and Down Arrow` `Keyboard Page 0x5a`
    ///  - `Phone Key 2` `Telephony Device Page 0xb2`
    Numpad2,
    /// `3 PgDn` on a keyboard
    ///
    /// `3 DEF` on a phone or remote control
    ///
    /// USB HID:
    ///  - `Keypad 3 and PageDn` `Keyboard Page 0x5b`
    ///  - `Phone Key 3` `Telephony Device Page 0xb3`
    Numpad3,
    /// `4 ←` on a keyboard
    ///
    /// `4 GHI` on a phone or remote control
    ///
    /// USB HID:
    ///  - `Keypad 4 and Left Arrow` `Keyboard Page 0x5c`
    ///  - `Phone Key 4` `Telephony Device Page 0xb4`
    Numpad4,
    /// `5` on a keyboard
    ///
    /// `5 JKL` on a phone or remote control
    ///
    /// USB HID:
    ///  - `Keypad 5` `Keyboard Page 0x5d`
    ///  - `Phone Key 5` `Telephony Device Page 0xb5`
    Numpad5,
    /// `6 →` on a keyboard
    ///
    /// `6 MNO` on a phone or remote control
    ///
    /// USB HID:
    ///  - `Keypad 6 and Right Arrow` `Keyboard Page 0x5e`
    ///  - `Phone Key 6` `Telephony Device Page 0xb6`
    Numpad6,
    /// `7 Home` on a keyboard
    ///
    /// `7 PQRS` or `7 PRS` on a phone or remote control
    ///
    /// USB HID:
    ///  - `Keypad 7 and Home` `Keyboard Page 0x5f`
    ///  - `Phone Key 7` `Telephony Device Page 0xb7`
    Numpad7,
    /// `8 ↑` on a keyboard
    ///
    /// `8 TUV` on a phone or remote control
    ///
    /// USB HID:
    ///  - `Keypad 8 and Up Arrow` `Keyboard Page 0x60`
    ///  - `Phone Key 8` `Telephony Device Page 0xb8`
    Numpad8,
    /// `9 PgUp` on a keyboard
    ///
    /// `9 WXYZ` or `9 WXY` on a phone or remote control
    ///
    /// USB HID:
    ///  - `Keypad 9 and PageUp` `Keyboard Page 0x61`
    ///  - `Phone Key 9` `Telephony Device Page 0xb9`
    Numpad9,
    /// `+`
    ///
    /// USB HID:
    ///  - `Keypad +` `Keyboard Page 0x57`
    ///  - `Menu Value Increase` `Consumer Page 0x47`
    NumpadAdd,
    /// Found on the Microsoft Natural Keyboard.
    ///
    /// USB HID:
    ///  - `Keypad Backspace` `Keyboard Page 0xbb`
    NumpadBackspace,
    /// `C` or `AC` (All Clear). Also for use with numpads that have a `Clear`
    /// key that is separate from the `NumLock` key. On the Mac, the numpad
    /// `Clear` key should always be encoded as `"NumLock"`.
    ///
    /// USB HID:
    ///  - `Keypad Clear` `Keyboard Page 0xd8`
    NumpadClear,
    /// `CE` (Clear Entry)
    ///
    /// USB HID:
    ///  - `Keypad Clear Entry` `Keyboard Page 0xd9`
    NumpadClearEntry,
    /// `,` (thousands separator). For locales where the thousands separator is
    /// a "." (e.g., Brazil), this key may generate a `.`.
    ///
    /// USB HID:
    ///  - `Keypad Comma` `Keyboard Page 0x85`
    NumpadComma,
    /// `. Del`. For locales where the decimal separator is "," (e.g., Brazil),
    /// this key may generate a `,`.
    ///
    /// USB HID:
    ///  - `Keypad . and Delete` `Keyboard Page 0x63`
    NumpadDecimal,
    /// `/`
    ///
    /// USB HID:
    ///  - `Keypad /` `Keyboard Page 0x54`
    NumpadDivide,
    /// USB HID:
    ///  - `Keypad ENTER` `Keyboard Page 0x58`
    NumpadEnter,
    /// `=`
    ///
    /// USB HID:
    ///  - `Keypad =` `Keyboard Page 0x67`
    NumpadEqual,
    /// `#` on a phone or remote control device. This key is typically found
    /// below the `9` key and to the right of the `0` key.
    ///
    /// USB HID:
    ///  - `Phone Key Pound` `Telephony Device Page 0xbb`
    NumpadHash,
    /// `M+` Add current entry to the value stored in memory.
    ///
    /// USB HID:
    ///  - `Keypad Memory Add` `Keyboard Page 0xd3`
    NumpadMemoryAdd,
    /// `MC` Clear the value stored in memory.
    ///
    /// USB HID:
    ///  - `Keypad Memory Clear` `Keyboard Page 0xd2`
    NumpadMemoryClear,
    /// `MR` Replace the current entry with the value stored in memory.
    ///
    /// USB HID:
    ///  - `Keypad Memory Recall` `Keyboard Page 0xd1`
    NumpadMemoryRecall,
    /// `MS` Replace the value stored in memory with the current entry.
    ///
    /// USB HID:
    ///  - `Keypad Memory Store` `Keyboard Page 0xd0`
    NumpadMemoryStore,
    /// `M-` Subtract current entry from the value
    /// stored in memory.
    ///
    /// USB HID:
    ///  - `Keypad Memory Subtract` `Keyboard Page 0xd4`
    NumpadMemorySubtract,
    /// `*` on a keyboard. For use with numpads that provide mathematical
    /// operations (`+`, `-`, `*` and `/`).
    ///
    /// Use [`KeyCode::NumpadStar`] for the `*` key on phones and remote controls.
    ///
    /// USB HID:
    ///  - `Keypad *` `Keyboard Page 0x55`
    NumpadMultiply,
    /// `(` Found on the Microsoft Natural Keyboard.
    ///
    /// USB HID:
    ///  - `Keypad (` `Keyboard Page 0xb6`
    NumpadParenLeft,
    /// `)` Found on the Microsoft Natural Keyboard.
    ///
    /// USB HID:
    ///  - `Keypad )` `Keyboard Page 0xb7`
    NumpadParenRight,
    /// `*` on a phone or remote control device. This key is typically found
    /// below the `7` key and to the left of the `0` key.
    ///
    /// Use [`KeyCode::NumpadMultiply`] for the `*` key on numeric keypads.
    ///
    /// USB HID:
    ///  - `Phone Key Star` `Telephony Device Page 0xba`
    NumpadStar,
    /// `-`
    ///
    /// USB HID:
    ///  - `Keypad -` `Keyboard Page 0x56`
    ///  - `Menu Value Decrease` `Consumer Page 0x48`
    NumpadSubtract,

    /// `Esc` or `⎋`
    ///
    /// USB HID:
    ///  - `Keyboard ESCAPE` `Keyboard Page 0x29`
    ///  - `Menu Escape` `Consumer Page 0x46`
    Escape,
    /// `F1`
    ///
    /// USB HID:
    ///  - `Keyboard F1` `Keyboard Page 0x3a`
    F1,
    /// `F2`
    ///
    /// USB HID:
    ///  - `Keyboard F2` `Keyboard Page 0x3b`
    F2,
    /// `F3`
    ///
    /// USB HID:
    ///  - `Keyboard F3` `Keyboard Page 0x3c`
    F3,
    /// `F4`
    ///
    /// USB HID:
    ///  - `Keyboard F4` `Keyboard Page 0x3d`
    F4,
    /// `F5`
    ///
    /// USB HID:
    ///  - `Keyboard F5` `Keyboard Page 0x3e`
    F5,
    /// `F6`
    ///
    /// USB HID:
    ///  - `Keyboard F6` `Keyboard Page 0x3f`
    F6,
    /// `F7`
    ///
    /// USB HID:
    ///  - `Keyboard F7` `Keyboard Page 0x40`
    F7,
    /// `F8`
    ///
    /// USB HID:
    ///  - `Keyboard F8` `Keyboard Page 0x41`
    F8,
    /// `F9`
    ///
    /// USB HID:
    ///  - `Keyboard F9` `Keyboard Page 0x42`
    F9,
    /// `F10`
    ///
    /// USB HID:
    ///  - `Keyboard F10` `Keyboard Page 0x43`
    F10,
    /// `F11`
    ///
    /// USB HID:
    ///  - `Keyboard F11` `Keyboard Page 0x44`
    F11,
    /// `F12`
    ///
    /// USB HID:
    ///  - `Keyboard F12` `Keyboard Page 0x45`
    F12,
    /// `F13`
    ///
    /// USB HID:
    ///  - `Keyboard F13` `Keyboard Page 0x68`
    F13,
    /// `F14`
    ///
    /// USB HID:
    ///  - `Keyboard F14` `Keyboard Page 0x69`
    F14,
    /// `F15`
    ///
    /// USB HID:
    ///  - `Keyboard F15` `Keyboard Page 0x6a`
    F15,
    /// `F16`
    ///
    /// USB HID:
    ///  - `Keyboard F16` `Keyboard Page 0x6b`
    F16,
    /// `F17`
    ///
    /// USB HID:
    ///  - `Keyboard F17` `Keyboard Page 0x6c`
    F17,
    /// `F18`
    ///
    /// USB HID:
    ///  - `Keyboard F18` `Keyboard Page 0x6d`
    F18,
    /// `F19`
    ///
    /// USB HID:
    ///  - `Keyboard F19` `Keyboard Page 0x6e`
    F19,
    /// `F20`
    ///
    /// USB HID:
    ///  - `Keyboard F20` `Keyboard Page 0x6f`
    F20,
    /// `F21`
    ///
    /// USB HID:
    ///  - `Keyboard F21` `Keyboard Page 0x70`
    F21,
    /// `F22`
    ///
    /// USB HID:
    ///  - `Keyboard F22` `Keyboard Page 0x71`
    F22,
    /// `F23`
    ///
    /// USB HID:
    ///  - `Keyboard F23` `Keyboard Page 0x72`
    F23,
    /// `F24`
    ///
    /// USB HID:
    ///  - `Keyboard F24` `Keyboard Page 0x73`
    F24,
    /// `Fn` This is typically a hardware key that does not generate a separate
    /// code. Most keyboards do not place this key in the function section, but
    /// it is included here to keep it with related keys.
    ///
    /// USB HID:
    ///
    Fn,
    /// `FLock` or `FnLock`. Function Lock key. Found on the Microsoft Natural
    /// Keyboard.
    ///
    /// USB HID:
    ///
    FnLock,
    /// `PrtScr SysRq` or `Print Screen`
    ///
    /// USB HID:
    ///  - `Keyboard PrintScreen` `Keyboard Page 0x46`
    PrintScreen,
    /// `Scroll Lock`
    ///
    /// USB HID:
    ///  - `Keyboard Scroll Lock` `Keyboard Page 0x47`
    ScrollLock,
    /// `Pause Break`
    ///
    /// USB HID:
    ///  - `Keyboard Pause` `Keyboard Page 0x48`
    ///  - `Pause` `Consumer Page 0xb1`
    Pause,

    /// Some laptops place this key to the left of the `↑` key.
    ///
    /// USB HID:
    ///  - `Reserved` `Keyboard Page 0xf1`
    ///  - `AC Back` `Consumer Page 0x224`
    BrowserBack,
    /// USB HID:
    ///  - `AL Programmable Button Configuration` `Consumer Page 0x182`
    ///  - `AC Bookmarks` `Consumer Page 0x22a`
    BrowserFavorites,
    /// Some laptops place this key to the right of the `↑` key.
    ///
    /// USB HID:
    ///  - `Reserved` `Keyboard Page 0xf2`
    ///  - `AC Forward` `Consumer Page 0x225`
    BrowserForward,
    /// USB HID:
    ///  - `AC Home` `Consumer Page 0x223`
    BrowserHome,
    /// USB HID:
    ///  - `Reserved` `Keyboard Page 0xfa`
    ///  - `AC Refresh` `Consumer Page 0x227`
    BrowserRefresh,
    /// USB HID:
    ///  - `AC Search` `Consumer Page 0x221`
    BrowserSearch,
    /// USB HID:
    ///  - `Keyboard Stop` `Keyboard Page 0x78`
    ///  - `Reserved` `Keyboard Page 0xf3`
    ///  - `AC Stop` `Consumer Page 0x226`
    BrowserStop,
    /// `Eject` or `⏏`. This key is placed in the function section on some Apple
    /// keyboards.
    ///
    /// USB HID:
    ///  - `Reserved` `Keyboard Page 0xec`
    ///  - `Eject` `Consumer Page 0xb8`
    Eject,
    /// Sometimes labelled `My Computer` on the keyboard
    ///
    /// USB HID:
    ///  - `AL Local Machine Browser` `Consumer Page 0x194`
    ///  - `AL File Browser` `Consumer Page 0x1b4`
    LaunchApp1,
    /// Sometimes labelled `Calculator` on the keyboard
    ///
    /// USB HID:
    ///  - `Reserved` `Keyboard Page 0xfb`
    ///  - `AL Calculator` `Consumer Page 0x192`
    LaunchApp2,
    /// USB HID:
    ///  - `AL Email Reader` `Consumer Page 0x18a`
    LaunchMail,
    /// USB HID:
    ///  - `Reserved` `Keyboard Page 0xe8`
    ///  - `Play/Pause` `Consumer Page 0xcd`
    MediaPlayPause,
    /// USB HID:
    ///  - `AL Consumer Control Configuration` `Consumer Page 0x183`
    MediaSelect,
    /// USB HID:
    ///  - `Reserved` `Keyboard Page 0xe9`
    ///  - `Stop` `Consumer Page 0xb7`
    MediaStop,
    /// USB HID:
    ///  - `Reserved` `Keyboard Page 0xeb`
    ///  - `Scan Next Track` `Consumer Page 0xb5`
    MediaTrackNext,
    /// USB HID:
    ///  - `Reserved` `Keyboard Page 0xea`
    ///  - `Scan Previous Track` `Consumer Page 0xb6`
    MediaTrackPrevious,
    /// This key is placed in the function section on some Apple keyboards,
    /// replacing the `Eject` key.
    ///
    /// USB HID:
    ///  - `Keyboard Power` `Keyboard Page 0x66`
    ///  - `Power` `Consumer Page 0x30`
    Power,
    /// USB HID:
    ///  - `Reserved` `Keyboard Page 0xf8`
    ///  - `Sleep` `Consumer Page 0x32`
    ///  - `Sleep Mode` `Consumer Page 0x34`
    Sleep,
    /// USB HID:
    ///  - `Keyboard Volume Down` `Keyboard Page 0x81`
    ///  - `Reserved` `Keyboard Page 0xee`
    ///  - `Volume Decrement` `Consumer Page 0xea`
    AudioVolumeDown,
    /// USB HID:
    ///  - `Keyboard Mute` `Keyboard Page 0x7f`
    ///  - `Reserved` `Keyboard Page 0xef`
    ///  - `Mute` `Consumer Page 0xe2`
    AudioVolumeMute,
    /// USB HID:
    ///  - `Keyboard Volume Up` `Keyboard Page 0x80`
    ///  - `Reserved` `Keyboard Page 0xed`
    ///  - `Volume Increment` `Consumer Page 0xe9`
    AudioVolumeUp,
    /// USB HID:
    ///  - `System Wake Up` `Generic Desktop Page 0x83`
    WakeUp,

    /// Found on Sun’s USB keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard Again` `Keyboard Page 0x79`
    Again,
    /// Found on Sun’s USB keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard Copy` `Keyboard Page 0x7c`
    ///  - `AC Copy` `Consumer Page 0x21b`
    Copy,
    /// Found on Sun’s USB keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard Cut` `Keyboard Page 0x7b`
    ///  - `AC Cut` `Consumer Page 0x21c`
    Cut,
    /// Found on Sun’s USB keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard Find` `Keyboard Page 0x7e`
    ///  - `Reserved` `Keyboard Page 0xf4`
    ///  - `AC Find` `Consumer Page 0x21f`
    Find,
    /// Found on Sun’s USB keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard Execute` `Keyboard Page 0x74`
    ///  - `AC Open` `Consumer Page 0x202`
    Open,
    /// Found on Sun’s USB keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard Paste` `Keyboard Page 0x7d`
    ///  - `AC Paste` `Consumer Page 0x21d`
    Paste,
    /// Found on Sun’s USB keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard Menu` `Keyboard Page 0x76`
    ///  - `AC Properties` `Consumer Page 0x209`
    Props,
    /// Found on Sun’s USB keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard Select` `Keyboard Page 0x77`
    Select,
    /// Found on Sun’s USB keyboard.
    ///
    /// USB HID:
    ///  - `Keyboard Undo` `Keyboard Page 0x7a`
    ///  - `AC Undo` `Consumer Page 0x21a`
    Undo,

    /// USB HID:
    ///  - `Button 1 (primary/trigger)` `Button Page 0x1`
    Gamepad0,
    /// USB HID:
    ///  - `Button 2 (secondary)` `Button Page 0x2`
    Gamepad1,
    /// USB HID:
    ///  - `Button 3 (tertiary)` `Button Page 0x3`
    Gamepad2,
    /// USB HID:
    ///  - `Button 4` `Button Page 0x4`
    Gamepad3,
    /// USB HID:
    ///  - `Button 5` `Button Page 0x5`
    Gamepad4,
    /// USB HID:
    ///  - `Button 6` `Button Page 0x6`
    Gamepad5,
    /// USB HID:
    ///  - `Button 7` `Button Page 0x7`
    Gamepad6,
    /// USB HID:
    ///  - `Button 8` `Button Page 0x8`
    Gamepad7,
    /// USB HID:
    ///  - `Button 9` `Button Page 0x9`
    Gamepad8,
    /// USB HID:
    ///  - `Button 10` `Button Page 0xa`
    Gamepad9,
    /// USB HID:
    ///  - `Button 11` `Button Page 0xb`
    Gamepad10,
    /// USB HID:
    ///  - `Button 12` `Button Page 0xc`
    Gamepad11,
    /// USB HID:
    ///  - `Button 13` `Button Page 0xd`
    Gamepad12,
    /// USB HID:
    ///  - `Button 14` `Button Page 0xe`
    Gamepad13,
    /// USB HID:
    ///  - `Button 15` `Button Page 0xf`
    Gamepad14,
    /// USB HID:
    ///  - `Button 16` `Button Page 0x10`
    Gamepad15,
    /// USB HID:
    ///  - `Button 17` `Button Page 0x11`
    Gamepad16,
    /// USB HID:
    ///  - `Button 18` `Button Page 0x12`
    Gamepad17,
    /// USB HID:
    ///  - `Button 19` `Button Page 0x13`
    Gamepad18,
    /// USB HID:
    ///  - `Button 20` `Button Page 0x14`
    Gamepad19,

    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `Display Brightness Decrement` `Consumer Page 0x70`
    BrightnessDown,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `Display Brightness Increment` `Consumer Page 0x6f`
    BrightnessUp,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `System Display Toggle Int/Ext Mode` `Generic Desktop Page 0xb5`
    DisplayToggleIntExt,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `AC Next Keyboard Layout Select` `Consumer Page 0x29d`
    KeyboardLayoutSelect,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `AL Context-aware Desktop Assistant` `Consumer Page 0x1cb`
    LaunchAssistant,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `AL Control Panel` `Consumer Page 0x19f`
    LaunchControlPanel,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `AL Screen Saver` `Consumer Page 0x1b1`
    LaunchScreenSaver,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `AC Forward Msg` `Consumer Page 0x28b`
    MailForward,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `AC Reply` `Consumer Page 0x289`
    MailReply,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `AC Send` `Consumer Page 0x28c`
    MailSend,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `Fast Forward` `Consumer Page 0xb3`
    MediaFastForward,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `Play` `Consumer Page 0xb0`
    MediaPlay,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `Pause` `Consumer Page 0xb1`
    MediaPause,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `Record` `Consumer Page 0xb2`
    MediaRecord,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `Rewind` `Consumer Page 0xb4`
    MediaRewind,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `Phone Mute` `Telephony Device Page 0x2f`
    MicrophoneMuteToggle,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `Privacy Screen Toggle` `Consumer Page 0x2d0`
    PrivacyScreenToggle,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `AL Select Task/Application` `Consumer Page 0x1a2`
    SelectTask,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `AC Desktop Show All Windows` `Consumer Page 0x29f`
    ShowAllWindows,
    /// Non-standard code value supported by Chromium.
    ///
    /// USB HID:
    ///  - `AC View Toggle` `Consumer Page 0x232`
    ZoomToggle,
}

/// Every [`KeyCode`] is grouped into one of these classes.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum KeyCodeClass {
    /// The *writing system keys* are those that change meaning (i.e., they
    /// produce different key values) based on the current locale and keyboard
    /// layout.
    WritingSystem,
    /// The *functional keys* (not to be confused with the [function
    /// keys](KeyCodeClass::Function) described later) are those keys in the
    /// alphanumeric section that provide general editing functions that are
    /// common to all locales (like `Shift`, `Tab`, `Enter` and `Backspace`).
    /// With a few exceptions, these keys do not change meaning based on the
    /// current keyboard layout.
    Functional,
    /// The *control pad* section of the keyboard is the set of (usually 6) keys
    /// that perform navigating and editing operations, for example, `Home`,
    /// `PageUp` and `Insert`.
    ControlPad,
    /// The *arrow pad* contains the 4 arrow keys. The keys are commonly
    /// arranged in an "upside-down T" configuration.
    ArrowPad,
    /// The *numpad section* is the set of keys on the keyboard arranged in a
    /// grid like a calculator or mobile phone. This section contains numeric
    /// and mathematical operator keys. Often this section will contain a
    /// `NumLock` key which causes the keys to switch between the standard
    /// numeric functions and mimicking the keys of the [control
    /// pad](KeyCodeClass::ControlPad) and arrow pad. Laptop computers and
    /// compact keyboards will commonly omit these keys to save space.
    Numpad,
    /// The *function section* runs along the top of the keyboard (above the
    /// alphanumeric section) and contains the function keys and a few
    /// additional special keys (for example, `Esc` and `Print Screen`).
    Function,
    /// *Media keys* are extra keys added to a keyboard that provide media related
    /// functionality like play, pause or volume control. These keys do not have
    /// a standard location on the keyboard so keyboards from different
    /// manufacturers are likely to have a different arrangement of keys or a
    /// completely different sets of keys.
    Media,
    /// These keys are not found on modern standard keyboards. They are listed
    /// here for reference purposes.
    Legacy,
    /// These buttons are found on gamepads.
    Gamepad,
    /// These keys are supported by some browsers.
    NonStandard,
}

impl KeyCode {
    /// Resolve the KeyCode according to the standard US layout.
    pub const fn as_str(self) -> &'static str {
        use self::KeyCode::*;
        match self {
            Backquote => "`",
            Backslash => r"\",
            BracketLeft => "[",
            BracketRight => "]",
            Comma => ",",
            Digit0 => "0",
            Digit1 => "1",
            Digit2 => "2",
            Digit3 => "3",
            Digit4 => "4",
            Digit5 => "5",
            Digit6 => "6",
            Digit7 => "7",
            Digit8 => "8",
            Digit9 => "9",
            Equal => "=",
            IntlBackslash => r"International Backslash",
            IntlRo => "ろ",
            IntlYen => "¥",
            KeyA => "A",
            KeyB => "B",
            KeyC => "C",
            KeyD => "D",
            KeyE => "E",
            KeyF => "F",
            KeyG => "G",
            KeyH => "H",
            KeyI => "I",
            KeyJ => "J",
            KeyK => "K",
            KeyL => "L",
            KeyM => "M",
            KeyN => "N",
            KeyO => "O",
            KeyP => "P",
            KeyQ => "Q",
            KeyR => "R",
            KeyS => "S",
            KeyT => "T",
            KeyU => "U",
            KeyV => "V",
            KeyW => "W",
            KeyX => "X",
            KeyY => "Y",
            KeyZ => "Z",
            Minus => "-",
            Period => ".",
            Quote => "'",
            Semicolon => ";",
            Slash => "/",
            AltLeft => "Alt Left",
            AltRight => "Alt Right",
            Backspace => "⌫",
            CapsLock => "⇪",
            ContextMenu => "Context Menu",
            ControlLeft => "Control Left",
            ControlRight => "Control Right",
            Enter => "↵",
            MetaLeft => "⌘ Left",
            MetaRight => "⌘ Right",
            ShiftLeft => "⇧ Left",
            ShiftRight => "⇧ Right",
            Space => "Space",
            Tab => "⇥",
            Convert => "変換",
            KanaMode => "カタカナ/ひらがな/ローマ字",
            Lang1 => "한/영 かな",
            Lang2 => "한자 英数",
            Lang3 => "カタカナ",
            Lang4 => "ひらがな",
            Lang5 => "半角/全角/漢字",
            NonConvert => "無変換",
            Delete => "Delete",
            End => "End",
            Help => "Help",
            Home => "Home",
            Insert => "Insert",
            PageDown => "Page Down",
            PageUp => "Page Up",
            ArrowDown => "↓",
            ArrowLeft => "←",
            ArrowRight => "→",
            ArrowUp => "↑",
            NumLock => "Num Lock",
            Numpad0 => "Numpad 0",
            Numpad1 => "Numpad 1",
            Numpad2 => "Numpad 2",
            Numpad3 => "Numpad 3",
            Numpad4 => "Numpad 4",
            Numpad5 => "Numpad 5",
            Numpad6 => "Numpad 6",
            Numpad7 => "Numpad 7",
            Numpad8 => "Numpad 8",
            Numpad9 => "Numpad 9",
            NumpadAdd => "Numpad +",
            NumpadBackspace => "Numpad ⌫",
            NumpadClear => "Numpad C",
            NumpadClearEntry => "Numpad CE",
            NumpadComma => "Numpad ,",
            NumpadDecimal => "Numpad .",
            NumpadDivide => "Numpad /",
            NumpadEnter => "Numpad ↵",
            NumpadEqual => "Numpad =",
            NumpadHash => "Numpad #",
            NumpadMemoryAdd => "Numpad M+",
            NumpadMemoryClear => "Numpad MC",
            NumpadMemoryRecall => "Numpad MR",
            NumpadMemoryStore => "Numpad MS",
            NumpadMemorySubtract => "Numpad M-",
            NumpadMultiply => "Numpad *",
            NumpadParenLeft => "Numpad (",
            NumpadParenRight => "Numpad )",
            NumpadStar => "Numpad * (Star)",
            NumpadSubtract => "Numpad -",
            Escape => "Escape",
            F1 => "F1",
            F2 => "F2",
            F3 => "F3",
            F4 => "F4",
            F5 => "F5",
            F6 => "F6",
            F7 => "F7",
            F8 => "F8",
            F9 => "F9",
            F10 => "F10",
            F11 => "F11",
            F12 => "F12",
            F13 => "F13",
            F14 => "F14",
            F15 => "F15",
            F16 => "F16",
            F17 => "F17",
            F18 => "F18",
            F19 => "F19",
            F20 => "F20",
            F21 => "F21",
            F22 => "F22",
            F23 => "F23",
            F24 => "F24",
            Fn => "Fn",
            FnLock => "FnLock",
            PrintScreen => "Print Screen",
            ScrollLock => "Scroll Lock",
            Pause => "Pause Break",
            BrowserBack => "Browser ⏮",
            BrowserFavorites => "Browser Favorites",
            BrowserForward => "Browser ⏭",
            BrowserHome => "Browser 🏠",
            BrowserRefresh => "Browser Refresh",
            BrowserSearch => "Browser Search",
            BrowserStop => "Browser Stop",
            Eject => "⏏",
            LaunchApp1 => "Launch App 1",
            LaunchApp2 => "Launch App 2",
            LaunchMail => "Launch Mail",
            MediaPlayPause => "⏯",
            MediaSelect => "Media Select",
            MediaStop => "◼",
            MediaTrackNext => "⏭",
            MediaTrackPrevious => "⏮",
            Power => "Power",
            Sleep => "Sleep",
            AudioVolumeDown => "🔉",
            AudioVolumeMute => "🔇",
            AudioVolumeUp => "🔊",
            WakeUp => "Wake Up",
            Again => "Again",
            Copy => "Copy",
            Cut => "Cut",
            Find => "Find",
            Open => "Open",
            Paste => "Paste",
            Props => "Props",
            Select => "Select",
            Undo => "Undo",
            Gamepad0 => "Gamepad 0",
            Gamepad1 => "Gamepad 1",
            Gamepad2 => "Gamepad 2",
            Gamepad3 => "Gamepad 3",
            Gamepad4 => "Gamepad 4",
            Gamepad5 => "Gamepad 5",
            Gamepad6 => "Gamepad 6",
            Gamepad7 => "Gamepad 7",
            Gamepad8 => "Gamepad 8",
            Gamepad9 => "Gamepad 9",
            Gamepad10 => "Gamepad 10",
            Gamepad11 => "Gamepad 11",
            Gamepad12 => "Gamepad 12",
            Gamepad13 => "Gamepad 13",
            Gamepad14 => "Gamepad 14",
            Gamepad15 => "Gamepad 15",
            Gamepad16 => "Gamepad 16",
            Gamepad17 => "Gamepad 17",
            Gamepad18 => "Gamepad 18",
            Gamepad19 => "Gamepad 19",
            BrightnessDown => "Brightness Down",
            BrightnessUp => "Brightness Up",
            DisplayToggleIntExt => "Display Toggle Intern / Extern",
            KeyboardLayoutSelect => "Keyboard Layout Select",
            LaunchAssistant => "Launch Assistant",
            LaunchControlPanel => "Launch Control Panel",
            LaunchScreenSaver => "Launch Screen Saver",
            MailForward => "Mail Forward",
            MailReply => "Mail Reply",
            MailSend => "Mail Send",
            MediaFastForward => "⏩",
            MediaPause => "⏸",
            MediaPlay => "▶",
            MediaRecord => "⏺",
            MediaRewind => "⏪",
            MicrophoneMuteToggle => "Microphone Mute Toggle",
            PrivacyScreenToggle => "Privacy Screen Toggle",
            SelectTask => "Select Task",
            ShowAllWindows => "Show All Windows",
            ZoomToggle => "Zoom Toggle",
        }
    }

    /// Classifies a key based on its grouping on the Keyboard.
    pub const fn classify(self) -> KeyCodeClass {
        use self::KeyCode::*;
        match self {
            // Writing System Keys
            Backquote | Backslash | BracketLeft | BracketRight | Comma | Digit0 | Digit1
            | Digit2 | Digit3 | Digit4 | Digit5 | Digit6 | Digit7 | Digit8 | Digit9 | Equal
            | IntlBackslash | IntlRo | IntlYen | KeyA | KeyB | KeyC | KeyD | KeyE | KeyF | KeyG
            | KeyH | KeyI | KeyJ | KeyK | KeyL | KeyM | KeyN | KeyO | KeyP | KeyQ | KeyR | KeyS
            | KeyT | KeyU | KeyV | KeyW | KeyX | KeyY | KeyZ | Minus | Period | Quote
            | Semicolon | Slash => KeyCodeClass::WritingSystem,

            // Functional Keys
            AltLeft | AltRight | Backspace | CapsLock | ContextMenu | ControlLeft
            | ControlRight | Enter | MetaLeft | MetaRight | ShiftLeft | ShiftRight | Space
            | Tab | Convert | KanaMode | Lang1 | Lang2 | Lang3 | Lang4 | Lang5 | NonConvert => {
                KeyCodeClass::Functional
            }

            // Control Pad Section
            Delete | End | Help | Home | Insert | PageDown | PageUp => KeyCodeClass::ControlPad,

            // Arrow Pad Section
            ArrowDown | ArrowLeft | ArrowRight | ArrowUp => KeyCodeClass::ArrowPad,

            // Numpad Section
            NumLock | Numpad0 | Numpad1 | Numpad2 | Numpad3 | Numpad4 | Numpad5 | Numpad6
            | Numpad7 | Numpad8 | Numpad9 | NumpadAdd | NumpadBackspace | NumpadClear
            | NumpadClearEntry | NumpadComma | NumpadDecimal | NumpadDivide | NumpadEnter
            | NumpadEqual | NumpadHash | NumpadMemoryAdd | NumpadMemoryClear
            | NumpadMemoryRecall | NumpadMemoryStore | NumpadMemorySubtract | NumpadMultiply
            | NumpadParenLeft | NumpadParenRight | NumpadStar | NumpadSubtract => {
                KeyCodeClass::Numpad
            }

            // Function Section
            Escape | F1 | F2 | F3 | F4 | F5 | F6 | F7 | F8 | F9 | F10 | F11 | F12 | F13 | F14
            | F15 | F16 | F17 | F18 | F19 | F20 | F21 | F22 | F23 | F24 | Fn | FnLock
            | PrintScreen | ScrollLock | Pause => KeyCodeClass::Function,

            // Media Keys
            BrowserBack | BrowserFavorites | BrowserForward | BrowserHome | BrowserRefresh
            | BrowserSearch | BrowserStop | Eject | LaunchApp1 | LaunchApp2 | LaunchMail
            | MediaPlayPause | MediaSelect | MediaStop | MediaTrackNext | MediaTrackPrevious
            | Power | Sleep | AudioVolumeDown | AudioVolumeMute | AudioVolumeUp | WakeUp => {
                KeyCodeClass::Media
            }

            // Legacy, Non-Standard and Special Keys
            Again | Copy | Cut | Find | Open | Paste | Props | Select | Undo => {
                KeyCodeClass::Legacy
            }

            // Gamepad Keys
            Gamepad0 | Gamepad1 | Gamepad2 | Gamepad3 | Gamepad4 | Gamepad5 | Gamepad6
            | Gamepad7 | Gamepad8 | Gamepad9 | Gamepad10 | Gamepad11 | Gamepad12 | Gamepad13
            | Gamepad14 | Gamepad15 | Gamepad16 | Gamepad17 | Gamepad18 | Gamepad19 => {
                KeyCodeClass::Gamepad
            }

            // Browser specific Keys
            BrightnessDown | BrightnessUp | DisplayToggleIntExt | KeyboardLayoutSelect
            | LaunchAssistant | LaunchControlPanel | LaunchScreenSaver | MailForward
            | MailReply | MailSend | MediaFastForward | MediaPause | MediaPlay | MediaRecord
            | MediaRewind | MicrophoneMuteToggle | PrivacyScreenToggle | SelectTask
            | ShowAllWindows | ZoomToggle => KeyCodeClass::NonStandard,
        }
    }

    /// Resolves the key according to the current keyboard layout.
    pub fn resolve(self) -> Cow<'static, str> {
        let class = self.classify();
        if class == KeyCodeClass::WritingSystem {
            if let Some(resolved) = crate::platform::try_resolve(self) {
                let uppercase = if resolved != "ß" {
                    resolved.to_uppercase()
                } else {
                    resolved
                };
                return uppercase.into();
            }
        }
        self.as_str().into()
    }
}

impl FromStr for KeyCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::KeyCode::*;
        Ok(match s {
            // Writing System Keys
            "Backquote" => Backquote,
            "Backslash" => Backslash,
            "BracketLeft" => BracketLeft,
            "BracketRight" => BracketRight,
            "Comma" => Comma,
            "Digit0" | "0" => Digit0,
            "Digit1" | "1" => Digit1,
            "Digit2" | "2" => Digit2,
            "Digit3" | "3" => Digit3,
            "Digit4" | "4" => Digit4,
            "Digit5" | "5" => Digit5,
            "Digit6" | "6" => Digit6,
            "Digit7" | "7" => Digit7,
            "Digit8" | "8" => Digit8,
            "Digit9" | "9" => Digit9,
            "Equal" => Equal,
            "IntlBackslash" => IntlBackslash,
            "IntlRo" => IntlRo,
            "IntlYen" => IntlYen,
            "KeyA" | "A" => KeyA,
            "KeyB" | "B" => KeyB,
            "KeyC" | "C" => KeyC,
            "KeyD" | "D" => KeyD,
            "KeyE" | "E" => KeyE,
            "KeyF" | "F" => KeyF,
            "KeyG" | "G" => KeyG,
            "KeyH" | "H" => KeyH,
            "KeyI" | "I" => KeyI,
            "KeyJ" | "J" => KeyJ,
            "KeyK" | "K" => KeyK,
            "KeyL" | "L" => KeyL,
            "KeyM" | "M" => KeyM,
            "KeyN" | "N" => KeyN,
            "KeyO" | "O" => KeyO,
            "KeyP" | "P" => KeyP,
            "KeyQ" | "Q" => KeyQ,
            "KeyR" | "R" => KeyR,
            "KeyS" | "S" => KeyS,
            "KeyT" | "T" => KeyT,
            "KeyU" | "U" => KeyU,
            "KeyV" | "V" => KeyV,
            "KeyW" | "W" => KeyW,
            "KeyX" | "X" => KeyX,
            "KeyY" | "Y" => KeyY,
            "KeyZ" | "Z" => KeyZ,
            "Minus" => Minus,
            "Period" => Period,
            "Quote" => Quote,
            "Semicolon" => Semicolon,
            "Slash" => Slash,

            // Functional Keys
            "AltLeft" => AltLeft,
            "AltRight" => AltRight,
            "Backspace" => Backspace,
            "CapsLock" => CapsLock,
            "ContextMenu" => ContextMenu,
            "ControlLeft" => ControlLeft,
            "ControlRight" => ControlRight,
            "Enter" => Enter,
            // OS is used instead of Meta in all Firefox versions, Chrome <52
            // and all Safari GTK and WPE versions.
            // Firefox Tracking Issue: https://bugzilla.mozilla.org/show_bug.cgi?id=1595863
            "MetaLeft" | "OSLeft" => MetaLeft,
            "MetaRight" | "OSRight" => MetaRight,
            "ShiftLeft" => ShiftLeft,
            "ShiftRight" => ShiftRight,
            "Space" => Space,
            "Tab" => Tab,

            // Functional Keys found on Japanese and Korean keyboards
            "Convert" => Convert,
            "KanaMode" => KanaMode,
            "Lang1" => Lang1, // MDN claims Chrome uses `HangulMode` but it's not true
            "Lang2" => Lang2, // MDN claims Chrome uses `Hanja` but it's not true
            "Lang3" => Lang3,
            "Lang4" => Lang4,
            "Lang5" => Lang5,
            "NonConvert" => NonConvert,

            // Control Pad Section
            "Delete" => Delete,
            "End" => End,
            "Help" => Help,
            "Home" => Home,
            "Insert" => Insert,
            "PageDown" => PageDown,
            "PageUp" => PageUp,

            // Arrow Pad Section
            "ArrowDown" => ArrowDown,
            "ArrowLeft" => ArrowLeft,
            "ArrowRight" => ArrowRight,
            "ArrowUp" => ArrowUp,

            // Numpad Section
            "NumLock" => NumLock,
            "Numpad0" => Numpad0,
            "Numpad1" => Numpad1,
            "Numpad2" => Numpad2,
            "Numpad3" => Numpad3,
            "Numpad4" => Numpad4,
            "Numpad5" => Numpad5,
            "Numpad6" => Numpad6,
            "Numpad7" => Numpad7,
            "Numpad8" => Numpad8,
            "Numpad9" => Numpad9,
            "NumpadAdd" => NumpadAdd,
            "NumpadBackspace" => NumpadBackspace, // Not exposed by any OS.
            "NumpadClear" => NumpadClear,         // Not exposed by any OS.
            "NumpadClearEntry" => NumpadClearEntry, // Not exposed by any OS.
            "NumpadComma" => NumpadComma,
            "NumpadDecimal" => NumpadDecimal,
            "NumpadDivide" => NumpadDivide,
            "NumpadEnter" => NumpadEnter,
            "NumpadEqual" => NumpadEqual,
            "NumpadHash" => NumpadHash, // Not exposed by any OS.
            "NumpadMemoryAdd" => NumpadMemoryAdd, // Not exposed by any OS.
            "NumpadMemoryClear" => NumpadMemoryClear, // Not exposed by any OS.
            "NumpadMemoryRecall" => NumpadMemoryRecall, // Not exposed by any OS.
            "NumpadMemoryStore" => NumpadMemoryStore, // Not exposed by any OS.
            "NumpadMemorySubtract" => NumpadMemorySubtract, // Not exposed by any OS.
            "NumpadMultiply" => NumpadMultiply,
            "NumpadParenLeft" => NumpadParenLeft,
            "NumpadParenRight" => NumpadParenRight,
            "NumpadStar" => NumpadStar, // Not exposed by any OS.
            "NumpadSubtract" => NumpadSubtract,

            // Function Section
            "Escape" => Escape,
            "F1" => F1,
            "F2" => F2,
            "F3" => F3,
            "F4" => F4,
            "F5" => F5,
            "F6" => F6,
            "F7" => F7,
            "F8" => F8,
            "F9" => F9,
            "F10" => F10,
            "F11" => F11,
            "F12" => F12,
            "F13" => F13,
            "F14" => F14,
            "F15" => F15,
            "F16" => F16,
            "F17" => F17,
            "F18" => F18,
            "F19" => F19,
            "F20" => F20,
            "F21" => F21,
            "F22" => F22,
            "F23" => F23,
            "F24" => F24,
            "Fn" => Fn,
            "FnLock" => FnLock, // Not exposed by any OS.
            "PrintScreen" => PrintScreen,
            "ScrollLock" => ScrollLock,
            "Pause" => Pause,

            // Media Keys
            "BrowserBack" => BrowserBack,
            "BrowserFavorites" => BrowserFavorites,
            "BrowserForward" => BrowserForward,
            "BrowserHome" => BrowserHome,
            "BrowserRefresh" => BrowserRefresh,
            "BrowserSearch" => BrowserSearch,
            "BrowserStop" => BrowserStop, // MDN claims it is `Cancel` in Chrome, but it never even was.
            "Eject" => Eject,
            "LaunchApp1" => LaunchApp1,
            "LaunchApp2" => LaunchApp2,
            "LaunchMail" => LaunchMail,
            "MediaPlayPause" => MediaPlayPause,
            // According to MDN some versions of Firefox use `LaunchMediaPlayer`
            // here, but that's wrong. However Safari GTK and WPE use this value.
            "MediaSelect" | "LaunchMediaPlayer" => MediaSelect,
            "MediaStop" => MediaStop,
            "MediaTrackNext" => MediaTrackNext,
            "MediaTrackPrevious" => MediaTrackPrevious,
            "Power" => Power,
            "Sleep" => Sleep,
            // Wrong Volume* names in Firefox are tracked here: https://bugzilla.mozilla.org/show_bug.cgi?id=1272579
            "AudioVolumeDown" | "VolumeDown" => AudioVolumeDown, // VolumeDown in old browsers, sometimes even new ones
            "AudioVolumeMute" | "VolumeMute" => AudioVolumeMute, // VolumeMute in old browsers, sometimes even new ones
            "AudioVolumeUp" | "VolumeUp" => AudioVolumeUp, // VolumeUp in old browsers, sometimes even new ones
            "WakeUp" => WakeUp,

            // Legacy, Non-Standard and Special Keys
            "Again" => Again,
            "Copy" => Copy,
            "Cut" => Cut,
            "Find" => Find,
            "Open" => Open,
            "Paste" => Paste,
            "Props" => Props,
            "Select" => Select,
            "Undo" => Undo,

            // Gamepad Keys
            "Gamepad0" => Gamepad0,
            "Gamepad1" => Gamepad1,
            "Gamepad2" => Gamepad2,
            "Gamepad3" => Gamepad3,
            "Gamepad4" => Gamepad4,
            "Gamepad5" => Gamepad5,
            "Gamepad6" => Gamepad6,
            "Gamepad7" => Gamepad7,
            "Gamepad8" => Gamepad8,
            "Gamepad9" => Gamepad9,
            "Gamepad10" => Gamepad10,
            "Gamepad11" => Gamepad11,
            "Gamepad12" => Gamepad12,
            "Gamepad13" => Gamepad13,
            "Gamepad14" => Gamepad14,
            "Gamepad15" => Gamepad15,
            "Gamepad16" => Gamepad16,
            "Gamepad17" => Gamepad17,
            "Gamepad18" => Gamepad18,
            "Gamepad19" => Gamepad19,

            // Browser specific Keys
            "BrightnessDown" => BrightnessDown,
            "BrightnessUp" => BrightnessUp,
            "DisplayToggleIntExt" => DisplayToggleIntExt,
            "KeyboardLayoutSelect" => KeyboardLayoutSelect,
            "LaunchAssistant" => LaunchAssistant,
            "LaunchControlPanel" => LaunchControlPanel,
            "LaunchScreenSaver" => LaunchScreenSaver,
            "MailForward" => MailForward,
            "MailReply" => MailReply,
            "MailSend" => MailSend,
            "MediaFastForward" => MediaFastForward,
            "MediaPause" => MediaPause,
            "MediaPlay" => MediaPlay,
            "MediaRecord" => MediaRecord,
            "MediaRewind" => MediaRewind,
            "MicrophoneMuteToggle" => MicrophoneMuteToggle,
            "PrivacyScreenToggle" => PrivacyScreenToggle,
            "SelectTask" => SelectTask,
            "ShowAllWindows" => ShowAllWindows,
            "ZoomToggle" => ZoomToggle,
            _ => return Err(()),
        })
    }
}
