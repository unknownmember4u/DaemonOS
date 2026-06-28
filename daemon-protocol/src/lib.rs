//! Inter-process communication protocol definitions for DaemonOS.

/// The version of the DaemonOS IPC protocol.
pub const PROTOCOL_VERSION: &str = "1.0.0";

/// Shared protocol message header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageHeader {
    /// Unique identifier for tracking message response loops.
    pub message_id: u64,
    /// Unix timestamp of when the message was sent.
    pub timestamp: u64,
}

/// Shared system status indicators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemStatus {
    /// The component is starting.
    Initializing,
    /// The component is running normally.
    Active,
    /// The component is suspended or locked.
    Idle,
    /// The component has encountered an error.
    Faulted,
}

/// Protocol IPC messages exchanged between DaemonOS components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolMessage {
    /// A diagnostic status ping.
    Ping(MessageHeader),
    /// A response verifying message reception.
    Pong(MessageHeader),
    /// An action command.
    Command {
        /// Header metadata.
        header: MessageHeader,
        /// Action instruction.
        action: String,
    },
    /// A system status change broadcast.
    StatusChanged {
        /// Header metadata.
        header: MessageHeader,
        /// Component originating the status change.
        sender: String,
        /// New status value.
        status: SystemStatus,
    },
}
