use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Event types emmitted by a node's SSE port.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EventType {
    ApiVersion,
    BlockAdded,
    Fault,
    FinalitySignature,
    Shutdown,
    SidecarVersion,
    Step,
    TransactionAccepted,
    TransactionExpired,
    TransactionProcessed,
}

// Event data associated with an SSE event.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EventData {
    ApiVersion(EventPayload),
    BlockAdded(EventPayload),
    Fault(EventPayload),
    FinalitySignature(EventPayload),
    Shutdown,
    SidecarVersion(EventPayload),
    Step(EventPayload),
    TransactionAccepted(EventPayload),
    TransactionExpired(EventPayload),
    TransactionProcessed(EventPayload),
}

// Payload associated with event data.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EventPayload {
    Json(JsonValue),
    Binary(Vec<u8>),
}

// Supported payload codecs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EventPayloadCodec {
    Json,
    Binary,
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl EventData {
    pub fn event_type(&self) -> EventType {
        EventType::from(self)
    }
}

// ------------------------------------------------------------------------
// Traits.
// ------------------------------------------------------------------------

impl From<&EventData> for EventType {
    fn from(value: &EventData) -> Self {
        match value {
            EventData::ApiVersion(_) => EventType::ApiVersion,
            EventData::SidecarVersion(_) => EventType::SidecarVersion,
            EventData::BlockAdded(_) => EventType::BlockAdded,
            EventData::TransactionAccepted(_) => EventType::TransactionAccepted,
            EventData::TransactionProcessed(_) => EventType::TransactionProcessed,
            EventData::TransactionExpired(_) => EventType::TransactionExpired,
            EventData::Fault(_) => EventType::Fault,
            EventData::FinalitySignature(_) => EventType::FinalitySignature,
            EventData::Step(_) => EventType::Step,
            EventData::Shutdown => EventType::Shutdown,
        }
    }
}
