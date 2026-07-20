use strum::EnumCount;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
pub enum Key
{
    // Letters
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

    // Numbers (row, not numpad)
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,

    // Arrows
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,

    // Modifiers
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,

    // Common
    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    CapsLock,

    // Function keys
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

    // Numpad
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
    NumpadEnter,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
}

impl TryFrom<&str> for Key
{
    type Error = ();

    fn try_from(code: &str) -> Result<Self, Self::Error>
    {
        match code
        {
            "KeyA" => Ok(Key::A),
            "KeyB" => Ok(Key::B),
            "KeyC" => Ok(Key::C),
            "KeyD" => Ok(Key::D),
            "KeyE" => Ok(Key::E),
            "KeyF" => Ok(Key::F),
            "KeyG" => Ok(Key::G),
            "KeyH" => Ok(Key::H),
            "KeyI" => Ok(Key::I),
            "KeyJ" => Ok(Key::J),
            "KeyK" => Ok(Key::K),
            "KeyL" => Ok(Key::L),
            "KeyM" => Ok(Key::M),
            "KeyN" => Ok(Key::N),
            "KeyO" => Ok(Key::O),
            "KeyP" => Ok(Key::P),
            "KeyQ" => Ok(Key::Q),
            "KeyR" => Ok(Key::R),
            "KeyS" => Ok(Key::S),
            "KeyT" => Ok(Key::T),
            "KeyU" => Ok(Key::U),
            "KeyV" => Ok(Key::V),
            "KeyW" => Ok(Key::W),
            "KeyX" => Ok(Key::X),
            "KeyY" => Ok(Key::Y),
            "KeyZ" => Ok(Key::Z),

            "Digit0" => Ok(Key::N0),
            "Digit1" => Ok(Key::N1),
            "Digit2" => Ok(Key::N2),
            "Digit3" => Ok(Key::N3),
            "Digit4" => Ok(Key::N4),
            "Digit5" => Ok(Key::N5),
            "Digit6" => Ok(Key::N6),
            "Digit7" => Ok(Key::N7),
            "Digit8" => Ok(Key::N8),
            "Digit9" => Ok(Key::N9),

            "ArrowUp" => Ok(Key::ArrowUp),
            "ArrowDown" => Ok(Key::ArrowDown),
            "ArrowLeft" => Ok(Key::ArrowLeft),
            "ArrowRight" => Ok(Key::ArrowRight),

            "ShiftLeft" => Ok(Key::ShiftLeft),
            "ShiftRight" => Ok(Key::ShiftRight),
            "ControlLeft" => Ok(Key::ControlLeft),
            "ControlRight" => Ok(Key::ControlRight),
            "AltLeft" => Ok(Key::AltLeft),
            "AltRight" => Ok(Key::AltRight),

            "Space" => Ok(Key::Space),
            "Enter" => Ok(Key::Enter),
            "Escape" => Ok(Key::Escape),
            "Backspace" => Ok(Key::Backspace),
            "Tab" => Ok(Key::Tab),
            "CapsLock" => Ok(Key::CapsLock),

            "F1" => Ok(Key::F1),
            "F2" => Ok(Key::F2),
            "F3" => Ok(Key::F3),
            "F4" => Ok(Key::F4),
            "F5" => Ok(Key::F5),
            "F6" => Ok(Key::F6),
            "F7" => Ok(Key::F7),
            "F8" => Ok(Key::F8),
            "F9" => Ok(Key::F9),
            "F10" => Ok(Key::F10),
            "F11" => Ok(Key::F11),
            "F12" => Ok(Key::F12),

            "Numpad0" => Ok(Key::Numpad0),
            "Numpad1" => Ok(Key::Numpad1),
            "Numpad2" => Ok(Key::Numpad2),
            "Numpad3" => Ok(Key::Numpad3),
            "Numpad4" => Ok(Key::Numpad4),
            "Numpad5" => Ok(Key::Numpad5),
            "Numpad6" => Ok(Key::Numpad6),
            "Numpad7" => Ok(Key::Numpad7),
            "Numpad8" => Ok(Key::Numpad8),
            "Numpad9" => Ok(Key::Numpad9),
            "NumpadEnter" => Ok(Key::NumpadEnter),
            "NumpadAdd" => Ok(Key::NumpadAdd),
            "NumpadSubtract" => Ok(Key::NumpadSubtract),
            "NumpadMultiply" => Ok(Key::NumpadMultiply),
            "NumpadDivide" => Ok(Key::NumpadDivide),

            _ => Err(()), // unknown key, silently ignored
        }
    }
}
