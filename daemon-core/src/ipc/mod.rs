//! IPC broker interfaces and protocol configuration abstractions.

use crate::errors::Result;
use std::path::Path;

/// Abstract definition of IPC channel brokers.
pub trait IpcBroker {
    /// Bind the IPC broker listener path.
    ///
    /// # Errors
    ///
    /// Returns an error if socket binding or creation fails.
    fn bind<P: AsRef<Path>>(&self, path: P) -> Result<()>;

    /// Send a message to a recipient module.
    ///
    /// # Errors
    ///
    /// Returns an error if transmission fails.
    fn send_message(&self, recipient: &str, payload: &[u8]) -> Result<LayoutMessage>;
}

/// Representation of transmission payload details inside the IPC broker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMessage {
    /// Sender identifier.
    pub sender: String,
    /// Message byte contents.
    pub payload: Vec<u8>,
}
