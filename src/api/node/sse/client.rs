use std::any::Any;

use eventsource_stream::{Event, EventStreamError, Eventsource};
use futures::stream::{BoxStream, TryStreamExt};
use tokio::time::Timeout;

use super::{error::SseError, types::EventFilter, SseData};

const DEFAULT_SSE_SERVER: &str = "http://localhost:18101";
const DEFAULT_EVENT_CHANNEL: &str = "/events";

type BoxedEventStream = BoxStream<'static, Result<Event, EventStreamError<reqwest::Error>>>;

pub struct CasperEventClient {
    pub url: String,
    pub event_stream: Option<BoxedEventStream>,
}

impl Default for CasperEventClient {
    fn default() -> Self {
        let url = format!("{}{}", DEFAULT_SSE_SERVER, DEFAULT_EVENT_CHANNEL);
        Self {
            url,
            event_stream: None,
        }
    }
}

impl CasperEventClient {
    pub fn new(url: &str) -> Self {
        CasperEventClient {
            url: url.to_string(),
            event_stream: None,
        }
    }

    pub async fn connect(&mut self) -> Result<(), SseError> {
        // Connect to SSE endpoint.
        let client = reqwest::Client::new();
        let response = client.get(&self.url).send().await?;

        let stream = response.bytes_stream();
        let mut event_stream = stream.eventsource();

        // Handle the handshake with API version.
        let handshake_event = event_stream
            .try_next()
            .await?
            .ok_or(SseError::StreamExhausted)?;
        let handshake_data: SseData = serde_json::from_str(&handshake_event.data)?;
        let _api_version = match handshake_data {
            SseData::ApiVersion(v) => Ok(v),
            _ => Err(SseError::InvalidHandshake),
        }?;

        // Wrap stream with box and store it.
        let boxed_event_stream = Box::pin(event_stream);
        self.event_stream = Some(boxed_event_stream);

        Ok(())
    }

    // Wait for a specific event type (no timeout)
    pub async fn wait_for_event(
        &mut self,
        event_type: EventFilter,
        // timeout: u64,
        // from_id: u64,
    ) -> Result<SseData, SseError> {
        // Take stream out of state.
        let mut event_stream = match self.event_stream.take() {
            Some(s) => Ok(s),
            None => Err(SseError::NotConnected),
        }?;
        //TODO: add a check for a specific event type, if there is a match return the event from the function
        while let Some(event) = event_stream.try_next().await? {
            let data: SseData = serde_json::from_str(&event.data)?;
            if data.type_label() == "BlockAdded" {
                return Ok(data);
            }
            // match data {
            //     received_event if received_event.type_id() == event_type.type_id() => {
            //         return Ok(received_event);
            //     }
            //     SseData::ApiVersion(_) => Err(SseError::UnexpectedHandshake)?,
            //     SseData::BlockAdded(_) => {}
            //     SseData::DeployAccepted(_) => {}
            //     SseData::DeployProcessed(_) => {}
            //     SseData::Fault(_) => {}
            //     SseData::FinalitySignature(_) => {}
            //     SseData::DeployExpired(_) => {}
            //     SseData::Step(_) => {}
            //     SseData::Shutdown => Err(SseError::NodeShutdown)?,
            // }
        }

        // Stream was exhausted.
        Err(SseError::StreamExhausted)?
    }
}
