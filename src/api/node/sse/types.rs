use eventsource_stream::{Event, EventStreamError};
use futures::stream::BoxStream;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

//copied from casper-sidecar

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum EventType {
    ApiVersion,
    SidecarVersion,
    BlockAdded,
    TransactionAccepted,
    TransactionProcessed,
    TransactionExpired,
    Fault,
    FinalitySignature,
    Step,
    Shutdown,
}

/// Casper does not expose SSE types directly, so we have to reimplement them.
/// Source: https://github.com/casper-network/casper-node/blob/8a9a864212b7c20fc17e1d0106b02c813ffded9d/node/src/components/event_stream_server/sse_server.rs#L56.
/// TODO: Add full deserialization details.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum SseData {
    ApiVersion(casper_types::ProtocolVersion),
    SidecarVersion(serde_json::Value),
    BlockAdded(serde_json::Value),
    TransactionAccepted(serde_json::Value),
    TransactionProcessed(serde_json::Value),
    TransactionExpired(serde_json::Value),
    Fault(serde_json::Value),
    FinalitySignature(serde_json::Value),
    Step(serde_json::Value),
    Shutdown,
}

impl SseData {
    pub fn event_type(&self) -> EventType {
        match self {
            SseData::ApiVersion(_) => EventType::ApiVersion,
            SseData::SidecarVersion(_) => EventType::SidecarVersion,
            SseData::BlockAdded(_) => EventType::BlockAdded,
            SseData::TransactionAccepted(_) => EventType::TransactionAccepted,
            SseData::TransactionProcessed(_) => EventType::TransactionProcessed,
            SseData::TransactionExpired(_) => EventType::TransactionExpired,
            SseData::Fault(_) => EventType::Fault,
            SseData::FinalitySignature(_) => EventType::FinalitySignature,
            SseData::Step(_) => EventType::Step,
            SseData::Shutdown => EventType::Shutdown,
        }
    }
}

pub enum CoreCommand {
    Connect(oneshot::Sender<()>),
    AddOnEventHandler(EventType, Box<Handler>, oneshot::Sender<u64>),
    RemoveEventHandler(u64, oneshot::Sender<bool>),
}

pub type Handler = dyn Fn(SseData) + 'static + Send + Sync;
pub type BoxedEventStream = BoxStream<'static, Result<Event, EventStreamError<reqwest::Error>>>;
