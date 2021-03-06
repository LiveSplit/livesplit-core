use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum KeyCode {
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,

    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadClear,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEquals,
    NumpadMinus,
    NumpadMultiply,
    NumpadPlus,

    Backslash,
    Comma,
    Equal,
    Grave,
    LeftBracket,
    Minus,
    Period,
    Quote,
    RightBracket,
    Semicolon,
    Slash,
    CapsLock,
    Command,
    Control,
    Delete,
    DownArrow,
    End,
    Escape,
    ForwardDelete,
    Function,
    Help,
    Home,
    IsoSection,
    JisEisu,
    JisKana,
    JisKeypadComma,
    JisUnderscore,
    JisYen,
    LeftArrow,
    Mute,
    Option,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    RightCommand,
    RightControl,
    RightOption,
    RightShift,
    Shift,
    Space,
    Tab,
    UpArrow,
    VolumeDown,
    VolumeUp,
}

impl FromStr for KeyCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::KeyCode::*;
        Ok(match s {
            "Digit0" => Digit0,
            "Digit1" => Digit1,
            "Digit2" => Digit2,
            "Digit3" => Digit3,
            "Digit4" => Digit4,
            "Digit5" => Digit5,
            "Digit6" => Digit6,
            "Digit7" => Digit7,
            "Digit8" => Digit8,
            "Digit9" => Digit9,

            "A" => A,
            "B" => B,
            "C" => C,
            "D" => D,
            "E" => E,
            "F" => F,
            "G" => G,
            "H" => H,
            "I" => I,
            "J" => J,
            "K" => K,
            "L" => L,
            "M" => M,
            "N" => N,
            "O" => O,
            "P" => P,
            "Q" => Q,
            "R" => R,
            "S" => S,
            "T" => T,
            "U" => U,
            "V" => V,
            "W" => W,
            "X" => X,
            "Y" => Y,
            "Z" => Z,

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
            "NumpadClear" => NumpadClear,
            "NumpadDecimal" => NumpadDecimal,
            "NumpadDivide" => NumpadDivide,
            "NumpadEnter" => NumpadEnter,
            "NumpadEquals" => NumpadEquals,
            "NumpadMinus" => NumpadMinus,
            "NumpadMultiply" => NumpadMultiply,
            "NumpadPlus" => NumpadPlus,

            "Backslash" => Backslash,
            "Comma" => Comma,
            "Equal" => Equal,
            "Grave" => Grave,
            "LeftBracket" => LeftBracket,
            "Minus" => Minus,
            "Period" => Period,
            "Quote" => Quote,
            "RightBracket" => RightBracket,
            "Semicolon" => Semicolon,
            "Slash" => Slash,
            "CapsLock" => CapsLock,
            "Command" => Command,
            "Control" => Control,
            "Delete" => Delete,
            "DownArrow" => DownArrow,
            "End" => End,
            "Escape" => Escape,
            "ForwardDelete" => ForwardDelete,
            "Function" => Function,
            "Help" => Help,
            "Home" => Home,
            "IsoSection" => IsoSection,
            "JisEisu" => JisEisu,
            "JisKana" => JisKana,
            "JisKeypadComma" => JisKeypadComma,
            "JisUnderscore" => JisUnderscore,
            "JisYen" => JisYen,
            "LeftArrow" => LeftArrow,
            "Mute" => Mute,
            "Option" => Option,
            "PageDown" => PageDown,
            "PageUp" => PageUp,
            "Return" => Return,
            "RightArrow" => RightArrow,
            "RightCommand" => RightCommand,
            "RightControl" => RightControl,
            "RightOption" => RightOption,
            "RightShift" => RightShift,
            "Shift" => Shift,
            "Space" => Space,
            "Tab" => Tab,
            "UpArrow" => UpArrow,
            "VolumeDown" => VolumeDown,
            "VolumeUp" => VolumeUp,

            _ => return Err(()),
        })
    }
}
