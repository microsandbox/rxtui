use crate::app::Context;
use crate::node::Node;
use std::any::Any;
use std::fmt::Debug;

#[cfg(feature = "effects")]
use crate::effect::Effect;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Action returned by a component's update method
pub enum Action {
    /// Update the component's state
    Update(Box<dyn State>),

    /// Update a topic's state (idempotent - first writer becomes owner)
    UpdateTopic(String, Box<dyn State>),

    /// No action needed
    None,

    /// Exit the application
    Exit,
}

/// Unique identifier for components in the tree
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub String);

/// Trait for messages that can be sent between components
pub trait Message: Any + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Message>;
}

/// Extension trait for convenient message downcasting
pub trait MessageExt {
    /// Downcast the message to a concrete type
    fn downcast<T: Any>(&self) -> Option<&T>;
}

impl MessageExt for dyn Message {
    fn downcast<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl MessageExt for Box<dyn Message> {
    fn downcast<T: Any>(&self) -> Option<&T> {
        Message::as_any(self.as_ref()).downcast_ref::<T>()
    }
}

/// Trait for component state management
pub trait State: Any + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clone_box(&self) -> Box<dyn State>;
}

/// Extension trait for convenient state downcasting
pub trait StateExt {
    /// Downcast the state to a concrete type
    fn downcast<T: Any>(&self) -> Option<&T>;
}

impl StateExt for dyn State {
    fn downcast<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl StateExt for Box<dyn State> {
    fn downcast<T: Any>(&self) -> Option<&T> {
        State::as_any(self.as_ref()).downcast_ref::<T>()
    }
}

/// Auto-implementation of State for types that are Clone
impl<T> State for T
where
    T: Any + Clone + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }
}

/// Main component trait for building UI components
///
/// Components can be created easily using the `#[derive(Component)]` macro.
/// The `update` and `view` methods can be simplified using the `#[update]` and `#[view]`
/// attribute macros which automatically handle message downcasting and state fetching.
///
/// # Example
///
/// ```ignore
/// use rxtui::prelude::*;
///
/// #[derive(Component, Clone, Default)]
/// struct Counter {}
///
/// impl Counter {
///     // Using the #[update] macro - handles downcasting and state automatically
///     #[update]
///     fn update(&self, ctx: &Context, msg: CounterMsg, mut state: CounterState) -> Action {
///         match msg {
///             CounterMsg::Increment => {
///                 state.count += 1;
///                 Action::update(state)
///             }
///             CounterMsg::Decrement => {
///                 state.count -= 1;
///                 Action::update(state)
///             }
///         }
///     }
///
///     // Using the #[view] macro - automatically fetches state
///     #[view]
///     fn view(&self, ctx: &Context, state: CounterState) -> Node {
///         node! {
///             div [
///                 text(format!("Count: {}", state.count))
///             ]
///         }
///     }
/// }
/// ```
///
/// # Manual Implementation
///
/// The trait can also be implemented manually for more control:
///
/// ```ignore
/// fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action {
///     if let Some(msg) = msg.downcast::<MyMsg>() {
///         // Handle message
///     }
///     Action::none()
/// }
///
/// fn view(&self, ctx: &Context) -> Node {
///     let state = ctx.get_state::<MyState>();
///     // Build UI
/// }
/// ```
pub trait Component: 'static {
    fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action;

    fn view(&self, ctx: &Context) -> Node;

    /// Define effects for this component
    /// Effects are async tasks that run outside the main event loop
    /// They are spawned when the component mounts and cancelled when it unmounts
    #[cfg(feature = "effects")]
    fn effects(&self, _ctx: &Context) -> Vec<Effect> {
        vec![]
    }

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn clone_box(&self) -> Box<dyn Component>;
}

/// Helper trait for cloning Component trait objects
pub trait ComponentClone {
    fn clone_box(&self) -> Box<dyn Component>;
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Action {
    /// Create an Update action with the given state
    #[inline]
    pub fn update(state: impl State) -> Self {
        Action::Update(Box::new(state))
    }

    /// Create an UpdateTopic action with the given topic and state
    #[inline]
    pub fn update_topic(topic: impl Into<String>, state: impl State) -> Self {
        Action::UpdateTopic(topic.into(), Box::new(state))
    }

    /// Create a None action (no-op)
    #[inline(always)]
    pub fn none() -> Self {
        Action::None
    }

    /// Create an Exit action to terminate the application
    #[inline(always)]
    pub fn exit() -> Self {
        Action::Exit
    }
}

impl ComponentId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn child(&self, index: usize) -> Self {
        Self(format!("{}.{}", self.0, index))
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for ComponentId {
    fn default() -> Self {
        Self("0".to_string())
    }
}

impl<T> Message for T
where
    T: Any + Clone + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn Message> {
        Box::new(self.clone())
    }
}

impl<T> ComponentClone for T
where
    T: Component + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}
