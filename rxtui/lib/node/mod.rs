use crate::component::Component;
use std::sync::Arc;

pub mod div;
pub mod rich_text;
pub mod text;

pub use div::{Div, DivStyles, EventCallbacks, KeyHandler, KeyWithModifiersHandler};
pub use rich_text::{RichText, TextSpan};
pub use text::Text;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A node in the component tree (can contain components)
#[allow(clippy::large_enum_variant)]
pub enum Node {
    /// A component that can be expanded
    Component(Arc<dyn Component>),

    /// A div that can have children
    Div(Div<Node>),

    /// Text content that is rendered directly
    Text(Text),

    /// Rich text with multiple styled segments
    RichText(RichText),
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Node {
    /// Creates a text node with the given content.
    #[inline]
    pub fn text(content: impl Into<String>) -> Node {
        Node::Text(Text::new(content))
    }

    /// Creates a div node with no children.
    #[inline]
    pub fn div() -> Node {
        Node::Div(Div::new())
    }

    /// Creates a rich text node.
    #[inline]
    pub fn rich_text() -> Node {
        Node::RichText(RichText::new())
    }
}

//--------------------------------------------------------------------------------------------------
// Builder Methods
//--------------------------------------------------------------------------------------------------

impl Node {
    /// Adds a single child (only valid for Div variant).
    #[inline]
    pub fn child(mut self, child: impl Into<Node>) -> Self {
        if let Node::Div(ref mut div) = self {
            div.children.push(child.into());
        }
        self
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Clone for Node {
    fn clone(&self) -> Self {
        match self {
            Node::Component(c) => Node::Component(Arc::clone(c)),
            Node::Div(div) => Node::Div(div.clone()),
            Node::Text(text) => Node::Text(text.clone()),
            Node::RichText(rich) => Node::RichText(rich.clone()),
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Component(_) => write!(f, "Node::Component(...)"),
            Node::Div(div) => write!(f, "Node::Div({div:?})"),
            Node::Text(text) => write!(f, "Node::Text({text:?})"),
            Node::RichText(rich) => write!(f, "Node::RichText({rich:?})"),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node::Text(a), Node::Text(b)) => a == b,
            (Node::RichText(a), Node::RichText(b)) => a == b,
            // Components and containers can't be easily compared due to trait objects
            _ => false,
        }
    }
}

impl From<Text> for Node {
    fn from(text: Text) -> Self {
        Node::Text(text)
    }
}

impl From<RichText> for Node {
    fn from(rich: RichText) -> Self {
        Node::RichText(rich)
    }
}

impl From<Arc<dyn Component>> for Node {
    fn from(component: Arc<dyn Component>) -> Self {
        Node::Component(component)
    }
}

impl From<Div<Node>> for Node {
    fn from(div: Div<Node>) -> Self {
        Node::Div(div)
    }
}
