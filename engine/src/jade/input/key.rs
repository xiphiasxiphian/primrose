use strum::EnumCount;
use winit::keyboard::KeyCode;

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

impl TryFrom<KeyCode> for Key
{
    type Error = ();

    fn try_from(code: KeyCode) -> Result<Self, Self::Error>
    {
        use KeyCode as W;
        match code
        {
            W::KeyA => Ok(Key::A),
            W::KeyB => Ok(Key::B),
            W::KeyC => Ok(Key::C),
            W::KeyD => Ok(Key::D),
            W::KeyE => Ok(Key::E),
            W::KeyF => Ok(Key::F),
            W::KeyG => Ok(Key::G),
            W::KeyH => Ok(Key::H),
            W::KeyI => Ok(Key::I),
            W::KeyJ => Ok(Key::J),
            W::KeyK => Ok(Key::K),
            W::KeyL => Ok(Key::L),
            W::KeyM => Ok(Key::M),
            W::KeyN => Ok(Key::N),
            W::KeyO => Ok(Key::O),
            W::KeyP => Ok(Key::P),
            W::KeyQ => Ok(Key::Q),
            W::KeyR => Ok(Key::R),
            W::KeyS => Ok(Key::S),
            W::KeyT => Ok(Key::T),
            W::KeyU => Ok(Key::U),
            W::KeyV => Ok(Key::V),
            W::KeyW => Ok(Key::W),
            W::KeyX => Ok(Key::X),
            W::KeyY => Ok(Key::Y),
            W::KeyZ => Ok(Key::Z),

            W::Digit0 => Ok(Key::N0),
            W::Digit1 => Ok(Key::N1),
            W::Digit2 => Ok(Key::N2),
            W::Digit3 => Ok(Key::N3),
            W::Digit4 => Ok(Key::N4),
            W::Digit5 => Ok(Key::N5),
            W::Digit6 => Ok(Key::N6),
            W::Digit7 => Ok(Key::N7),
            W::Digit8 => Ok(Key::N8),
            W::Digit9 => Ok(Key::N9),

            W::ArrowUp => Ok(Key::ArrowUp),
            W::ArrowDown => Ok(Key::ArrowDown),
            W::ArrowLeft => Ok(Key::ArrowLeft),
            W::ArrowRight => Ok(Key::ArrowRight),

            W::ShiftLeft => Ok(Key::ShiftLeft),
            W::ShiftRight => Ok(Key::ShiftRight),
            W::ControlLeft => Ok(Key::ControlLeft),
            W::ControlRight => Ok(Key::ControlRight),

            W::Space => Ok(Key::Space),
            W::Enter => Ok(Key::Enter),
            W::Escape => Ok(Key::Escape),
            W::Backspace => Ok(Key::Backspace),
            W::Tab => Ok(Key::Tab),
            W::CapsLock => Ok(Key::CapsLock),

            W::F1 => Ok(Key::F1),
            W::F2 => Ok(Key::F2),
            W::F3 => Ok(Key::F3),
            W::F4 => Ok(Key::F4),
            W::F5 => Ok(Key::F5),
            W::F6 => Ok(Key::F6),
            W::F7 => Ok(Key::F7),
            W::F8 => Ok(Key::F8),
            W::F9 => Ok(Key::F9),
            W::F10 => Ok(Key::F10),
            W::F11 => Ok(Key::F11),
            W::F12 => Ok(Key::F12),

            _ => Err(()),
        }
    }
}
