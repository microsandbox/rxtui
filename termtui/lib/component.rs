use crate::app::Context;
use crate::node::Node;
use std::any::Any;
use std::fmt::Debug;

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
pub trait Message: Any + Send + 'static {
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
        self.as_any().downcast_ref::<T>()
    }
}

/// Trait for component state management
pub trait State: Any + Send + 'static {
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
        self.as_any().downcast_ref::<T>()
    }
}

/// Auto-implementation of State for types that are Clone
impl<T> State for T
where
    T: Any + Clone + Send + 'static,
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
pub trait Component: 'static {
    fn update(&self, ctx: &Context, msg: Box<dyn Message>, topic: Option<&str>) -> Action;

    fn view(&self, ctx: &Context) -> Node;

    fn get_id(&self) -> Option<ComponentId>;

    fn set_id(&mut self, id: ComponentId);

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
    T: Any + Clone + Send + 'static,
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
