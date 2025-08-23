use std::future::Future;
use std::pin::Pin;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// An effect is just a pinned boxed future that outputs nothing
/// This allows any async operation to be an effect
pub type Effect = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// Internal trait for the Component macro system to handle optional effects.
///
/// DO NOT implement or use this trait directly - it's automatically handled by the macro system.
/// This uses Rust's method resolution order where inherent methods shadow trait methods,
/// allowing #[component] to optionally override the default empty implementation.
#[doc(hidden)]
pub trait EffectsProvider {
    /// Internal method that returns empty effects by default.
    /// This is shadowed by an inherent method when #[component] is used.
    fn __component_effects_impl(&self, _ctx: &crate::Context) -> Vec<Effect> {
        vec![]
    }
}

// Blanket implementation provides the default for all types
impl<T> EffectsProvider for T {}
