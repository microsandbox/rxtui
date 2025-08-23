//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Configuration options for debugging and optimization control.
#[derive(Clone)]
pub struct RenderConfig {
    /// Enable double buffering for flicker-free rendering (default: true)
    pub double_buffering: bool,

    /// Enable terminal-specific optimizations (default: true)
    pub terminal_optimizations: bool,

    /// Enable cell-level diffing (default: true)
    pub cell_diffing: bool,

    /// Event polling duration in milliseconds (default: 100ms)
    /// Lower values make the app more responsive but use more CPU
    pub poll_duration_ms: u64,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl RenderConfig {
    /// Creates a debug configuration with all optimizations disabled.
    pub fn debug() -> Self {
        Self {
            double_buffering: false,
            terminal_optimizations: false,
            cell_diffing: false,
            poll_duration_ms: 100,
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            double_buffering: true,
            terminal_optimizations: true,
            cell_diffing: true,
            poll_duration_ms: 100,
        }
    }
}
