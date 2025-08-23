use std::future::Future;
use std::pin::Pin;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// An effect is just a pinned boxed future that outputs nothing
/// This allows any async operation to be an effect
pub type Effect = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// Dependencies that determine when an effect should run
#[derive(Debug, Clone)]
pub enum EffectDeps {
    /// Run once when component mounts
    OnMount,

    /// Run when specific state fields change
    /// (Not implemented in initial version)
    OnChange(Vec<String>),

    /// Run on every component update
    /// (Not implemented in initial version)
    Always,
}

/// An effect definition with its dependencies
pub struct EffectDef {
    /// When this effect should run
    pub deps: EffectDeps,

    /// The actual effect to run
    pub effect: Effect,
}
