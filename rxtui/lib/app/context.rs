use crate::component::{ComponentId, Message, State};
use std::any::TypeId;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, Ordering},
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Type alias for the message queue storage
type MessageQueueMap = Arc<RwLock<HashMap<ComponentId, VecDeque<Box<dyn Message>>>>>;

/// Type alias for topic message queue storage
type TopicMessageQueueMap = Arc<RwLock<HashMap<String, VecDeque<Box<dyn Message>>>>>;

/// Dispatcher for sending messages to components
#[derive(Clone)]
pub struct Dispatcher {
    queues: MessageQueueMap,
    topic_queues: TopicMessageQueueMap,
}

/// State storage for components with interior mutability
#[derive(Clone)]
pub struct StateMap {
    states: Arc<RwLock<HashMap<ComponentId, Box<dyn State>>>>,
}

/// Target for focus requests emitted during rendering
#[derive(Clone)]
pub(crate) enum FocusTarget {
    /// Focus the first focusable element inside the component's subtree
    Component(ComponentId),

    /// Focus the first focusable element in the entire application tree
    GlobalFirst,
}

/// Pending focus request queued by components
#[derive(Clone)]
pub(crate) struct FocusRequest {
    pub target: FocusTarget,
}

/// Topic storage for shared state between components
pub struct TopicStore {
    /// Topic states indexed by topic name
    states: RwLock<HashMap<String, Box<dyn State>>>,

    /// Topic owners - first writer becomes owner
    owners: RwLock<HashMap<String, ComponentId>>,
}

/// Tracks component instances for effect management
#[derive(Clone)]
pub struct ComponentInstanceTracker {
    /// Set of (ComponentId, TypeId) pairs for components with spawned effects
    spawned_effects: Arc<RwLock<HashSet<(ComponentId, TypeId)>>>,
}

/// Context passed to components during rendering
#[derive(Clone)]
pub struct Context {
    /// Current component ID in the tree walk
    pub(crate) current_component_id: ComponentId,

    /// Message dispatcher
    pub(crate) dispatch: Dispatcher,

    /// Component states
    pub(crate) states: StateMap,

    /// Topic states
    pub(crate) topics: Arc<TopicStore>,

    /// Message queues (shared with dispatcher)
    pub(crate) message_queues: MessageQueueMap,

    /// Topic message queues (shared with dispatcher)
    pub(crate) topic_message_queues: TopicMessageQueueMap,

    /// Tracks which components have effects spawned
    pub(crate) effect_tracker: ComponentInstanceTracker,

    /// Focus requests queued during rendering
    pub(crate) pending_focus_requests: Arc<RwLock<Vec<FocusRequest>>>,

    /// Pending request to clear focus if nothing else claims it
    pub(crate) pending_focus_clear: Arc<AtomicBool>,

    /// Components that have completed their first render pass
    pub(crate) rendered_components: Arc<RwLock<HashSet<ComponentId>>>,

    /// Whether the current component invocation is on its first render
    pub(crate) current_is_first_render: Arc<RwLock<bool>>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Dispatcher {
    pub fn new(queues: MessageQueueMap, topic_queues: TopicMessageQueueMap) -> Self {
        Self {
            queues,
            topic_queues,
        }
    }

    pub fn send_to_id(&self, component_id: ComponentId, message: impl Message) {
        let mut queues = self.queues.write().unwrap();
        queues
            .entry(component_id)
            .or_default()
            .push_back(Box::new(message));
    }

    pub fn send_to_topic(&self, topic: String, message: impl Message) {
        let mut queues = self.topic_queues.write().unwrap();
        queues
            .entry(topic)
            .or_default()
            .push_back(Box::new(message));
    }
}

impl StateMap {
    pub fn new() -> Self {
        Self {
            states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_or_init<T: State + Default + Clone + 'static>(
        &self,
        component_id: &ComponentId,
    ) -> T {
        let mut states = self.states.write().unwrap();

        // Check if entry exists and try to downcast
        if let Some(existing_state) = states.get(component_id)
            && let Some(typed_state) = State::as_any(existing_state.as_ref()).downcast_ref::<T>()
        {
            // Type matches, return the existing state
            return typed_state.clone();
        }
        // Type mismatch or no entry - will replace with new default below

        // Either no entry exists or type mismatch - create new default
        let new_state = Box::new(T::default());
        let cloned = State::as_any(new_state.as_ref())
            .downcast_ref::<T>()
            .unwrap()
            .clone();
        states.insert(component_id.clone(), new_state);
        cloned
    }

    pub fn insert(&self, component_id: ComponentId, state: Box<dyn State>) {
        self.states.write().unwrap().insert(component_id, state);
    }

    pub fn remove(&self, component_id: &ComponentId) -> Option<Box<dyn State>> {
        self.states.write().unwrap().remove(component_id)
    }
}

impl TopicStore {
    pub fn new() -> Self {
        Self {
            states: RwLock::new(HashMap::new()),
            owners: RwLock::new(HashMap::new()),
        }
    }

    pub(crate) fn update_topic(
        &self,
        topic: String,
        state: Box<dyn State>,
        component_id: ComponentId,
    ) -> bool {
        let mut owners = self.owners.write().unwrap();
        let mut states = self.states.write().unwrap();

        // Check if topic has an owner
        if let Some(owner) = owners.get(&topic) {
            // Only the owner can update the topic
            if owner == &component_id {
                states.insert(topic, state);
                true
            } else {
                false
            }
        } else {
            // First writer becomes the owner
            owners.insert(topic.clone(), component_id);
            states.insert(topic, state);
            true
        }
    }

    /// Claim ownership of an unassigned topic
    pub(crate) fn claim_topic(&self, topic: String, component_id: ComponentId) -> bool {
        let mut owners = self.owners.write().unwrap();

        // Only claim if topic has no owner
        use std::collections::hash_map::Entry;
        if let Entry::Vacant(e) = owners.entry(topic) {
            e.insert(component_id);
            true
        } else {
            false
        }
    }

    pub fn read_topic<T: State + Clone + 'static>(&self, topic: &str) -> Option<T> {
        let states = self.states.read().unwrap();
        states
            .get(topic)
            .and_then(|state| State::as_any(state.as_ref()).downcast_ref::<T>().cloned())
    }

    pub fn get_topic_owner(&self, topic: &str) -> Option<ComponentId> {
        self.owners.read().unwrap().get(topic).cloned()
    }

    pub fn get_owned_topics(&self, component_id: &ComponentId) -> Vec<String> {
        self.owners
            .read()
            .unwrap()
            .iter()
            .filter_map(|(topic, owner)| {
                if owner == component_id {
                    Some(topic.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl ComponentInstanceTracker {
    pub fn new() -> Self {
        Self {
            spawned_effects: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Check if a component instance has effects spawned
    pub fn has_effects(&self, component_id: &ComponentId, type_id: TypeId) -> bool {
        self.spawned_effects
            .read()
            .unwrap()
            .contains(&(component_id.clone(), type_id))
    }

    /// Mark a component instance as having effects spawned
    pub fn mark_spawned(&self, component_id: ComponentId, type_id: TypeId) {
        self.spawned_effects
            .write()
            .unwrap()
            .insert((component_id, type_id));
    }

    /// Remove a component instance from tracking (for cleanup)
    pub fn remove(&self, component_id: &ComponentId, type_id: TypeId) -> bool {
        self.spawned_effects
            .write()
            .unwrap()
            .remove(&(component_id.clone(), type_id))
    }

    /// Get all tracked component instances
    pub fn get_all(&self) -> HashSet<(ComponentId, TypeId)> {
        self.spawned_effects.read().unwrap().clone()
    }
}

impl Default for ComponentInstanceTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new(pending_focus_clear: Arc<AtomicBool>) -> Self {
        let queues = Arc::new(RwLock::new(HashMap::new()));
        let topic_queues = Arc::new(RwLock::new(HashMap::new()));

        Self {
            current_component_id: ComponentId::default(),
            dispatch: Dispatcher::new(queues.clone(), topic_queues.clone()),
            states: StateMap::new(),
            topics: Arc::new(TopicStore::new()),
            message_queues: queues,
            topic_message_queues: topic_queues,
            effect_tracker: ComponentInstanceTracker::new(),
            pending_focus_requests: Arc::new(RwLock::new(Vec::new())),
            pending_focus_clear,
            rendered_components: Arc::new(RwLock::new(HashSet::new())),
            current_is_first_render: Arc::new(RwLock::new(false)),
        }
    }

    /// Get the current component's ID
    pub fn id(&self) -> &ComponentId {
        &self.current_component_id
    }

    /// Creates a message handler that captures the current component ID
    pub fn handler<T: Message + Clone + 'static>(&self, msg: T) -> Box<dyn Fn() + 'static> {
        let id = self.current_component_id.clone();
        let dispatcher = self.dispatch.clone();
        Box::new(move || {
            dispatcher.send_to_id(id.clone(), msg.clone());
        })
    }

    /// Creates a message handler with a value parameter
    pub fn handler_with_value<T, M, F>(&self, msg_fn: F) -> Box<dyn Fn(T) + 'static>
    where
        T: 'static,
        M: Message + 'static,
        F: Fn(T) -> M + 'static,
    {
        let id = self.current_component_id.clone();
        let dispatcher = self.dispatch.clone();
        Box::new(move |value| {
            dispatcher.send_to_id(id.clone(), msg_fn(value));
        })
    }

    /// Get the state for the current component, initializing with Default if not already present
    pub fn get_state<T: State + Default + Clone + 'static>(&self) -> T {
        self.states.get_or_init::<T>(&self.current_component_id)
    }

    /// Set state for the current component
    pub fn set_state(&self, state: Box<dyn State>) {
        self.states.insert(self.current_component_id.clone(), state);
    }

    /// Read state from a topic
    pub fn read_topic<T: State + Clone + 'static>(&self, topic: &str) -> Option<T> {
        self.topics.read_topic(topic)
    }

    /// Send a message to the current component
    pub fn send(&self, message: impl Message) {
        self.dispatch
            .send_to_id(self.current_component_id.clone(), message);
    }

    /// Send a message to a specific component
    pub fn send_to(&self, component_id: ComponentId, message: impl Message) {
        self.dispatch.send_to_id(component_id, message);
    }

    /// Send a message to a topic owner
    pub fn send_to_topic(&self, topic: impl Into<String>, message: impl Message) {
        self.dispatch.send_to_topic(topic.into(), message);
    }

    /// Creates a topic message handler
    pub fn topic_handler<T: Message + Clone + 'static>(
        &self,
        topic: impl Into<String>,
        msg: T,
    ) -> impl Fn() + 'static {
        let topic = topic.into();
        let dispatcher = self.dispatch.clone();
        move || {
            dispatcher.send_to_topic(topic.clone(), msg.clone());
        }
    }

    /// Creates a topic message handler with a value parameter
    pub fn topic_handler_with_value<T, M, F>(
        &self,
        topic: impl Into<String>,
        msg_fn: F,
    ) -> impl Fn(T) + 'static
    where
        T: 'static,
        M: Message + 'static,
        F: Fn(T) -> M + 'static,
    {
        let topic = topic.into();
        let dispatcher = self.dispatch.clone();
        move |value| {
            dispatcher.send_to_topic(topic.clone(), msg_fn(value));
        }
    }

    /// Create a child context with updated component ID
    pub fn child(&self, index: usize) -> Self {
        Self {
            current_component_id: self.current_component_id.child(index),
            dispatch: self.dispatch.clone(),
            states: self.states.clone(), // Share the state map
            topics: self.topics.clone(), // Share the topic store
            message_queues: self.message_queues.clone(), // Share the message queues
            topic_message_queues: self.topic_message_queues.clone(), // Share the topic message queues
            effect_tracker: self.effect_tracker.clone(),             // Share the effect tracker
            pending_focus_requests: self.pending_focus_requests.clone(),
            pending_focus_clear: self.pending_focus_clear.clone(),
            rendered_components: self.rendered_components.clone(),
            current_is_first_render: self.current_is_first_render.clone(),
        }
    }

    /// Request focus for the first focusable element inside the current component
    pub fn focus_self(&self) {
        let mut queue = self.pending_focus_requests.write().unwrap();
        queue.push(FocusRequest {
            target: FocusTarget::Component(self.current_component_id.clone()),
        });
    }

    /// Request focus for the first focusable element in the entire tree
    pub fn focus_first(&self) {
        let mut queue = self.pending_focus_requests.write().unwrap();
        queue.push(FocusRequest {
            target: FocusTarget::GlobalFirst,
        });
    }

    /// Request that no element remain focused after this render cycle.
    pub fn blur_focus(&self) {
        self.pending_focus_clear.store(true, Ordering::SeqCst);
    }

    /// Drain all focus requests accumulated during rendering
    pub(crate) fn take_focus_requests(&self) -> Vec<FocusRequest> {
        let mut queue = self.pending_focus_requests.write().unwrap();
        queue.drain(..).collect()
    }

    /// Returns true if a focus clear was requested and resets the flag.
    pub(crate) fn take_focus_clear_request(&self) -> bool {
        self.pending_focus_clear.swap(false, Ordering::SeqCst)
    }

    /// Cancels any pending focus clear request.
    pub(crate) fn cancel_focus_clear(&self) {
        self.pending_focus_clear.store(false, Ordering::SeqCst);
    }

    /// Mark the beginning of a component render and return whether it is the first render
    pub(crate) fn begin_component_render(&self) -> bool {
        let mut rendered = self.rendered_components.write().unwrap();
        let is_first = rendered.insert(self.current_component_id.clone());
        *self.current_is_first_render.write().unwrap() = is_first;
        is_first
    }

    /// Mark the end of a component render
    pub(crate) fn end_component_render(&self) {
        *self.current_is_first_render.write().unwrap() = false;
    }

    /// Returns true if the current render invocation is the component's first
    pub fn is_first_render(&self) -> bool {
        *self.current_is_first_render.read().unwrap()
    }

    /// Take and drain messages for a specific component
    pub fn drain_messages(&self, component_id: &ComponentId) -> Vec<Box<dyn Message>> {
        let mut queues = self.message_queues.write().unwrap();
        if let Some(queue) = queues.get_mut(component_id) {
            queue.drain(..).collect()
        } else {
            Vec::new()
        }
    }

    /// Take and drain messages for a specific topic
    pub fn drain_topic_messages(&self, topic: &str) -> Vec<Box<dyn Message>> {
        let mut queues = self.topic_message_queues.write().unwrap();
        if let Some(queue) = queues.get_mut(topic) {
            queue.drain(..).collect()
        } else {
            Vec::new()
        }
    }

    /// Drain all messages for the current component (regular, owned topics, and unassigned topics)
    pub fn drain_all_messages(&self) -> Vec<(Box<dyn Message>, Option<String>)> {
        let mut all_messages = Vec::new();

        // Get regular component messages (no topic associated)
        for msg in self.drain_messages(&self.current_component_id) {
            all_messages.push((msg, None));
        }

        // Get messages for topics owned by this component
        let owned_topics = self.topics.get_owned_topics(&self.current_component_id);
        for topic in owned_topics {
            for msg in self.drain_topic_messages(&topic) {
                all_messages.push((msg, Some(topic.clone())));
            }
        }

        // Get cloned unassigned topic messages (topics without owners)
        let unassigned = self.get_unassigned_topic_messages();
        for (topic, msg) in unassigned {
            all_messages.push((msg, Some(topic)));
        }

        all_messages
    }

    /// Get cloned messages from topics that don't have owners yet
    fn get_unassigned_topic_messages(&self) -> Vec<(String, Box<dyn Message>)> {
        let mut unassigned = Vec::new();
        let topic_queues = self.topic_message_queues.read().unwrap();

        // Check each topic queue
        for (topic, queue) in topic_queues.iter() {
            // If this topic has no owner, clone its messages (don't drain)
            if self.topics.get_topic_owner(topic).is_none() && !queue.is_empty() {
                for msg in queue.iter() {
                    unassigned.push((topic.clone(), Message::clone_box(msg.as_ref())));
                }
            }
        }

        unassigned
    }

    /// Drain topic messages if the topic was just claimed
    pub fn drain_topic_if_claimed(&self, topic: &str, component_id: &ComponentId) {
        // Check if this component just became the owner
        if let Some(owner) = self.topics.get_topic_owner(topic)
            && owner == *component_id
        {
            // Drain the messages since we now own the topic
            let mut queues = self.topic_message_queues.write().unwrap();
            if let Some(queue) = queues.get_mut(topic) {
                queue.clear();
            }
        }
    }

    /// Check if there are any pending messages in any queue
    pub fn has_pending_messages(&self) -> bool {
        // Check component message queues
        {
            let queues = self.message_queues.read().unwrap();
            for queue in queues.values() {
                if !queue.is_empty() {
                    return true;
                }
            }
        }

        // Check topic message queues
        {
            let queues = self.topic_message_queues.read().unwrap();
            for queue in queues.values() {
                if !queue.is_empty() {
                    return true;
                }
            }
        }

        false
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for TopicStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StateMap {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new(Arc::new(AtomicBool::new(false)))
    }
}
