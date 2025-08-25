use crate::style::{TextAlign, TextStyle};
use crate::{Color, TextWrap};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Text content with styling
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Text {
    pub content: String,
    pub style: Option<TextStyle>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Text {
    /// Creates a new Text with the given content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            style: None,
        }
    }

    /// Sets the text color
    pub fn color(mut self, color: Color) -> Self {
        self.style.get_or_insert(TextStyle::default()).color = Some(color);
        self
    }

    /// Sets the background color
    pub fn background(mut self, color: Color) -> Self {
        self.style.get_or_insert(TextStyle::default()).background = Some(color);
        self
    }

    /// Makes the text bold
    pub fn bold(mut self) -> Self {
        self.style.get_or_insert(TextStyle::default()).bold = Some(true);
        self
    }

    /// Makes the text italic
    pub fn italic(mut self) -> Self {
        self.style.get_or_insert(TextStyle::default()).italic = Some(true);
        self
    }

    /// Makes the text underlined
    pub fn underline(mut self) -> Self {
        self.style.get_or_insert(TextStyle::default()).underline = Some(true);
        self
    }

    /// Makes the text strikethrough
    pub fn strikethrough(mut self) -> Self {
        self.style.get_or_insert(TextStyle::default()).strikethrough = Some(true);
        self
    }

    /// Sets the text wrapping mode
    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.style.get_or_insert(TextStyle::default()).wrap = Some(wrap);
        self
    }

    /// Sets the text alignment
    pub fn align(mut self, align: TextAlign) -> Self {
        self.style.get_or_insert(TextStyle::default()).align = Some(align);
        self
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl From<String> for Text {
    fn from(content: String) -> Self {
        Self::new(content)
    }
}

impl From<&str> for Text {
    fn from(content: &str) -> Self {
        Self::new(content)
    }
}
