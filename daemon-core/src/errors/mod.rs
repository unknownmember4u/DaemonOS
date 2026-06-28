//! Typed error system for the DaemonOS core library.

use thiserror::Error;

/// Core error enumeration covering config, systems, IPC, and event bus failures.
#[derive(Error, Debug)]
pub enum CoreError {
    /// Error parsing, validating, or finding configurations.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Standard input/output related errors.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Inter-process communication broker error.
    #[error("IPC error: {0}")]
    Ipc(String),

    /// Internal system or tracing initialization failures.
    #[error("System error: {0}")]
    System(String),

    /// Event publishing or subscription errors.
    #[error("Event bus error: {0}")]
    Event(String),

    /// Serialization/Deserialization formatting error.
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Convenience type alias for Result in daemon-core.
pub type Result<T> = std::result::Result<T, CoreError>;
