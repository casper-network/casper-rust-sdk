use super::typeset::EventData;
use eventsource_client::Error as EventStreamError;
use reqwest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Failed to send command to core: {0}")]
    CommandSendError(#[from] tokio::sync::mpsc::error::SendError<CoreCommand>),

    #[error("Failed to recive command from core: {0}")]
    CommandRecvError(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("Failed to connect to SSE endpoint: {0}")]
    ConnectionError(#[from] reqwest::Error),

    #[error("Failed to send Event into the channel: {0}")]
    EventChannelInternalError(#[from] tokio::sync::mpsc::error::TrySendError<EventData>),

    #[error("Error reading from event stream:{0}")]
    EventStreamError(#[from] EventStreamError),

    #[error("SSE stream exhausted unexpectedly")]
    EventStreamExhaustedError,

    #[error("No event stream available")]
    EventStreamUnavailableError,

    #[error("Invalid handshake event")]
    InvalidHandshakeError,

    #[error("Deserialization error: {0}")]
    JsonDecodingError(#[from] serde_json::Error),

    #[error("Node shutdown")]
    NodeShutdownError,

    #[error("Failed to send ack to client")]
    ReciverDroppedError(),

    #[error("Unexpected handshake event")]
    UnexpectedHandshakeError,
}
