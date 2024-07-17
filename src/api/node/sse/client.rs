use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use eventsource_stream::{Event, EventStreamError, Eventsource};
use futures::stream::{BoxStream, TryStreamExt};

use super::{error::SseError, types::EventFilter, SseData};

//TODO: take it from .env file
const DEFAULT_SSE_SERVER: &str = "http://localhost:18101";
const DEFAULT_EVENT_CHANNEL: &str = "/events";

type BoxedEventStream = BoxStream<'static, Result<Event, EventStreamError<reqwest::Error>>>;

pub struct EventClient {
    pub url: String,
    pub event_stream: Option<BoxedEventStream>,
    pub event_handlers: HashMap<EventFilter, Vec<Box<dyn Fn() + Send + Sync + 'static>>>,
}

impl Default for EventClient {
    fn default() -> Self {
        let url = format!("{}{}", DEFAULT_SSE_SERVER, DEFAULT_EVENT_CHANNEL);
        Self {
            url,
            event_stream: None,
            event_handlers: HashMap::new(),
        }
    }
}

impl EventClient {
    pub fn new(url: &str) -> Self {
        EventClient {
            url: url.to_string(),
            event_stream: None,
            event_handlers: HashMap::new(),
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

    pub fn on_event<F>(&mut self, event_type: EventFilter, handler: F) -> usize
    where
        F: Fn() + 'static + Send + Sync,
    {
        let boxed_handler = Box::new(handler);
        let handlers = self.event_handlers.entry(event_type).or_default();
        handlers.push(boxed_handler);
        //TODO: we need proper, unique ids and probably additional map in the client for storing it
        handlers.len()
    }

    pub fn remove_handler(&self, event_type: EventFilter, id: usize) {
        unimplemented!()
    }

    //TODO: do we need to look for any registered handlers in this function? Not sure what is the relation between this and run function.
    pub async fn wait_for_event<F>(
        &mut self,
        event_type: EventFilter,
        predicate: F,
        timeout: Duration,
    ) -> Result<SseData, SseError>
    where
        F: Fn(&SseData) -> bool + Send + Sync,
    {
        let start_time = Instant::now();
        loop {
            if Instant::now().duration_since(start_time) > timeout {
                return Err(SseError::Timeout);
            }

            // Await for next event
            if let Some(event) = self
                .event_stream
                .as_mut()
                .ok_or(SseError::NotConnected)?
                .try_next()
                .await?
            {
                let data: SseData = serde_json::from_str(&event.data)?;

                if data.event_type() == event_type && predicate(&data) {
                    return Ok(data); //matching event
                }
            } else {
                return Err(SseError::StreamExhausted);
            }
        }
    }

    pub async fn run(&mut self) -> Result<(), SseError> {
        // Ensure the client is connected
        let mut event_stream = self.event_stream.take().ok_or(SseError::NotConnected)?;

        while let Some(event) = event_stream.try_next().await? {
            let data: SseData = serde_json::from_str(&event.data)?;

            match data {
                SseData::ApiVersion(_) => return Err(SseError::UnexpectedHandshake), // Should only happen once at connection
                SseData::Shutdown => return Err(SseError::NodeShutdown),

                // For each type, find and invoke registered handlers
                event => {
                    if let Some(handlers) = self.event_handlers.get_mut(&event.event_type()) {
                        for handler in handlers {
                            handler(); // Invoke each handler for the event
                        }
                    }
                }
            }
        }
        // Stream was exhausted.
        Err(SseError::StreamExhausted)
    }
}
