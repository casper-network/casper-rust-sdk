use super::{types::CoreCommand, SseData};
use eventsource_stream::EventStreamError;
use thiserror::Error;

//TODO: after implementing proper mock sse revisit the errors below and remove the unnecessary ones
#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Failed to connect to SSE endpoint: {0}")]
    ConnectionError(#[from] reqwest::Error),

    #[error("Not connected to event stream")]
    NotConnected,

    #[error("SSE stream exhausted unexpectedly")]
    StreamExhausted,

    #[error("Invalid handshake event")]
    InvalidHandshake,

    #[error("Unexpected handshake event")]
    UnexpectedHandshake,

    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),

    #[error("Node shutdown")]
    NodeShutdown,

    #[error("Timeout while waiting for event")]
    Timeout,

    #[error("Invalid command received")]
    InvalidCommand,

    #[error("Failed to send command to core: {0}")]
    CommandSendError(#[from] tokio::sync::mpsc::error::SendError<CoreCommand>),

    #[error("Failed to send ack to client")]
    ReciverDroppedError(),

    #[error("Failed to recive command from core: {0}")]
    CommandRecvError(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("Event handler error")]
    EventHandlerError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("Error registering event handler")]
    RegisterEventHanbdlerError,

    #[error("Failed to send Event into the channel: {0}")]
    ChannelInternalError(#[from] tokio::sync::mpsc::error::TrySendError<SseData>),

    #[error("Error reading from event stream:{0}")]
    EventStreamError(#[from] EventStreamError<reqwest::Error>),

    #[error("No event stream available")]
    NoEventStreamAvailable,
}
