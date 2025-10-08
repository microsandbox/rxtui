//! Internal macros used by the node! macro
//! These are not part of the public API

/// Converts color values to the Color type
/// Supports: named colors, hex strings, RGB values, and expressions
#[doc(hidden)]
#[macro_export]
macro_rules! color_value {
    // Named colors (without Color:: prefix)
    (black) => {
        $crate::Color::Black
    };
    (red) => {
        $crate::Color::Red
    };
    (green) => {
        $crate::Color::Green
    };
    (yellow) => {
        $crate::Color::Yellow
    };
    (blue) => {
        $crate::Color::Blue
    };
    (magenta) => {
        $crate::Color::Magenta
    };
    (cyan) => {
        $crate::Color::Cyan
    };
    (white) => {
        $crate::Color::White
    };
    (bright_black) => {
        $crate::Color::BrightBlack
    };
    (bright_red) => {
        $crate::Color::BrightRed
    };
    (bright_green) => {
        $crate::Color::BrightGreen
    };
    (bright_yellow) => {
        $crate::Color::BrightYellow
    };
    (bright_blue) => {
        $crate::Color::BrightBlue
    };
    (bright_magenta) => {
        $crate::Color::BrightMagenta
    };
    (bright_cyan) => {
        $crate::Color::BrightCyan
    };
    (bright_white) => {
        $crate::Color::BrightWhite
    };

    // Hex color strings
    ($hex:literal) => {
        $crate::Color::hex($hex)
    };

    // Any other expression - pass through
    ($color:expr) => {
        $color
    };
}

/// Converts direction shortcuts to Direction values
#[doc(hidden)]
#[macro_export]
macro_rules! direction_value {
    (vertical) => {
        $crate::Direction::Vertical
    };
    (v) => {
        $crate::Direction::Vertical
    };
    (horizontal) => {
        $crate::Direction::Horizontal
    };
    (h) => {
        $crate::Direction::Horizontal
    };
    ($dir:expr) => {
        $dir
    };
}

/// Converts edge names to BorderEdges values
#[doc(hidden)]
#[macro_export]
macro_rules! tui_edge_value {
    (top) => {
        $crate::BorderEdges::TOP
    };
    (bottom) => {
        $crate::BorderEdges::BOTTOM
    };
    (left) => {
        $crate::BorderEdges::LEFT
    };
    (right) => {
        $crate::BorderEdges::RIGHT
    };
    (horizontal) => {
        $crate::BorderEdges::HORIZONTAL
    };
    (vertical) => {
        $crate::BorderEdges::VERTICAL
    };
    (all) => {
        $crate::BorderEdges::ALL
    };
    (corners) => {
        $crate::BorderEdges::CORNERS
    };
    (edges) => {
        $crate::BorderEdges::EDGES
    };
    (top_left) => {
        $crate::BorderEdges::TOP_LEFT
    };
    (top_right) => {
        $crate::BorderEdges::TOP_RIGHT
    };
    (bottom_left) => {
        $crate::BorderEdges::BOTTOM_LEFT
    };
    (bottom_right) => {
        $crate::BorderEdges::BOTTOM_RIGHT
    };
    ($edge:expr) => {
        $edge
    };
}

/// Converts overflow values to Overflow enum
#[doc(hidden)]
#[macro_export]
macro_rules! overflow_value {
    (none) => {
        $crate::Overflow::None
    };
    (hidden) => {
        $crate::Overflow::Hidden
    };
    (scroll) => {
        $crate::Overflow::Scroll
    };
    (auto) => {
        $crate::Overflow::Auto
    };
    ($overflow:expr) => {
        $overflow
    };
}

/// Converts wrap mode values to WrapMode enum
#[doc(hidden)]
#[macro_export]
macro_rules! wrap_value {
    (nowrap) => {
        $crate::WrapMode::NoWrap
    };
    (no_wrap) => {
        $crate::WrapMode::NoWrap
    };
    (wrap) => {
        $crate::WrapMode::Wrap
    };
    (wrap_reverse) => {
        $crate::WrapMode::WrapReverse
    };
    ($wrap:expr) => {
        $wrap
    };
}

/// Converts text wrap values to TextWrap enum
#[doc(hidden)]
#[macro_export]
macro_rules! text_wrap_value {
    (none) => {
        $crate::TextWrap::None
    };
    (character) => {
        $crate::TextWrap::Character
    };
    (char) => {
        $crate::TextWrap::Character
    };
    (word) => {
        $crate::TextWrap::Word
    };
    (word_break) => {
        $crate::TextWrap::WordBreak
    };
    ($wrap:expr) => {
        $wrap
    };
}

/// Converts text align values to TextAlign enum
#[doc(hidden)]
#[macro_export]
macro_rules! text_align_value {
    (left) => {
        $crate::style::TextAlign::Left
    };
    (center) => {
        $crate::style::TextAlign::Center
    };
    (right) => {
        $crate::style::TextAlign::Right
    };
    ($align:expr) => {
        $align
    };
}

/// Converts position values to Position enum
#[doc(hidden)]
#[macro_export]
macro_rules! position_value {
    (relative) => {
        $crate::Position::Relative
    };
    (absolute) => {
        $crate::Position::Absolute
    };
    (fixed) => {
        $crate::Position::Fixed
    };
    ($pos:expr) => {
        $pos
    };
}

/// Converts justify content values to JustifyContent enum
#[doc(hidden)]
#[macro_export]
macro_rules! justify_content_value {
    (start) => {
        $crate::style::JustifyContent::Start
    };
    (center) => {
        $crate::style::JustifyContent::Center
    };
    (end) => {
        $crate::style::JustifyContent::End
    };
    (space_between) => {
        $crate::style::JustifyContent::SpaceBetween
    };
    (space_around) => {
        $crate::style::JustifyContent::SpaceAround
    };
    (space_evenly) => {
        $crate::style::JustifyContent::SpaceEvenly
    };
    ($justify:expr) => {
        $justify
    };
}

/// Converts align items values to AlignItems enum
#[doc(hidden)]
#[macro_export]
macro_rules! align_items_value {
    (start) => {
        $crate::style::AlignItems::Start
    };
    (center) => {
        $crate::style::AlignItems::Center
    };
    (end) => {
        $crate::style::AlignItems::End
    };
    ($align:expr) => {
        $align
    };
}

/// Converts align self values to AlignSelf enum
#[doc(hidden)]
#[macro_export]
macro_rules! align_self_value {
    (auto) => {
        $crate::style::AlignSelf::Auto
    };
    (start) => {
        $crate::style::AlignSelf::Start
    };
    (center) => {
        $crate::style::AlignSelf::Center
    };
    (end) => {
        $crate::style::AlignSelf::End
    };
    ($align:expr) => {
        $align
    };
}

/// Converts key values to Key enum
/// Supports lowercase/snake_case names for ergonomics
#[doc(hidden)]
#[macro_export]
macro_rules! key_value {
    // Special keys (lowercase)
    (esc) => {
        $crate::Key::Esc
    };
    (enter) => {
        $crate::Key::Enter
    };
    (tab) => {
        $crate::Key::Tab
    };
    (backtab) => {
        $crate::Key::BackTab
    };
    (back_tab) => {
        $crate::Key::BackTab
    };
    (backspace) => {
        $crate::Key::Backspace
    };
    (delete) => {
        $crate::Key::Delete
    };

    // Arrow keys (lowercase)
    (up) => {
        $crate::Key::Up
    };
    (down) => {
        $crate::Key::Down
    };
    (left) => {
        $crate::Key::Left
    };
    (right) => {
        $crate::Key::Right
    };

    // Navigation keys (snake_case)
    (page_up) => {
        $crate::Key::PageUp
    };
    (pageup) => {
        $crate::Key::PageUp
    };
    (page_down) => {
        $crate::Key::PageDown
    };
    (pagedown) => {
        $crate::Key::PageDown
    };
    (home) => {
        $crate::Key::Home
    };
    (end) => {
        $crate::Key::End
    };

    // Function keys (lowercase)
    (f1) => {
        $crate::Key::F1
    };
    (f2) => {
        $crate::Key::F2
    };
    (f3) => {
        $crate::Key::F3
    };
    (f4) => {
        $crate::Key::F4
    };
    (f5) => {
        $crate::Key::F5
    };
    (f6) => {
        $crate::Key::F6
    };
    (f7) => {
        $crate::Key::F7
    };
    (f8) => {
        $crate::Key::F8
    };
    (f9) => {
        $crate::Key::F9
    };
    (f10) => {
        $crate::Key::F10
    };
    (f11) => {
        $crate::Key::F11
    };
    (f12) => {
        $crate::Key::F12
    };

    // Backwards compatibility - support CamelCase variants
    (Esc) => {
        $crate::Key::Esc
    };
    (Enter) => {
        $crate::Key::Enter
    };
    (Tab) => {
        $crate::Key::Tab
    };
    (BackTab) => {
        $crate::Key::BackTab
    };
    (Backspace) => {
        $crate::Key::Backspace
    };
    (Delete) => {
        $crate::Key::Delete
    };
    (Up) => {
        $crate::Key::Up
    };
    (Down) => {
        $crate::Key::Down
    };
    (Left) => {
        $crate::Key::Left
    };
    (Right) => {
        $crate::Key::Right
    };
    (PageUp) => {
        $crate::Key::PageUp
    };
    (PageDown) => {
        $crate::Key::PageDown
    };
    (Home) => {
        $crate::Key::Home
    };
    (End) => {
        $crate::Key::End
    };
    (F1) => {
        $crate::Key::F1
    };
    (F2) => {
        $crate::Key::F2
    };
    (F3) => {
        $crate::Key::F3
    };
    (F4) => {
        $crate::Key::F4
    };
    (F5) => {
        $crate::Key::F5
    };
    (F6) => {
        $crate::Key::F6
    };
    (F7) => {
        $crate::Key::F7
    };
    (F8) => {
        $crate::Key::F8
    };
    (F9) => {
        $crate::Key::F9
    };
    (F10) => {
        $crate::Key::F10
    };
    (F11) => {
        $crate::Key::F11
    };
    (F12) => {
        $crate::Key::F12
    };

    // Any other expression - pass through
    ($key:expr) => {
        $key
    };
}

/// Converts modifier combinations to KeyWithModifiers
#[doc(hidden)]
#[macro_export]
macro_rules! key_with_modifiers_value {
    (ctrl + $($rest:tt)+) => {{
        let mut km = $crate::key_with_modifiers_value!($($rest)+);
        km.ctrl = true;
        km
    }};
    (alt + $($rest:tt)+) => {{
        let mut km = $crate::key_with_modifiers_value!($($rest)+);
        km.alt = true;
        km
    }};
    (shift + $($rest:tt)+) => {{
        let mut km = $crate::key_with_modifiers_value!($($rest)+);
        km.shift = true;
        km
    }};
    (meta + $($rest:tt)+) => {{
        let mut km = $crate::key_with_modifiers_value!($($rest)+);
        km.meta = true;
        km
    }};
    (cmd + $($rest:tt)+) => {{
        let mut km = $crate::key_with_modifiers_value!($($rest)+);
        km.meta = true;
        km
    }};
    (key: $key:expr) => {{
        $crate::KeyWithModifiers::new($key)
    }};
    (Char($ch:literal)) => {{
        $crate::KeyWithModifiers::new($crate::Key::Char($ch))
    }};
    ($ch:literal) => {{
        $crate::KeyWithModifiers::new($crate::Key::Char($ch))
    }};
    ($key:ident) => {{
        $crate::KeyWithModifiers::new($crate::key_value!($key))
    }};
    ($key:expr) => {{
        $crate::KeyWithModifiers::new($key)
    }};
}
