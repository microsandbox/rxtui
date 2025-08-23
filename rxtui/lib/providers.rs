//! Provider traits for Component macro system
//!
//! These traits use Rust's method resolution order where inherent methods shadow trait methods,
//! allowing the macro system to provide default implementations that can be optionally overridden.

use crate::{Action, Context, Message, Node};

#[cfg(feature = "effects")]
use crate::effect::Effect;

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// Internal trait for the Component macro system to handle optional update methods.
///
/// DO NOT implement or use this trait directly - it's automatically handled by the macro system.
/// This uses Rust's method resolution order where inherent methods shadow trait methods,
/// allowing #[update] to optionally override the default empty implementation.
#[doc(hidden)]
pub trait UpdateProvider {
    /// Internal method that returns Action::None by default.
    /// This is shadowed by an inherent method when #[update] is used.
    fn __component_update_impl(
        &self,
        _ctx: &Context,
        _msg: Box<dyn Message>,
        _topic: Option<&str>,
    ) -> Action {
        Action::default()
    }
}

/// Internal trait for the Component macro system to handle optional view methods.
///
/// DO NOT implement or use this trait directly - it's automatically handled by the macro system.
/// This uses Rust's method resolution order where inherent methods shadow trait methods,
/// allowing #[view] to optionally override the default empty implementation.
#[doc(hidden)]
pub trait ViewProvider {
    /// Internal method that returns an empty Node by default.
    /// This is shadowed by an inherent method when #[view] is used.
    fn __component_view_impl(&self, _ctx: &Context) -> Node {
        Node::div()
    }
}

/// Internal trait for the Component macro system to handle optional effects.
///
/// DO NOT implement or use this trait directly - it's automatically handled by the macro system.
/// This uses Rust's method resolution order where inherent methods shadow trait methods,
/// allowing #[component] to optionally override the default empty implementation.
#[doc(hidden)]
#[cfg(feature = "effects")]
pub trait EffectsProvider {
    /// Internal method that returns empty effects by default.
    /// This is shadowed by an inherent method when #[component] is used.
    fn __component_effects_impl(&self, _ctx: &Context) -> Vec<Effect> {
        vec![]
    }
}

//--------------------------------------------------------------------------------------------------
// Blanket Implementations
//--------------------------------------------------------------------------------------------------

// Blanket implementations provide the defaults for all types
impl<T> UpdateProvider for T {}
impl<T> ViewProvider for T {}

#[cfg(feature = "effects")]
impl<T> EffectsProvider for T {}
