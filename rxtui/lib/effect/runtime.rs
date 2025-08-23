use super::Effect;
use crate::component::ComponentId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Runtime for managing async effects
pub struct EffectRuntime {
    /// Tokio runtime for executing futures
    runtime: Runtime,

    /// Track active effects by component ID for cleanup
    active: Arc<RwLock<HashMap<ComponentId, Vec<JoinHandle<()>>>>>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl EffectRuntime {
    /// Create a new effect runtime
    pub fn new() -> Self {
        // Create tokio runtime with a small thread pool
        let runtime = Runtime::new().expect("Failed to create tokio runtime");

        Self {
            runtime,
            active: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Spawn effects for a component
    /// This will cancel any existing effects for the component first
    pub fn spawn(&self, component_id: ComponentId, effects: Vec<Effect>) {
        // Cancel existing effects for this component
        self.cleanup(&component_id);

        if effects.is_empty() {
            return;
        }

        // Spawn new effects
        let handles: Vec<_> = effects
            .into_iter()
            .map(|effect| self.runtime.spawn(effect))
            .collect();

        // Track handles for cleanup
        self.active.write().unwrap().insert(component_id, handles);
    }

    /// Cancel all effects for a component
    pub fn cleanup(&self, component_id: &ComponentId) {
        if let Some(handles) = self.active.write().unwrap().remove(component_id) {
            // Abort all tasks for this component
            for handle in handles {
                handle.abort();
            }
        }
    }

    /// Cleanup all effects (used on shutdown)
    pub fn cleanup_all(&self) {
        let mut active = self.active.write().unwrap();
        for (_, handles) in active.drain() {
            for handle in handles {
                handle.abort();
            }
        }
    }

    /// Check if a component has active effects
    pub fn has_effects(&self, component_id: &ComponentId) -> bool {
        self.active.read().unwrap().contains_key(component_id)
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for EffectRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for EffectRuntime {
    fn drop(&mut self) {
        // Cleanup all effects when runtime is dropped
        self.cleanup_all();
    }
}
