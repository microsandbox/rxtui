use crate::component::{ComponentId, Message, State};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Type alias for the message queue storage
type MessageQueueMap = Rc<RefCell<HashMap<ComponentId, VecDeque<Box<dyn Message>>>>>;

/// Type alias for topic message queue storage
type TopicMessageQueueMap = Rc<RefCell<HashMap<String, VecDeque<Box<dyn Message>>>>>;

/// Dispatcher for sending messages to components
#[derive(Clone)]
pub struct Dispatcher {
    queues: MessageQueueMap,
    topic_queues: TopicMessageQueueMap,
}

/// State storage for components with interior mutability
pub struct StateMap {
    states: RefCell<HashMap<ComponentId, Box<dyn State>>>,
}

/// Topic storage for shared state between components
pub struct TopicStore {
    /// Topic states indexed by topic name
    states: RefCell<HashMap<String, Box<dyn State>>>,

    /// Topic owners - first writer becomes owner
    owners: RefCell<HashMap<String, ComponentId>>,
}

/// Context passed to components during rendering
pub struct Context {
    /// Current component ID in the tree walk
    pub(crate) current_component_id: ComponentId,

    /// Message dispatcher
    pub(crate) dispatch: Dispatcher,

    /// Component states
    pub(crate) states: StateMap,

    /// Topic states
    pub(crate) topics: Rc<TopicStore>,

    /// Message queues (shared with dispatcher)
    pub(crate) message_queues: MessageQueueMap,

    /// Topic message queues (shared with dispatcher)
    pub(crate) topic_message_queues: TopicMessageQueueMap,
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

    pub fn send(&self, component_id: ComponentId, message: Box<dyn Message>) {
        let mut queues = self.queues.borrow_mut();
        queues.entry(component_id).or_default().push_back(message);
    }

    pub fn send_to_topic(&self, topic: String, message: Box<dyn Message>) {
        let mut queues = self.topic_queues.borrow_mut();
        queues.entry(topic).or_default().push_back(message);
    }
}

impl StateMap {
    pub fn new() -> Self {
        Self {
            states: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_or_init<T: State + Default + Clone + 'static>(
        &self,
        component_id: &ComponentId,
    ) -> T {
        let mut states = self.states.borrow_mut();

        // Check if entry exists and try to downcast
        if let Some(existing_state) = states.get(component_id)
            && let Some(typed_state) = existing_state.as_any().downcast_ref::<T>()
        {
            // Type matches, return the existing state
            return typed_state.clone();
        }
        // Type mismatch or no entry - will replace with new default below

        // Either no entry exists or type mismatch - create new default
        let new_state = Box::new(T::default());
        let cloned = State::as_any(&*new_state)
            .downcast_ref::<T>()
            .unwrap()
            .clone();
        states.insert(component_id.clone(), new_state);
        cloned
    }

    pub fn insert(&self, component_id: ComponentId, state: Box<dyn State>) {
        self.states.borrow_mut().insert(component_id, state);
    }

    pub fn remove(&self, component_id: &ComponentId) -> Option<Box<dyn State>> {
        self.states.borrow_mut().remove(component_id)
    }
}

impl TopicStore {
    pub fn new() -> Self {
        Self {
            states: RefCell::new(HashMap::new()),
            owners: RefCell::new(HashMap::new()),
        }
    }

    pub(crate) fn update_topic(
        &self,
        topic: String,
        state: Box<dyn State>,
        component_id: ComponentId,
    ) -> bool {
        let mut owners = self.owners.borrow_mut();
        let mut states = self.states.borrow_mut();

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
        let mut owners = self.owners.borrow_mut();

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
        let states = self.states.borrow();
        states
            .get(topic)
            .and_then(|state| state.as_any().downcast_ref::<T>().cloned())
    }

    pub fn get_topic_owner(&self, topic: &str) -> Option<ComponentId> {
        self.owners.borrow().get(topic).cloned()
    }

    pub fn get_owned_topics(&self, component_id: &ComponentId) -> Vec<String> {
        self.owners
            .borrow()
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

impl Context {
    pub fn new() -> Self {
        let queues = Rc::new(RefCell::new(HashMap::new()));
        let topic_queues = Rc::new(RefCell::new(HashMap::new()));

        Self {
            current_component_id: ComponentId::default(),
            dispatch: Dispatcher::new(queues.clone(), topic_queues.clone()),
            states: StateMap::new(),
            topics: Rc::new(TopicStore::new()),
            message_queues: queues,
            topic_message_queues: topic_queues,
        }
    }

    /// Creates a message handler that captures the current component ID
    pub fn handler<T: Message + Clone + 'static>(&self, msg: T) -> impl Fn() + 'static {
        let id = self.current_component_id.clone();
        let dispatcher = self.dispatch.clone();
        move || {
            dispatcher.send(id.clone(), Box::new(msg.clone()));
        }
    }

    /// Creates a message handler with a value parameter
    pub fn handler_with_value<T, F>(&self, msg_fn: F) -> impl Fn(T) + 'static
    where
        T: 'static,
        F: Fn(T) -> Box<dyn Message> + 'static,
    {
        let id = self.current_component_id.clone();
        let dispatcher = self.dispatch.clone();
        move |value| {
            dispatcher.send(id.clone(), msg_fn(value));
        }
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

    /// Send a message to a topic owner
    pub fn send_to_topic(&self, topic: impl Into<String>, message: Box<dyn Message>) {
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
            dispatcher.send_to_topic(topic.clone(), Box::new(msg.clone()));
        }
    }

    /// Creates a topic message handler with a value parameter
    pub fn topic_handler_with_value<T, F>(
        &self,
        topic: impl Into<String>,
        msg_fn: F,
    ) -> impl Fn(T) + 'static
    where
        T: 'static,
        F: Fn(T) -> Box<dyn Message> + 'static,
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
            states: StateMap::new(), // Child context gets empty state map, will be populated as needed
            topics: self.topics.clone(), // Share the topic store
            message_queues: self.message_queues.clone(), // Share the message queues
            topic_message_queues: self.topic_message_queues.clone(), // Share the topic message queues
        }
    }

    /// Take and drain messages for a specific component
    pub fn drain_messages(&self, component_id: &ComponentId) -> Vec<Box<dyn Message>> {
        let mut queues = self.message_queues.borrow_mut();
        if let Some(queue) = queues.get_mut(component_id) {
            queue.drain(..).collect()
        } else {
            Vec::new()
        }
    }

    /// Take and drain messages for a specific topic
    pub fn drain_topic_messages(&self, topic: &str) -> Vec<Box<dyn Message>> {
        let mut queues = self.topic_message_queues.borrow_mut();
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
        let topic_queues = self.topic_message_queues.borrow();

        // Check each topic queue
        for (topic, queue) in topic_queues.iter() {
            // If this topic has no owner, clone its messages (don't drain)
            if self.topics.get_topic_owner(topic).is_none() && !queue.is_empty() {
                for msg in queue.iter() {
                    unassigned.push((topic.clone(), msg.clone_box()));
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
            let mut queues = self.topic_message_queues.borrow_mut();
            if let Some(queue) = queues.get_mut(topic) {
                queue.clear();
            }
        }
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
        Self::new()
    }
}
