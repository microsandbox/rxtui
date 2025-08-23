use std::future::Future;
use std::pin::Pin;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// An effect is just a pinned boxed future that outputs nothing
/// This allows any async operation to be an effect
pub type Effect = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
