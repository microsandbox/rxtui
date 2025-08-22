//! Key representation for keyboard input handling.
//!
//! This module provides a Key enum that represents both regular characters
//! and special keyboard keys, enabling type-safe event handling.

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Represents a keyboard key with modifier states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyWithModifiers {
    /// The key that was pressed
    pub key: Key,

    /// Whether Ctrl (or Cmd on macOS) was held
    pub ctrl: bool,

    /// Whether Alt (or Option on macOS) was held
    pub alt: bool,

    /// Whether Shift was held
    pub shift: bool,

    /// Whether Meta/Super key was held (Cmd on macOS, Win on Windows)
    pub meta: bool,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl KeyWithModifiers {
    /// Creates a new KeyWithModifiers with no modifiers pressed
    pub fn new(key: Key) -> Self {
        Self {
            key,
            ctrl: false,
            alt: false,
            shift: false,
            meta: false,
        }
    }

    /// Creates from a key with Ctrl/Cmd pressed
    pub fn with_ctrl(key: Key) -> Self {
        Self {
            key,
            ctrl: true,
            alt: false,
            shift: false,
            meta: false,
        }
    }

    /// Creates from a key with Alt/Option pressed
    pub fn with_alt(key: Key) -> Self {
        Self {
            key,
            ctrl: false,
            alt: true,
            shift: false,
            meta: false,
        }
    }

    /// Creates from a key with Shift pressed
    pub fn with_shift(key: Key) -> Self {
        Self {
            key,
            ctrl: false,
            alt: false,
            shift: true,
            meta: false,
        }
    }

    /// Creates from crossterm KeyEvent
    pub fn from_key_event(event: crossterm::event::KeyEvent) -> Option<Self> {
        use crossterm::event::KeyModifiers;

        Key::from_key_code(event.code).map(|key| Self {
            key,
            ctrl: event.modifiers.contains(KeyModifiers::CONTROL),
            alt: event.modifiers.contains(KeyModifiers::ALT),
            shift: event.modifiers.contains(KeyModifiers::SHIFT),
            meta: event.modifiers.contains(KeyModifiers::META),
        })
    }

    /// Checks if this is a platform-specific shortcut
    /// On macOS: uses Cmd (meta), on others: uses Ctrl
    pub fn is_primary_modifier(&self) -> bool {
        if cfg!(target_os = "macos") {
            self.meta
        } else {
            self.ctrl
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Represents a keyboard key that can be handled by UI models.
///
/// This enum provides type-safe representation of keyboard input,
/// distinguishing between regular characters and special keys.
///
/// ## Example
///
/// ```text
/// Elements::div()
///     .on_key(Key::Char('q'), move || app.quit())
///     .on_key(Key::Esc, move || cancel())
///     .on_key(Key::Enter, move || submit())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// Regular character key
    Char(char),

    /// Escape key
    Esc,

    /// Enter/Return key
    Enter,

    /// Tab key
    Tab,

    /// Back Tab key (Shift+Tab)
    BackTab,

    /// Backspace key
    Backspace,

    /// Delete key
    Delete,

    /// Arrow keys
    Up,
    Down,
    Left,
    Right,

    /// Page navigation
    PageUp,
    PageDown,
    Home,
    End,

    /// Function keys
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
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Key {
    /// Converts a crossterm KeyCode to a Key enum variant.
    ///
    /// Returns None if the key code doesn't map to a supported key.
    pub fn from_key_code(code: crossterm::event::KeyCode) -> Option<Self> {
        use crossterm::event::KeyCode;

        match code {
            KeyCode::Char(c) => Some(Key::Char(c)),
            KeyCode::Esc => Some(Key::Esc),
            KeyCode::Enter => Some(Key::Enter),
            KeyCode::Tab => Some(Key::Tab),
            KeyCode::BackTab => Some(Key::BackTab),
            KeyCode::Backspace => Some(Key::Backspace),
            KeyCode::Delete => Some(Key::Delete),
            KeyCode::Up => Some(Key::Up),
            KeyCode::Down => Some(Key::Down),
            KeyCode::Left => Some(Key::Left),
            KeyCode::Right => Some(Key::Right),
            KeyCode::PageUp => Some(Key::PageUp),
            KeyCode::PageDown => Some(Key::PageDown),
            KeyCode::Home => Some(Key::Home),
            KeyCode::End => Some(Key::End),
            KeyCode::F(1) => Some(Key::F1),
            KeyCode::F(2) => Some(Key::F2),
            KeyCode::F(3) => Some(Key::F3),
            KeyCode::F(4) => Some(Key::F4),
            KeyCode::F(5) => Some(Key::F5),
            KeyCode::F(6) => Some(Key::F6),
            KeyCode::F(7) => Some(Key::F7),
            KeyCode::F(8) => Some(Key::F8),
            KeyCode::F(9) => Some(Key::F9),
            KeyCode::F(10) => Some(Key::F10),
            KeyCode::F(11) => Some(Key::F11),
            KeyCode::F(12) => Some(Key::F12),
            _ => None,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Char(c) => write!(f, "{c}"),
            Key::Esc => write!(f, "Esc"),
            Key::Enter => write!(f, "Enter"),
            Key::Tab => write!(f, "Tab"),
            Key::BackTab => write!(f, "BackTab"),
            Key::Backspace => write!(f, "Backspace"),
            Key::Delete => write!(f, "Delete"),
            Key::Up => write!(f, "↑"),
            Key::Down => write!(f, "↓"),
            Key::Left => write!(f, "←"),
            Key::Right => write!(f, "→"),
            Key::PageUp => write!(f, "PgUp"),
            Key::PageDown => write!(f, "PgDn"),
            Key::Home => write!(f, "Home"),
            Key::End => write!(f, "End"),
            Key::F1 => write!(f, "F1"),
            Key::F2 => write!(f, "F2"),
            Key::F3 => write!(f, "F3"),
            Key::F4 => write!(f, "F4"),
            Key::F5 => write!(f, "F5"),
            Key::F6 => write!(f, "F6"),
            Key::F7 => write!(f, "F7"),
            Key::F8 => write!(f, "F8"),
            Key::F9 => write!(f, "F9"),
            Key::F10 => write!(f, "F10"),
            Key::F11 => write!(f, "F11"),
            Key::F12 => write!(f, "F12"),
        }
    }
}
