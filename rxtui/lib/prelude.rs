//! Prelude module for convenient imports.
//!
//! This module re-exports commonly used types and traits for easier usage.
//!
//! # Example
//!
//! ```rust
//! use rxtui::prelude::*;
//! ```

// Core app types
pub use crate::app::{App, Context};

// Component system
pub use crate::component::{Action, ComponentId, Message, MessageExt, State, StateExt};

// Re-export both the trait and the derive macro
pub use crate::Component;
pub use crate::ComponentMacro as Component;

// UI elements
pub use crate::node::{Div, Node, RichText, Text};

// Components
pub use crate::components::TextInput;

// Style types
pub use crate::style::{
    Border, BorderEdges, BorderStyle, Color, Dimension, Direction, Overflow, Position, Spacing,
    Style, TextStyle, TextWrap, WrapMode,
};

// Key handling
pub use crate::key::Key;

// Layout types
pub use crate::bounds::Rect;

// Main macro for building TUI components
pub use crate::node;
