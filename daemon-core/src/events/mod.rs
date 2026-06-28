//! Event bus definition providing pub/sub architecture patterns.

use crate::errors::{CoreError, Result};
use std::sync::{Arc, Mutex};

/// Representation of an event payload propagated through the event bus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// Target topic channel identifier.
    pub topic: String,
    /// Message payload details.
    pub payload: String,
}

/// Callback function signature for event listeners.
pub type EventCallback = Arc<dyn Fn(&Event) + Send + Sync + 'static>;

/// Thread-safe event bus skeleton implementing publish/subscribe pathways.
#[derive(Default, Clone)]
pub struct EventBus {
    subscribers: Arc<Mutex<Vec<(String, EventCallback)>>>,
}

impl EventBus {
    /// Create a new empty EventBus registry.
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Subscribe to a specific topic or use "*" for wildcard listening.
    pub fn subscribe<F>(&self, topic: &str, callback: F)
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        if let Ok(mut subs) = self.subscribers.lock() {
            subs.push((topic.to_string(), Arc::new(callback)));
        }
    }

    /// Publish an event to all subscribers matching the topic filter.
    ///
    /// # Errors
    ///
    /// Returns a `CoreError::Event` if the subscribers lock is poisoned.
    pub fn publish(&self, event: Event) -> Result<()> {
        let subs = {
            let lock = self
                .subscribers
                .lock()
                .map_err(|e| CoreError::Event(format!("Mutex poisoned during publish: {}", e)))?;
            lock.clone()
        };

        for (topic, callback) in subs {
            if topic == event.topic || topic == "*" {
                callback(&event);
            }
        }
        Ok(())
    }
}
