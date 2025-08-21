//! Render tree and layout engine for terminal UI.
//!
//! This module transforms virtual nodes into a render tree with calculated
//! positions and dimensions. The render tree is what actually gets drawn
//! to the terminal screen.
//!
//! ## Rendering Pipeline
//!
//! ```text
//!   Node Tree               Render Tree            Terminal
//!   ┌─────────┐           ┌────────────┐          ┌─────────┐
//!   │ element │ ──build─▶ │ RenderNode │ ──draw─▶ │ ┌─────┐ │
//!   │  text   │           │ x:0 y:0    │          │ │Hello│ │
//!   └─────────┘           │ w:10 h:1   │          │ └─────┘ │
//!                         └────────────┘          └─────────┘
//! ```
//!
//! ## Layout Algorithm
//!
//! The layout system supports:
//! - Vertical and horizontal stacking
//! - Padding and spacing
//! - Fixed and flexible sizing
//! - Hit testing for mouse events

mod node;
mod tree;

pub use node::{RenderNode, RenderNodeType};
pub use tree::RenderTree;

#[cfg(test)]
mod tests;
