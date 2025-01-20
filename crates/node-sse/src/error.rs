use super::typeset::EventData;
use eventsource_client::Error as EventStreamError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Failed to connect to SSE endpoint: {0}")]
    ConnectionError(#[from] reqwest::Error),

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

    #[error("Failed to send command to core: {0}")]
    CommandSendError(#[from] tokio::sync::mpsc::error::SendError<CoreCommand>),

    #[error("Failed to send ack to client")]
    ReciverDroppedError(),

    #[error("Failed to recive command from core: {0}")]
    CommandRecvError(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("Failed to send Event into the channel: {0}")]
    ChannelInternalError(#[from] tokio::sync::mpsc::error::TrySendError<EventData>),

    #[error("Error reading from event stream:{0}")]
    EventStreamError(#[from] EventStreamError),

    #[error("No event stream available")]
    NoEventStreamAvailable,
}
