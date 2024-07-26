use super::{
    error::ClientError,
    types::{CoreCommand, EventType},
    ClientCore, SseData,
};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};

pub struct Client {
    command_sender: mpsc::Sender<CoreCommand>,
}

impl Client {
    pub async fn new(url: &str) -> Self {
        let client_core = ClientCore::new(url).await;

        let (tx, rx) = mpsc::channel(32);
        //TODO: not sure how to handle these errors from run_client_core
        let _handle = tokio::spawn(async move {
            let _ = run_client_core(rx, client_core).await;
        });

        Client { command_sender: tx }
    }

    pub async fn connect(&self) -> Result<(), ClientError> {
        let (tx, rx) = oneshot::channel();
        self.command_sender
            .send(CoreCommand::Connect(tx))
            .await
            .map_err(|err| ClientError::CommandSendError(err))?;
        rx.await.map_err(|err| ClientError::CommandRecvError(err))
    }

    pub async fn on_event<F>(
        &mut self,
        event_type: EventType,
        handler: F,
    ) -> Result<u64, ClientError>
    where
        F: Fn(SseData) + 'static + Send + Sync,
    {
        let (tx, rx) = oneshot::channel();
        self.command_sender
            .send(CoreCommand::AddOnEventHandler(
                event_type,
                Box::new(handler),
                tx,
            ))
            .await
            .map_err(|err| ClientError::CommandSendError(err))?;
        rx.await.map_err(|err| ClientError::CommandRecvError(err))
    }

    pub async fn wait_for_event<F>(
        &mut self,
        event_type: EventType,
        predicate: F,
        timeout: Duration,
    ) -> Result<Option<SseData>, ClientError>
    where
        F: Fn(SseData) -> bool + Send + Sync + 'static,
    {
        let (tx, mut rx) = mpsc::channel(1);

        // Register the event handler
        let handler_id = self
            .on_event(event_type, move |event_info: SseData| {
                if predicate(event_info.clone()) {
                    // Send the matching event to the channel
                    let _ = tx
                        .try_send(event_info)
                        .map_err(|err| ClientError::ChannelInternalError(err));
                }
            })
            .await?;

        // Wait for the event or timeout
        let result = if timeout.is_zero() {
            rx.recv().await
        } else {
            tokio::time::timeout(timeout, rx.recv())
                .await
                .ok()
                .flatten()
        };

        // Remove the event handler after the event is received or timeout occurs
        self.remove_handler(handler_id).await?;

        match result {
            Some(event_info) => Ok(Some(event_info)),
            None => {
                eprintln!("Timed out or stream exhausted while waiting for event");
                Ok(None)
            }
        }
    }

    pub async fn remove_handler(&mut self, id: u64) -> Result<bool, ClientError> {
        let (tx, rx) = oneshot::channel();
        self.command_sender
            .send(CoreCommand::RemoveEventHandler(id, tx))
            .await
            .map_err(|err| ClientError::CommandSendError(err))?;
        rx.await.map_err(|err| ClientError::CommandRecvError(err))
    }
}

/// Handles incoming commands and delegates tasks to ClientCore.
async fn run_client_core(
    mut rx: mpsc::Receiver<CoreCommand>,
    mut client_core: ClientCore,
) -> Result<(), ClientError> {
    loop {
        if !client_core.is_connected {
            // Not connected yet, so only process Connect commands.
            if let Some(command) = rx.recv().await {
                client_core.handle_command(command).await?
            }
        } else {
            tokio::select! {
                Ok(Some(event)) = client_core.run_once() => {
                    client_core.handle_event(event)?;
                },
                Some(command) = rx.recv() => {
                      client_core.handle_command(command)
                            .await?
                },
            }
        }
    }
}
