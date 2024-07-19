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
    Other,
}

// TODO: add full deserialization of the json
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
            EventInfo::Shutdown => EventType::Other,
        }
    }
}
