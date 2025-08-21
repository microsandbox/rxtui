use crate::style::TextStyle;
use crate::{Color, TextWrap};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A span of text with optional styling
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TextSpan {
    pub content: String,
    pub style: Option<TextStyle>,
    /// Internal flag to preserve cursor during wrapping
    #[doc(hidden)]
    pub is_cursor: bool,
}

/// Rich text with multiple styled segments for inline styling
#[derive(Debug, Clone, PartialEq)]
pub struct RichText {
    pub spans: Vec<TextSpan>,
    pub style: Option<TextStyle>, // For top-level styling like wrapping
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl RichText {
    /// Creates a new empty RichText
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            style: None,
        }
    }

    /// Creates RichText with a cursor at the specified position
    /// The cursor style will be preserved even after text wrapping
    /// Used internally by TextInput component
    pub fn with_cursor(text: &str, cursor_pos: usize, cursor_style: TextStyle) -> Self {
        let mut spans = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let char_count = chars.len();

        // Add text before cursor
        if cursor_pos > 0 && cursor_pos <= char_count {
            let before: String = chars[..cursor_pos].iter().collect();
            spans.push(TextSpan {
                content: before,
                style: None,
                is_cursor: false,
            });
        }

        // Add cursor character or space at end
        if cursor_pos < char_count {
            // Cursor on a character
            spans.push(TextSpan {
                content: chars[cursor_pos].to_string(),
                style: Some(cursor_style.clone()),
                is_cursor: true, // Mark as cursor span
            });
            // Add text after cursor
            if cursor_pos + 1 < char_count {
                let after: String = chars[cursor_pos + 1..].iter().collect();
                spans.push(TextSpan {
                    content: after,
                    style: None,
                    is_cursor: false,
                });
            }
        } else {
            // Cursor at end - show space with cursor style
            spans.push(TextSpan {
                content: " ".to_string(),
                style: Some(cursor_style),
                is_cursor: true, // Mark as cursor span
            });
        }

        Self { spans, style: None }
    }

    /// Adds a plain text span
    pub fn text(mut self, content: impl Into<String>) -> Self {
        self.spans.push(TextSpan {
            content: content.into(),
            style: None,
            is_cursor: false,
        });
        self
    }

    /// Adds a colored text span
    pub fn colored(mut self, content: impl Into<String>, color: Color) -> Self {
        self.spans.push(TextSpan {
            content: content.into(),
            style: Some(TextStyle {
                color: Some(color),
                ..Default::default()
            }),
            is_cursor: false,
        });
        self
    }

    /// Adds a bold text span
    pub fn bold(mut self, content: impl Into<String>) -> Self {
        self.spans.push(TextSpan {
            content: content.into(),
            style: Some(TextStyle {
                bold: Some(true),
                ..Default::default()
            }),
            is_cursor: false,
        });
        self
    }

    /// Adds an italic text span
    pub fn italic(mut self, content: impl Into<String>) -> Self {
        self.spans.push(TextSpan {
            content: content.into(),
            style: Some(TextStyle {
                italic: Some(true),
                ..Default::default()
            }),
            is_cursor: false,
        });
        self
    }

    /// Adds a text span with custom style
    pub fn styled(mut self, content: impl Into<String>, style: TextStyle) -> Self {
        self.spans.push(TextSpan {
            content: content.into(),
            style: Some(style),
            is_cursor: false,
        });
        self
    }

    /// Sets the text wrapping mode
    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.style.get_or_insert(TextStyle::default()).wrap = Some(wrap);
        self
    }

    /// Sets the color for all spans that don't already have a color
    pub fn color(mut self, color: Color) -> Self {
        for span in &mut self.spans {
            let style = span.style.get_or_insert(TextStyle::default());
            if style.color.is_none() {
                style.color = Some(color);
            }
        }
        self
    }

    /// Sets the background color for all spans
    pub fn background(mut self, color: Color) -> Self {
        for span in &mut self.spans {
            let style = span.style.get_or_insert(TextStyle::default());
            if style.background.is_none() {
                style.background = Some(color);
            }
        }
        self
    }

    /// Makes all spans bold
    pub fn bold_all(mut self) -> Self {
        for span in &mut self.spans {
            let style = span.style.get_or_insert(TextStyle::default());
            if style.bold.is_none() {
                style.bold = Some(true);
            }
        }
        self
    }

    /// Makes all spans italic
    pub fn italic_all(mut self) -> Self {
        for span in &mut self.spans {
            let style = span.style.get_or_insert(TextStyle::default());
            if style.italic.is_none() {
                style.italic = Some(true);
            }
        }
        self
    }

    /// Makes all spans underlined
    pub fn underline_all(mut self) -> Self {
        for span in &mut self.spans {
            let style = span.style.get_or_insert(TextStyle::default());
            if style.underline.is_none() {
                style.underline = Some(true);
            }
        }
        self
    }

    /// Returns the concatenated content of all spans
    pub fn content(&self) -> String {
        self.spans
            .iter()
            .map(|span| span.content.as_str())
            .collect()
    }

    /// Returns true if there are no spans or all spans are empty
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty() || self.spans.iter().all(|span| span.content.is_empty())
    }

    /// Clears all spans
    pub fn clear(&mut self) {
        self.spans.clear();
    }

    /// Appends another RichText's spans to this one
    pub fn append(&mut self, other: &mut RichText) {
        self.spans.append(&mut other.spans);
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for RichText {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for RichText {
    fn from(s: String) -> Self {
        Self::new().text(s)
    }
}

impl From<&str> for RichText {
    fn from(s: &str) -> Self {
        Self::new().text(s)
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rich_text_creation() {
        let rich = RichText::new()
            .text("Hello ")
            .colored("world", Color::Red)
            .text("!");

        assert_eq!(rich.spans.len(), 3);
        assert_eq!(rich.spans[0].content, "Hello ");
        assert_eq!(rich.spans[1].content, "world");
        assert_eq!(rich.spans[2].content, "!");
        assert_eq!(
            rich.spans[1].style.as_ref().unwrap().color,
            Some(Color::Red)
        );
    }

    #[test]
    fn test_rich_text_bold_italic() {
        let rich = RichText::new()
            .text("Normal ")
            .bold("Bold")
            .text(" ")
            .italic("Italic");

        assert_eq!(rich.spans.len(), 4);
        assert_eq!(rich.spans[1].style.as_ref().unwrap().bold, Some(true));
        assert_eq!(rich.spans[3].style.as_ref().unwrap().italic, Some(true));
    }

    #[test]
    fn test_rich_text_with_cursor() {
        // Cursor in middle
        let rich = RichText::with_cursor(
            "Hello",
            2,
            TextStyle {
                background: Some(Color::Blue),
                ..Default::default()
            },
        );

        assert_eq!(rich.spans.len(), 3);
        assert_eq!(rich.spans[0].content, "He");
        assert_eq!(rich.spans[1].content, "l");
        assert_eq!(rich.spans[2].content, "lo");
        assert_eq!(
            rich.spans[1].style.as_ref().unwrap().background,
            Some(Color::Blue)
        );

        // Cursor at end
        let rich_end = RichText::with_cursor(
            "Hi",
            2,
            TextStyle {
                background: Some(Color::Green),
                ..Default::default()
            },
        );

        assert_eq!(rich_end.spans.len(), 2);
        assert_eq!(rich_end.spans[0].content, "Hi");
        assert_eq!(rich_end.spans[1].content, " ");
        assert_eq!(
            rich_end.spans[1].style.as_ref().unwrap().background,
            Some(Color::Green)
        );
    }

    #[test]
    fn test_top_level_styling_methods() {
        let rich = RichText::new()
            .text("First")
            .text(" ")
            .text("Second")
            .color(Color::Yellow)
            .background(Color::Black);

        // All spans should have yellow text on black background
        for span in &rich.spans {
            assert_eq!(span.style.as_ref().unwrap().color, Some(Color::Yellow));
            assert_eq!(span.style.as_ref().unwrap().background, Some(Color::Black));
        }
    }

    #[test]
    fn test_rich_text_bold_all() {
        let rich = RichText::new()
            .text("One")
            .colored("Two", Color::Red)
            .text("Three")
            .bold_all();

        // All spans should be bold
        for span in &rich.spans {
            assert_eq!(span.style.as_ref().unwrap().bold, Some(true));
        }
        // Second span should retain its color
        assert_eq!(
            rich.spans[1].style.as_ref().unwrap().color,
            Some(Color::Red)
        );
    }

    #[test]
    fn test_rich_text_wrap() {
        let rich = RichText::new()
            .text("This is wrapped text")
            .wrap(TextWrap::Word);

        assert!(rich.style.is_some());
        assert_eq!(rich.style.as_ref().unwrap().wrap, Some(TextWrap::Word));
    }

    #[test]
    fn test_rich_text_helper_methods() {
        let mut rich = RichText::new().text("Hello").text(" ").text("World");

        // Test content()
        assert_eq!(rich.content(), "Hello World");

        // Test is_empty()
        assert!(!rich.is_empty());

        // Test clear()
        rich.clear();
        assert!(rich.is_empty());
        assert_eq!(rich.content(), "");

        // Test append()
        let mut rich1 = RichText::new().text("First");
        let mut rich2 = RichText::new().colored("Second", Color::Blue);
        rich1.append(&mut rich2);
        assert_eq!(rich1.spans.len(), 2);
        assert_eq!(rich1.content(), "FirstSecond");
        assert!(rich2.is_empty());
    }

    #[test]
    fn test_rich_text_from_traits() {
        // From String
        let from_string: RichText = String::from("test string").into();
        assert_eq!(from_string.spans.len(), 1);
        assert_eq!(from_string.content(), "test string");

        // From &str
        let from_str: RichText = "test str".into();
        assert_eq!(from_str.spans.len(), 1);
        assert_eq!(from_str.content(), "test str");
    }

    #[test]
    fn test_rich_text_default() {
        let rich = RichText::default();
        assert!(rich.is_empty());
        assert_eq!(rich.content(), "");
    }
}
