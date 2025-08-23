//! Async effects system for running background tasks in components
//!
//! Effects allow components to spawn async tasks that run outside the main
//! event loop. They can perform I/O, timers, network requests, etc. and
//! communicate back to the UI through messages.

//--------------------------------------------------------------------------------------------------
// Modules
//--------------------------------------------------------------------------------------------------

mod runtime;
mod types;

//--------------------------------------------------------------------------------------------------
// Exports
//--------------------------------------------------------------------------------------------------

pub use runtime::EffectRuntime;
pub use types::Effect;
