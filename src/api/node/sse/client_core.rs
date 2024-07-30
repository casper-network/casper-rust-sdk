use super::{
    error::ClientError,
    types::{BoxedEventStream, CoreCommand, EventType, Handler},
    SseData,
};
use eventsource_stream::{Event, Eventsource};
use futures::stream::TryStreamExt;
use std::collections::HashMap;

pub struct ClientCore {
    url: String,
    event_stream: Option<BoxedEventStream>,
    next_handler_id: u64,
    event_handlers: HashMap<EventType, HashMap<u64, Box<dyn Fn(SseData) + Send + Sync + 'static>>>,
    id_types: HashMap<u64, EventType>,
    is_connected: bool,
}

impl ClientCore {
    pub async fn new(url: &str) -> Self {
        ClientCore {
            url: url.to_string(),
            event_stream: None,
            next_handler_id: 0,
            event_handlers: HashMap::new(),
            id_types: HashMap::new(),
            is_connected: false,
        }
    }

    pub async fn connect(&mut self) -> Result<(), ClientError> {
        // Connect to SSE endpoint.
        let client = reqwest::Client::new();
        let response = client.get(&self.url).send().await?;

        let stream = response.bytes_stream();
        let mut event_stream = stream.eventsource();

        // Handle the handshake with API version.
        let handshake_event = event_stream
            .try_next()
            .await?
            .ok_or(ClientError::StreamExhausted)?;
        let handshake_data: SseData = serde_json::from_str(&handshake_event.data)?;
        let _api_version = match handshake_data {
            SseData::ApiVersion(v) => Ok(v),
            _ => Err(ClientError::InvalidHandshake),
        }?;

        // Wrap stream with box and store it.
        let boxed_event_stream = Box::pin(event_stream);
        self.event_stream = Some(boxed_event_stream);
        self.is_connected = true;

        Ok(())
    }

    pub fn remove_handler(&mut self, id: u64) -> bool {
        if let Some(event_type) = self.id_types.get(&id) {
            match self.event_handlers.get_mut(&event_type) {
                Some(handlers_for_type) => handlers_for_type.remove(&id).is_some(),
                None => false,
            }
        } else {
            false //not found
        }
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub fn handle_event(&mut self, event: Event) -> Result<(), ClientError> {
        let data: SseData = serde_json::from_str(&event.data)?;

        match data {
            SseData::ApiVersion(_) => return Err(ClientError::UnexpectedHandshake), // Should only happen once at connection
            SseData::Shutdown => return Err(ClientError::NodeShutdown),

            // For each type, find and invoke registered handlers
            event => {
                if let Some(handlers) = self.event_handlers.get_mut(&event.event_type()) {
                    for handler in handlers.values() {
                        handler(event.clone()); // Invoke each handler for the event
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn run_once(&mut self) -> Result<Option<Event>, ClientError> {
        if let Some(stream) = self.event_stream.as_mut() {
            match stream.try_next().await {
                Ok(Some(event)) => Ok(Some(event)),
                Ok(None) => Err(ClientError::StreamExhausted),
                Err(err) => Err(ClientError::EventStreamError(err)),
            }
        } else {
            Err(ClientError::NoEventStreamAvailable)
        }
    }

    pub fn add_on_event_handler(&mut self, event_type: EventType, handler: Box<Handler>) -> u64 {
        let handlers = self.event_handlers.entry(event_type).or_default();
        let handler_id = self.next_handler_id;
        handlers.insert(handler_id, handler);
        self.id_types.insert(handler_id, event_type);
        self.next_handler_id += 1;
        handler_id
    }

    pub async fn handle_command(&mut self, command: CoreCommand) -> Result<(), ClientError> {
        match command {
            CoreCommand::AddOnEventHandler(event_type, callback, completion_ack) => {
                let event_id = self.add_on_event_handler(event_type, callback);
                completion_ack
                    .send(event_id)
                    .map_err(|_| ClientError::ReciverDroppedError())?;
            }
            CoreCommand::Connect(completion_ack) => {
                self.connect().await.map_err(ClientError::from)?;
                completion_ack
                    .send(())
                    .map_err(|_| ClientError::ReciverDroppedError())?;
            }
            CoreCommand::RemoveEventHandler(id, completion_ack) => {
                let removed = self.remove_handler(id);
                completion_ack
                    .send(removed)
                    .map_err(|_| ClientError::ReciverDroppedError())?;
            }
        }
        Ok(())
    }
}
