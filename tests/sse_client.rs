#[cfg(test)]
mod tests {
    use casper_sdk_rs::api::node::sse::{types::EventFilter, EventClient};
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_sse_client_connection() {
        let mut client = EventClient::default();

        client
            .connect()
            .await
            .expect("Failed to connect to SSE endpoint");

        assert!(client.event_stream.is_some());
    }

    #[test]
    fn test_on_event_adds_handler() {
        let mut client = EventClient::new("test_url");

        let test_handler = || {
            println!("Test handler called!");
        };

        let event_type = EventFilter::BlockAdded;
        client.on_event(event_type, test_handler);

        let handlers = client.event_handlers.get(&event_type).unwrap();
        assert_eq!(handlers.len(), 1);

        client.on_event(event_type, test_handler);
        let handlers = client.event_handlers.get(&event_type).unwrap();
        assert_eq!(handlers.len(), 2);
    }

    #[tokio::test]
    async fn test_run_invokes_handlers() {
        let mut server = mockito::Server::new_async().await;
        server
            .mock("GET", "/")
            .with_header("content-type", "text/event-stream")
            .with_body(concat!(
                "data: {\"ApiVersion\": \"1.5.6\"}\n\n",
                "data: {\"BlockAdded\": \"test\"}\n\n",
                "data: {\"BlockAdded\": \"test\"}\n\n",
                "data: {\"BlockAdded\": \"test\"}\n\n",
                "data: {\"TransactionProcessed\": \"test\"}\n\n",
                "data: {\"BlockAdded\": \"test\"}\n\n"
            ))
            .create_async()
            .await;

        let mut client = EventClient::new(&server.url());
        client.connect().await.unwrap();

        let block_added_count = Arc::new(Mutex::new(0));
        let tx_processed_count = Arc::new(Mutex::new(0));

        let block_added_handler = {
            let block_added_count = Arc::clone(&block_added_count);
            move || {
                let mut count = block_added_count.lock().unwrap();
                *count += 1;
                println!("Block added handler");
            }
        };

        let tx_processed_handler = {
            let tx_processed_count = Arc::clone(&tx_processed_count);
            move || {
                let mut count = tx_processed_count.lock().unwrap();
                *count += 1;
                println!("Transaction processed handler");
            }
        };

        client.on_event(EventFilter::BlockAdded, block_added_handler);
        client.on_event(EventFilter::TransactionProcessed, tx_processed_handler);

        let _result = client.run().await;

        assert_eq!(*block_added_count.lock().unwrap(), 4);
        assert_eq!(*tx_processed_count.lock().unwrap(), 1);
    }
}
