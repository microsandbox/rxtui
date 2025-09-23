//! Utility functions for terminal rendering and text manipulation.
//!
//! This module provides helper functions for various terminal rendering tasks,
//! including calculating the display width of Unicode strings and characters,
//! and text wrapping algorithms for fitting text within width constraints.

use crate::style::TextWrap;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

//--------------------------------------------------------------------------------------------------
// Macros: Debug Logging
//--------------------------------------------------------------------------------------------------

/// Debug logging macro that only compiles in debug builds.
/// Writes timestamped messages to /tmp/radical_debug.log
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        {
            use std::fs::OpenOptions;
            use std::io::Write;
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open("/tmp/radical_debug.log")
            {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis();
                let _ = writeln!(file, "[{}] {}", timestamp, format!($($arg)*));
            }
        }
    };
}

/// No-op version for release builds
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {};
}

//--------------------------------------------------------------------------------------------------
// Functions: Display Width
//--------------------------------------------------------------------------------------------------

/// Returns the display width of a string in terminal columns.
///
/// This accounts for:
/// - Wide characters (CJK, emojis) that take 2 columns
/// - Zero-width characters (combining marks) that take 0 columns
/// - Control characters are handled as per unicode-width rules
pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Returns the display width of a character in terminal columns.
///
/// Returns:
/// - 0 for zero-width characters and control characters
/// - 1 for regular characters
/// - 2 for wide characters (CJK, full-width, emojis)
pub fn char_width(c: char) -> usize {
    UnicodeWidthChar::width(c).unwrap_or(0)
}

/// Extracts a substring based on display column positions.
///
/// Returns a substring that starts at `start_col` and ends at `end_col` display columns.
/// Handles multibyte UTF-8 characters correctly. If a wide character spans the boundary,
/// it is excluded to maintain valid UTF-8.
pub fn substring_by_columns(s: &str, start_col: usize, end_col: usize) -> &str {
    if start_col >= end_col {
        return "";
    }

    let mut current_col = 0;
    let mut start_byte = None;
    let mut end_byte = s.len();

    for (byte_idx, ch) in s.char_indices() {
        let ch_width = char_width(ch);

        // Find start byte index
        if start_byte.is_none() {
            if current_col >= start_col {
                start_byte = Some(byte_idx);
            } else if current_col + ch_width > start_col {
                // Wide character spans the start boundary, start after it
                start_byte = Some(byte_idx + ch.len_utf8());
            }
        }

        // Find end byte index
        if current_col >= end_col {
            end_byte = byte_idx;
            break;
        } else if current_col + ch_width > end_col {
            // Wide character spans the end boundary, end before it
            end_byte = byte_idx;
            break;
        }

        current_col += ch_width;
    }

    let start = start_byte.unwrap_or(s.len());
    if start >= end_byte {
        ""
    } else {
        &s[start..end_byte]
    }
}

//--------------------------------------------------------------------------------------------------
// Functions: Text Wrapping
//--------------------------------------------------------------------------------------------------

/// Wraps text according to the specified mode and width constraint.
///
/// Returns a vector of lines that fit within the given width.
/// Empty lines are preserved in the output.
pub fn wrap_text(text: &str, width: u16, mode: TextWrap) -> Vec<String> {
    if width == 0 {
        return vec![];
    }

    match mode {
        TextWrap::None => {
            // No wrapping - return original text as single line
            vec![text.to_string()]
        }
        TextWrap::Character => {
            // Break at any character boundary
            wrap_character(text, width)
        }
        TextWrap::Word => {
            // Break at word boundaries only
            wrap_word(text, width)
        }
        TextWrap::WordBreak => {
            // Try word boundaries first, break words if necessary
            wrap_word_break(text, width)
        }
    }
}

/// Wraps text at character boundaries.
///
/// Breaks the text based on display width, accounting for wide characters.
fn wrap_character(text: &str, width: u16) -> Vec<String> {
    let width = width as usize;
    let mut lines = Vec::new();

    if text.is_empty() {
        return vec![String::new()];
    }

    let mut current_line = String::new();
    let mut current_width = 0;

    for ch in text.chars() {
        let ch_width = char_width(ch);

        if current_width + ch_width > width && !current_line.is_empty() {
            // Start a new line
            lines.push(current_line);
            current_line = String::new();
            current_width = 0;
        }

        // Add character if it fits (or if line is empty to avoid infinite loop)
        if current_width + ch_width <= width || current_line.is_empty() {
            current_line.push(ch);
            current_width += ch_width;
        } else {
            // Character doesn't fit even on empty line (width too small for wide char)
            // Start next line with this character
            lines.push(current_line);
            current_line = ch.to_string();
            current_width = ch_width;
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

/// Wraps text at word boundaries.
///
/// Attempts to break lines at spaces and other word boundaries.
/// If a word is longer than the line width, it will overflow.
/// Preserves all spaces (leading, trailing, and in-between).
fn wrap_word(text: &str, width: u16) -> Vec<String> {
    let width = width as usize;
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    // Process character by character to preserve all spaces
    let mut in_word = false;
    let mut word = String::new();
    let mut word_width = 0;
    let mut pending_spaces = String::new();
    let mut pending_spaces_width = 0;

    for ch in text.chars() {
        if ch.is_whitespace() {
            // Handle any accumulated word first
            if in_word {
                // Check if word fits on current line
                if current_width == 0 {
                    // First content on line
                    current_line.push_str(&word);
                    current_width = word_width;
                } else if current_width + word_width <= width {
                    // Word fits on current line
                    current_line.push_str(&word);
                    current_width += word_width;
                } else {
                    // Word doesn't fit, start new line
                    lines.push(current_line.clone());
                    current_line = word.clone();
                    current_width = word_width;
                }
                word.clear();
                word_width = 0;
                in_word = false;
            }

            // Now accumulate the space
            pending_spaces.push(ch);
            pending_spaces_width += char_width(ch);
        } else {
            // Non-whitespace character

            // If we have pending spaces, handle them first
            if !pending_spaces.is_empty() {
                // Check if spaces fit on current line
                if current_width + pending_spaces_width <= width {
                    current_line.push_str(&pending_spaces);
                    current_width += pending_spaces_width;
                } else {
                    // Spaces don't fit, wrap to new line
                    // But only if we have content on the current line
                    if current_width > 0 {
                        lines.push(current_line.clone());
                        // Trim first leading space when starting new line with spaces
                        let mut new_line = pending_spaces.clone();
                        let mut new_width = pending_spaces_width;
                        if let Some(first_char) = new_line.chars().next()
                            && first_char == ' '
                        {
                            // Remove first space only
                            new_line = new_line.chars().skip(1).collect();
                            new_width = new_width.saturating_sub(char_width(' '));
                        }
                        current_line = new_line;
                        current_width = new_width;
                    } else {
                        // Line is empty, add the spaces anyway
                        current_line.push_str(&pending_spaces);
                        current_width = pending_spaces_width;
                    }
                }
                pending_spaces.clear();
                pending_spaces_width = 0;
            }

            // Start or continue building a word
            in_word = true;
            word.push(ch);
            word_width += char_width(ch);
        }
    }

    // Handle any remaining word
    if in_word {
        if current_width == 0 || current_width + word_width <= width {
            current_line.push_str(&word);
        } else {
            lines.push(current_line.clone());
            current_line = word;
        }
    }

    // Handle any trailing spaces
    if !pending_spaces.is_empty() {
        if current_width + pending_spaces_width <= width {
            current_line.push_str(&pending_spaces);
        } else if current_width > 0 {
            lines.push(current_line.clone());
            // Trim first leading space when starting new line with spaces
            let mut new_line = pending_spaces;
            if let Some(first_char) = new_line.chars().next()
                && first_char == ' '
            {
                // Remove first space only
                new_line = new_line.chars().skip(1).collect();
            }
            current_line = new_line;
        } else {
            current_line.push_str(&pending_spaces);
        }
    }

    // Add the last line
    if !current_line.is_empty() || lines.is_empty() {
        lines.push(current_line);
    }

    lines
}

/// Wraps text at word boundaries, breaking words if necessary.
///
/// First attempts to break at word boundaries. If a word is longer than
/// the line width, it breaks the word at character boundaries considering display width.
fn wrap_word_break(text: &str, width: u16) -> Vec<String> {
    let width = width as usize;
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    // Process text character by character to preserve spaces
    let chars = text.chars();
    let mut in_word = false;
    let mut word = String::new();
    let mut word_width = 0;

    for ch in chars {
        if ch.is_whitespace() {
            // Handle any accumulated word first
            if in_word {
                // Try to add the word to current line
                if current_width == 0 {
                    // First word on line
                    if word_width <= width {
                        current_line.push_str(&word);
                        current_width = word_width;
                    } else {
                        // Word too long, break it
                        for word_ch in word.chars() {
                            let ch_width = char_width(word_ch);
                            if current_width + ch_width > width && current_width > 0 {
                                lines.push(current_line.clone());
                                current_line.clear();
                                current_width = 0;
                            }
                            current_line.push(word_ch);
                            current_width += ch_width;
                        }
                    }
                } else if current_width + word_width <= width {
                    // Word fits on current line
                    current_line.push_str(&word);
                    current_width += word_width;
                } else {
                    // Word doesn't fit, start new line
                    lines.push(current_line.clone());
                    current_line.clear();
                    current_width = 0;

                    // Add word to new line (possibly breaking it)
                    if word_width <= width {
                        current_line.push_str(&word);
                        current_width = word_width;
                    } else {
                        // Break the word
                        for word_ch in word.chars() {
                            let ch_width = char_width(word_ch);
                            if current_width + ch_width > width && current_width > 0 {
                                lines.push(current_line.clone());
                                current_line.clear();
                                current_width = 0;
                            }
                            current_line.push(word_ch);
                            current_width += ch_width;
                        }
                    }
                }

                word.clear();
                word_width = 0;
                in_word = false;
            }

            // Now handle the whitespace character
            let ch_width = char_width(ch);
            if current_width + ch_width > width && current_width > 0 {
                // Whitespace would exceed width, start new line
                lines.push(current_line.clone());
                current_line.clear();
                // Skip first space when starting new line, preserve other whitespace
                if ch == ' ' {
                    // Skip the first space that would lead the new line
                    current_width = 0;
                } else {
                    // Preserve tabs and other whitespace
                    current_line.push(ch);
                    current_width = ch_width;
                }
            } else {
                // Add the whitespace character
                current_line.push(ch);
                current_width += ch_width;
            }
        } else {
            // Non-whitespace character - accumulate in word
            in_word = true;
            word.push(ch);
            word_width += char_width(ch);
        }
    }

    // Handle any remaining word
    if in_word {
        if current_width == 0 {
            // First word on line
            if word_width <= width {
                current_line.push_str(&word);
            } else {
                // Word too long, break it
                for word_ch in word.chars() {
                    let ch_width = char_width(word_ch);
                    if current_width + ch_width > width && current_width > 0 {
                        lines.push(current_line.clone());
                        current_line.clear();
                        current_width = 0;
                    }
                    current_line.push(word_ch);
                    current_width += ch_width;
                }
            }
        } else if current_width + word_width <= width {
            // Word fits on current line
            current_line.push_str(&word);
        } else {
            // Word doesn't fit, start new line
            lines.push(current_line.clone());
            current_line.clear();

            // Add word to new line (possibly breaking it)
            if word_width <= width {
                current_line = word;
            } else {
                // Break the word
                current_width = 0;
                for word_ch in word.chars() {
                    let ch_width = char_width(word_ch);
                    if current_width + ch_width > width && current_width > 0 {
                        lines.push(current_line.clone());
                        current_line.clear();
                        current_width = 0;
                    }
                    current_line.push(word_ch);
                    current_width += ch_width;
                }
            }
        }
    }

    // Add the last line if not empty
    if !current_line.is_empty() {
        lines.push(current_line);
    } else if lines.is_empty() {
        // Handle empty text
        lines.push(String::new());
    }

    lines
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    //----------------------------------------------------------------------------------------------
    // Tests: Display Width Functions
    //----------------------------------------------------------------------------------------------

    #[test]
    fn test_display_width_ascii() {
        assert_eq!(display_width("Hello"), 5);
        assert_eq!(display_width(""), 0);
        assert_eq!(display_width("Test 123"), 8);
    }

    #[test]
    fn test_display_width_unicode() {
        // CJK characters (2 width each)
        assert_eq!(display_width("世界"), 4);
        assert_eq!(display_width("Hello 世界"), 10);

        // Emoji (typically 2 width)
        assert_eq!(display_width("😀"), 2);
        assert_eq!(display_width("Test 😀"), 7);
    }

    #[test]
    fn test_char_width() {
        assert_eq!(char_width('A'), 1);
        assert_eq!(char_width('世'), 2);
        assert_eq!(char_width('😀'), 2);
        assert_eq!(char_width('\0'), 0); // Control character
    }

    #[test]
    fn test_substring_by_columns() {
        // ASCII tests
        assert_eq!(substring_by_columns("Hello World", 0, 5), "Hello");
        assert_eq!(substring_by_columns("Hello World", 6, 11), "World");
        assert_eq!(substring_by_columns("Hello World", 3, 8), "lo Wo");

        // Wide character tests
        assert_eq!(substring_by_columns("Hello 世界", 0, 6), "Hello ");
        assert_eq!(substring_by_columns("Hello 世界", 6, 10), "世界");
        assert_eq!(substring_by_columns("Hello 世界", 0, 8), "Hello 世");
        assert_eq!(substring_by_columns("Hello 世界", 7, 10), "界"); // Start in middle of 世
        assert_eq!(substring_by_columns("Hello 世界", 0, 7), "Hello "); // End in middle of 世

        // Emoji tests
        assert_eq!(substring_by_columns("Test😀End", 0, 4), "Test");
        assert_eq!(substring_by_columns("Test😀End", 4, 6), "😀");
        assert_eq!(substring_by_columns("Test😀End", 6, 9), "End");
        assert_eq!(substring_by_columns("Test😀End", 0, 5), "Test"); // End in middle of emoji
        assert_eq!(substring_by_columns("Test😀End", 5, 9), "End"); // Start in middle of emoji

        // Edge cases
        assert_eq!(substring_by_columns("", 0, 5), "");
        assert_eq!(substring_by_columns("Hello", 5, 5), "");
        assert_eq!(substring_by_columns("Hello", 10, 20), "");
    }

    //----------------------------------------------------------------------------------------------
    // Tests: Text Wrapping Functions
    //----------------------------------------------------------------------------------------------

    #[test]
    fn test_wrap_none() {
        let text = "This is a very long line that should not be wrapped";
        let wrapped = wrap_text(text, 10, TextWrap::None);
        assert_eq!(wrapped, vec![text]);
    }

    #[test]
    fn test_wrap_character() {
        let text = "Hello World";
        let wrapped = wrap_text(text, 5, TextWrap::Character);
        assert_eq!(wrapped, vec!["Hello", " Worl", "d"]);
    }

    #[test]
    fn test_wrap_character_exact() {
        let text = "12345678901234567890";
        let wrapped = wrap_text(text, 10, TextWrap::Character);
        assert_eq!(wrapped, vec!["1234567890", "1234567890"]);
    }

    #[test]
    fn test_wrap_word() {
        let text = "The quick brown fox jumps";
        let wrapped = wrap_text(text, 10, TextWrap::Word);
        // Trailing spaces are preserved, only first leading space is trimmed
        assert_eq!(wrapped, vec!["The quick ", "brown fox ", "jumps"]);
    }

    #[test]
    fn test_wrap_word_long_word() {
        let text = "A verylongword that exceeds width";
        let wrapped = wrap_text(text, 10, TextWrap::Word);
        // Long word overflows in Word mode, first leading space trimmed
        assert_eq!(
            wrapped,
            vec!["A ", "verylongword", "that ", "exceeds ", "width"]
        );
    }

    #[test]
    fn test_wrap_word_break() {
        let text = "A verylongword that exceeds";
        let wrapped = wrap_text(text, 10, TextWrap::WordBreak);
        // Trailing spaces preserved, first leading space trimmed
        assert_eq!(wrapped, vec!["A ", "verylongwo", "rd that ", "exceeds"]);
    }

    #[test]
    fn test_wrap_word_break_preserves_leading_spaces() {
        let text = "        _ => calculate";
        let wrapped = wrap_text(text, 15, TextWrap::WordBreak);
        assert_eq!(wrapped, vec!["        _ => ", "calculate"]);
    }

    #[test]
    fn test_wrap_empty_text() {
        assert_eq!(wrap_text("", 10, TextWrap::Character), vec![""]);
        assert_eq!(wrap_text("", 10, TextWrap::Word), vec![""]);
        assert_eq!(wrap_text("", 10, TextWrap::WordBreak), vec![""]);
    }

    #[test]
    fn test_wrap_zero_width() {
        let text = "Hello";
        assert_eq!(
            wrap_text(text, 0, TextWrap::Character),
            Vec::<String>::new()
        );
        assert_eq!(wrap_text(text, 0, TextWrap::Word), Vec::<String>::new());
    }

    #[test]
    fn test_wrap_unicode() {
        // Note: 世 and 界 are each 2 display width
        let text = "Hello 世界";
        let wrapped = wrap_text(text, 8, TextWrap::Character);
        assert_eq!(wrapped, vec!["Hello 世", "界"]); // "Hello " = 6, "世" = 2

        let wrapped = wrap_text(text, 7, TextWrap::Character);
        assert_eq!(wrapped, vec!["Hello ", "世界"]); // Can't fit 世 with Hello in 7 width

        // Test word wrapping with Unicode
        let text = "Hello 世界 World";
        let wrapped = wrap_text(text, 10, TextWrap::Word);
        assert_eq!(wrapped, vec!["Hello 世界", "World"]); // "Hello " = 6, "世界" = 4, first space trimmed on next line
    }

    #[test]
    fn test_wrap_emoji() {
        // Emoji typically have width 2
        let text = "Test 😀 emoji";
        let wrapped = wrap_text(text, 8, TextWrap::Character);
        assert_eq!(wrapped, vec!["Test 😀 ", "emoji"]); // "Test " = 5, "😀" = 2, " " = 1

        let wrapped = wrap_text(text, 7, TextWrap::Word);
        assert_eq!(wrapped, vec!["Test 😀", "emoji"]); // "Test " = 5, "😀" = 2, first space trimmed on next line
    }

    #[test]
    fn test_wrap_word_multiple_spaces() {
        let text = "Hello     World   Test";
        let wrapped = wrap_text(text, 10, TextWrap::Word);
        // First space is trimmed, rest are preserved, trailing spaces kept
        assert_eq!(wrapped, vec!["Hello     ", "World   ", "Test"]);
    }

    #[test]
    fn test_wrap_preserves_leading_spaces() {
        let text = "    Hello World";
        let wrapped = wrap_text(text, 10, TextWrap::Word);
        assert_eq!(wrapped, vec!["    Hello ", "World"]);
    }

    #[test]
    fn test_wrap_preserves_trailing_spaces() {
        let text = "Hello World    ";
        let wrapped = wrap_text(text, 10, TextWrap::Word);
        assert_eq!(wrapped, vec!["Hello ", "World    "]);
    }
}
