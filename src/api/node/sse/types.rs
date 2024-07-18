use serde::{Deserialize, Serialize};

//copied from casper-sidecar

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
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
    Other,
}

// TODO: add full deserialization of the json
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
    pub fn event_type(&self) -> EventFilter {
        match self {
            SseData::ApiVersion(_) => EventFilter::ApiVersion,
            SseData::SidecarVersion(_) => EventFilter::Other,
            SseData::BlockAdded(_) => EventFilter::BlockAdded,
            SseData::TransactionAccepted(_) => EventFilter::TransactionAccepted,
            SseData::TransactionProcessed(_) => EventFilter::TransactionProcessed,
            SseData::TransactionExpired(_) => EventFilter::TransactionExpired,
            SseData::Fault(_) => EventFilter::Fault,
            SseData::FinalitySignature(_) => EventFilter::FinalitySignature,
            SseData::Step(_) => EventFilter::Step,
            SseData::Shutdown => EventFilter::Other,
        }
    }
}
