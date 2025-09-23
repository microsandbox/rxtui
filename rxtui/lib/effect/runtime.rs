use super::Effect;
use crate::component::ComponentId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::runtime::{Handle, Runtime};
use tokio::task::JoinHandle;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// Runtime for managing async effects
pub struct EffectRuntime {
    /// Tokio runtime handle for executing futures
    /// Either owns a runtime or uses existing one
    runtime_handle: RuntimeHandle,

    /// Track active effects by component ID for cleanup
    active: Arc<RwLock<HashMap<ComponentId, Vec<JoinHandle<()>>>>>,
}

enum RuntimeHandle {
    /// We own the runtime (created when not in async context)
    Owned(Runtime),
    /// Reference to existing runtime (when already in async context)
    Existing(Handle),
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl EffectRuntime {
    /// Create a new effect runtime
    pub fn new() -> Self {
        // Try to get existing runtime handle first
        let runtime_handle = Handle::try_current()
            .map(RuntimeHandle::Existing)
            .unwrap_or_else(|_| {
                // No existing runtime, create a new one
                RuntimeHandle::Owned(Runtime::new().expect("Failed to create tokio runtime"))
            });

        Self {
            runtime_handle,
            active: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get the runtime handle for spawning tasks
    fn handle(&self) -> &Handle {
        match &self.runtime_handle {
            RuntimeHandle::Owned(runtime) => runtime.handle(),
            RuntimeHandle::Existing(handle) => handle,
        }
    }

    /// Spawn effects for a component
    /// The caller is responsible for tracking whether effects should be spawned
    pub fn spawn(&self, component_id: ComponentId, effects: Vec<Effect>) {
        if effects.is_empty() {
            return;
        }

        // Spawn new effects
        let handles: Vec<_> = effects
            .into_iter()
            .map(|effect| self.handle().spawn(effect))
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
