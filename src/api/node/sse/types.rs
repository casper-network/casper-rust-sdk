use serde::{Deserialize, Serialize};

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
pub enum EventInfo {
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

impl EventInfo {
    pub fn event_type(&self) -> EventType {
        match self {
            EventInfo::ApiVersion(_) => EventType::ApiVersion,
            EventInfo::SidecarVersion(_) => EventType::Other,
            EventInfo::BlockAdded(_) => EventType::BlockAdded,
            EventInfo::TransactionAccepted(_) => EventType::TransactionAccepted,
            EventInfo::TransactionProcessed(_) => EventType::TransactionProcessed,
            EventInfo::TransactionExpired(_) => EventType::TransactionExpired,
            EventInfo::Fault(_) => EventType::Fault,
            EventInfo::FinalitySignature(_) => EventType::FinalitySignature,
            EventInfo::Step(_) => EventType::Step,
            EventInfo::Shutdown => EventType::Shutdown,
        }
    }
}
