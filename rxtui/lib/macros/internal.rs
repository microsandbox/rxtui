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
