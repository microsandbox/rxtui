use crate::node::{Div, RichText, Text};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A virtual node in the VDOM (components are already expanded)
#[derive(Clone)]
#[allow(clippy::large_enum_variant)]
pub enum VNode {
    /// A div that can have children
    Div(Div<VNode>),

    /// Text content that is rendered directly
    Text(Text),

    /// Rich text with multiple styled segments
    RichText(RichText),
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl VNode {
    /// Creates a text vnode with the given content.
    #[inline]
    pub fn text(content: impl Into<String>) -> VNode {
        VNode::Text(Text::new(content))
    }

    /// Creates a div vnode with no children.
    #[inline]
    pub fn div() -> VNode {
        VNode::Div(Div::new())
    }

    /// Creates a rich text vnode.
    #[inline]
    pub fn rich_text() -> VNode {
        VNode::RichText(RichText::new())
    }
}

//--------------------------------------------------------------------------------------------------
// Builder Methods
//--------------------------------------------------------------------------------------------------

impl VNode {
    /// Adds a single child (only valid for Div variant).
    #[inline]
    pub fn child(mut self, child: impl Into<VNode>) -> Self {
        if let VNode::Div(ref mut div) = self {
            div.children.push(child.into());
        }
        self
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl std::fmt::Debug for VNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VNode::Div(div) => write!(f, "VNode::Div({div:?})"),
            VNode::Text(text) => write!(f, "VNode::Text({text:?})"),
            VNode::RichText(rich) => write!(f, "VNode::RichText({rich:?})"),
        }
    }
}

impl PartialEq for VNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VNode::Text(a), VNode::Text(b)) => a == b,
            (VNode::RichText(a), VNode::RichText(b)) => a == b,
            // Divs with callbacks can't be easily compared
            _ => false,
        }
    }
}

impl From<Text> for VNode {
    #[inline]
    fn from(text: Text) -> Self {
        VNode::Text(text)
    }
}

impl From<Div<VNode>> for VNode {
    #[inline]
    fn from(div: Div<VNode>) -> Self {
        VNode::Div(div)
    }
}

impl From<String> for VNode {
    #[inline]
    fn from(content: String) -> Self {
        VNode::Text(Text::from(content))
    }
}

impl From<&str> for VNode {
    #[inline]
    fn from(content: &str) -> Self {
        VNode::Text(Text::from(content))
    }
}

impl From<RichText> for VNode {
    #[inline]
    fn from(rich: RichText) -> Self {
        VNode::RichText(rich)
    }
}
