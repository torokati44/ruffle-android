use android_activity::input::Keycode as Android;
use ruffle_core::events::KeyCode as Ruffle;

pub fn android_keycode_to_ruffle(android: Android) -> Option<Ruffle> {
    Some(match android {
        Android::A => Ruffle::A,
        Android::B => Ruffle::B,
        Android::C => Ruffle::C,
        Android::D => Ruffle::D,
        Android::E => Ruffle::E,
        Android::F => Ruffle::F,
        Android::G => Ruffle::G,
        Android::H => Ruffle::H,
        Android::I => Ruffle::I,
        Android::J => Ruffle::J,
        Android::K => Ruffle::K,
        Android::L => Ruffle::L,
        Android::M => Ruffle::M,
        Android::N => Ruffle::N,
        Android::O => Ruffle::O,
        Android::P => Ruffle::P,
        Android::Q => Ruffle::Q,
        Android::R => Ruffle::R,
        Android::S => Ruffle::S,
        Android::T => Ruffle::T,
        Android::U => Ruffle::U,
        Android::V => Ruffle::V,
        Android::W => Ruffle::W,
        Android::X => Ruffle::X,
        Android::Y => Ruffle::Y,
        Android::Z => Ruffle::Z,
        Android::Comma => Ruffle::Comma,
        Android::Period => Ruffle::Period,
        Android::Tab => Ruffle::Tab,
        Android::Space => Ruffle::Space,
        Android::Enter => Ruffle::Return,
        Android::Del => Ruffle::Delete,
        Android::Grave => Ruffle::Grave,
        Android::Minus => Ruffle::Minus,
        Android::Equals => Ruffle::Equals,
        Android::LeftBracket => Ruffle::LBracket,
        Android::RightBracket => Ruffle::RBracket,
        Android::Backslash => Ruffle::Backslash,
        Android::Semicolon => Ruffle::Semicolon,
        Android::Apostrophe => Ruffle::Apostrophe,
        Android::Slash => Ruffle::Slash,
        Android::Plus => Ruffle::Plus,
        Android::PageUp => Ruffle::PgUp,
        Android::PageDown => Ruffle::PgDown,
        Android::Escape => Ruffle::Escape,
        Android::ForwardDel => Ruffle::Delete,
        Android::CapsLock => Ruffle::CapsLock,
        Android::ScrollLock => Ruffle::ScrollLock,
        Android::MoveHome => Ruffle::Home,
        Android::MoveEnd => Ruffle::End,
        Android::Insert => Ruffle::Insert,
        Android::F1 => Ruffle::F1,
        Android::F2 => Ruffle::F2,
        Android::F3 => Ruffle::F3,
        Android::F4 => Ruffle::F4,
        Android::F5 => Ruffle::F5,
        Android::F6 => Ruffle::F6,
        Android::F7 => Ruffle::F7,
        Android::F8 => Ruffle::F8,
        Android::F9 => Ruffle::F9,
        Android::F10 => Ruffle::F10,
        Android::F11 => Ruffle::F11,
        Android::F12 => Ruffle::F12,
        Android::Numpad0 => Ruffle::Numpad0,
        Android::Numpad1 => Ruffle::Numpad1,
        Android::Numpad2 => Ruffle::Numpad2,
        Android::Numpad3 => Ruffle::Numpad3,
        Android::Numpad4 => Ruffle::Numpad4,
        Android::Numpad5 => Ruffle::Numpad5,
        Android::Numpad6 => Ruffle::Numpad6,
        Android::Numpad7 => Ruffle::Numpad7,
        Android::Numpad8 => Ruffle::Numpad8,
        Android::Numpad9 => Ruffle::Numpad9,
        Android::NumpadDivide => Ruffle::NumpadSlash,
        Android::NumpadSubtract => Ruffle::NumpadMinus,
        Android::NumpadDot => Ruffle::NumpadPeriod,
        Android::NumpadEnter => Ruffle::NumpadEnter,
        _ => return None,
    })
}
