use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct SseEvent {
    pub id: String,
    pub data: serde_json::Value,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum EventFilter {
    ApiVersion,
    SidecarVersion,
    BlockAdded,
    TransactionAccepted,
    TransactionProcessed,
    TransactionExpired,
    Fault,
    FinalitySignature,
    Step,
}

/// The "data" field of the events sent on the event stream to clients.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum SseData {
    ApiVersion(casper_types::ProtocolVersion),
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
    pub fn type_label(&self) -> &str {
        match self {
            SseData::ApiVersion(_) => "ApiVersion",
            SseData::BlockAdded { .. } => "BlockAdded",
            SseData::TransactionAccepted(_) => "TransactionAccepted",
            SseData::TransactionProcessed { .. } => "TransactionProcessed",
            SseData::TransactionExpired { .. } => "TransactionExpired",
            SseData::Fault { .. } => "Fault",
            SseData::FinalitySignature(_) => "FinalitySignature",
            SseData::Step { .. } => "Step",
            SseData::Shutdown => "Shutdown",
        }
    }
}
