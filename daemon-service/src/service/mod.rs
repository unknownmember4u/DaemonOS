//! Service registry and lifecycle coordination management.

use std::future::Future;
use std::pin::Pin;
use tracing::info;

/// Type alias for a boxed future that is Send.
pub type BoxedFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// Type alias for service running tasks.
pub type ServiceTask = Box<dyn FnOnce(tokio::sync::broadcast::Receiver<()>) -> BoxedFuture + Send>;

/// Orchestrates background service registration and lifecycles.
#[derive(Default)]
pub struct ServiceRegistry {
    services: Vec<(String, ServiceTask)>,
}

impl ServiceRegistry {
    /// Create a new empty ServiceRegistry.
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    /// Register a service with its launch task callback.
    pub fn register<F, Fut>(&mut self, name: &str, task: F)
    where
        F: FnOnce(tokio::sync::broadcast::Receiver<()>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let boxed_task = Box::new(move |rx| {
            let fut = task(rx);
            let boxed: BoxedFuture = Box::pin(fut);
            boxed
        });
        self.services.push((name.to_string(), boxed_task));
    }

    /// Launch all registered services, returning handles to wait for them.
    pub fn start_all(
        self,
        shutdown_tx: &tokio::sync::broadcast::Sender<()>,
    ) -> Vec<tokio::task::JoinHandle<()>> {
        let mut handles = Vec::new();
        for (name, task) in self.services {
            info!("Starting registered service: {}", name);
            let rx = shutdown_tx.subscribe();
            let handle = tokio::spawn(async move {
                task(rx).await;
                info!("Service task '{}' stopped.", name);
            });
            handles.push(handle);
        }
        handles
    }
}
