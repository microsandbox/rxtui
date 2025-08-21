//! Reusable UI components for termtui
//!
//! This module provides pre-built components that can be easily composed
//! to create more complex user interfaces.

//--------------------------------------------------------------------------------------------------
// Modules
//--------------------------------------------------------------------------------------------------

/// Text input component for user text entry
pub mod text_input;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use text_input::TextInput;
