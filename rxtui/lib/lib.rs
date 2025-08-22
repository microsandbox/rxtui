//! # Radical TUI - Terminal User Interface Framework
//!
//! A reactive terminal UI framework inspired by React's virtual DOM architecture.
//! Provides a declarative API for building interactive terminal applications.
//!
//! ## Architecture Overview
//!
//! ```text
//!     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
//!     │  Elements   │────▶│    Node     │────▶│    VDom     │
//!     │   Factory   │     │    Tree     │     │   State     │
//!     └─────────────┘     └─────────────┘     └─────────────┘
//!            │                    │                    │
//!            │                    │                    │
//!            ▼                    ▼                    ▼
//!     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
//!     │    Style    │     │    Diff     │────▶│   Render    │
//!     │   System    │     │   Engine    │     │   Engine    │
//!     └─────────────┘     └─────────────┘     └─────────────┘
//!                                                     │
//!                                                     ▼
//!                                            ┌─────────────┐
//!                                            │  Terminal   │
//!                                            │   Output    │
//!                                            └─────────────┘
//! ```
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use rxtui::{Elements, ElementBuilder, Text, Node, Color, Direction, Spacing};
//!
//! // Create UI elements using the Elements factory
//! let ui = Elements::div()
//!     .background(Color::Blue)
//!     .padding(Spacing::all(2))
//!     .children(vec![
//!         Text::new("Hello, TUI!").color(Color::White).into(),
//!         Elements::div()
//!             .direction(Direction::Horizontal)
//!             .children(vec![
//!                 Text::new("Left").into(),
//!                 Text::new("Right").into(),
//!             ])
//!             .into(),
//!     ])
//!     .into();
//! ```
//!
//! ## Key Components
//!
//! - **Node**: Virtual representation of UI elements (Element trait objects or Text)
//! - **Elements**: Factory for creating concrete element types (Div, etc.)
//! - **Element**: Trait for element data access (props, children)
//! - **ElementBuilder**: Trait providing fluent builder API for elements
//! - **VDom**: Manages the virtual DOM state and updates
//! - **Diff**: Calculates minimal changes between UI states
//! - **Render**: Translates virtual nodes to terminal output
//! - **App**: Main application lifecycle and event loop
//! - **Model**: Type-safe init-view-update architecture for stateful models

//--------------------------------------------------------------------------------------------------
// Modules: Core Components
//--------------------------------------------------------------------------------------------------

/// Prelude module for convenient imports
pub mod prelude;

/// New component-based system (parallel implementation)
pub mod component;

/// Node types for component tree (includes div, text, rich_text)
pub mod node;

/// Virtual node types for the VDOM
pub mod vnode;

//--------------------------------------------------------------------------------------------------
// Modules: Rendering
//--------------------------------------------------------------------------------------------------

/// Virtual DOM implementation for managing the UI state.
/// Maintains the current UI tree and applies patches from the diff engine.
pub mod vdom;

/// Diffing algorithm for efficiently updating the UI.
/// Compares old and new virtual DOM trees to generate minimal change patches.
pub mod diff;

/// Rendering engine that converts virtual nodes into terminal output.
/// Handles the actual drawing of elements to the screen.
pub mod render_tree;

/// Double buffering and cell-level diffing for flicker-free rendering.
/// Maintains screen state to enable precise, minimal updates.
pub mod buffer;

/// Optimized terminal renderer for applying cell updates.
/// Minimizes escape sequences and I/O operations for best performance.
pub mod terminal;

//--------------------------------------------------------------------------------------------------
// Modules: Application
//--------------------------------------------------------------------------------------------------

/// Application framework for building terminal UIs.
/// Provides the main application lifecycle and event handling.
pub mod app;

//--------------------------------------------------------------------------------------------------
// Modules: Styling & Layout
//--------------------------------------------------------------------------------------------------

/// Styling system for terminal UI components.
/// Defines colors, spacing, borders, and other visual properties.
pub mod style;

/// Bounds and rectangle operations for dirty region tracking.
/// Provides types for tracking screen regions that need redrawing.
pub mod bounds;

//--------------------------------------------------------------------------------------------------
// Modules: Input & Utilities
//--------------------------------------------------------------------------------------------------

/// Key representation for keyboard input.
/// Provides an enum for representing both characters and special keys.
pub mod key;

/// Utilities for terminal rendering, Unicode width calculations, and text wrapping.
/// Provides helpers for display width, text manipulation, and wrapping algorithms.
pub mod utils;

//--------------------------------------------------------------------------------------------------
// Modules: Macros
//--------------------------------------------------------------------------------------------------

/// Macro-based DSL for building TUI components
/// Provides ergonomic macros for composing components with less boilerplate
pub mod macros;

//--------------------------------------------------------------------------------------------------
// Modules: Components
//--------------------------------------------------------------------------------------------------

/// Reusable UI components for building forms and interfaces
/// Provides pre-built components like TextInput, Button, etc.
pub mod components;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

// Re-export the derive macro with the same name
#[doc(hidden)]
pub use rxtui_macros::Component as ComponentMacro;
pub use rxtui_macros::{update, view};

pub use app::{App, Context, Dispatcher, RenderConfig, StateMap};
pub use bounds::Rect;
pub use component::{Action, Component, ComponentId, Message, State};
pub use components::TextInput;
pub use diff::{Patch, diff};
pub use key::Key;
pub use node::{Div, Node, RichText, Text, TextSpan};
pub use render_tree::RenderNode;
pub use style::{
    BorderEdges, BorderStyle, Color, Dimension, Direction, Overflow, Position, Spacing, Style,
    TextStyle, TextWrap, WrapMode,
};
pub use utils::wrap_text;
pub use vdom::VDom;
pub use vnode::VNode;

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------
