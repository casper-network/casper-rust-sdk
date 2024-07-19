#[cfg(test)]
mod tests {
    use casper_sdk_rs::api::node::sse::{types::EventType, Client, EventInfo};
    use core::panic;
    use mockito::ServerGuard;
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

    const EVENT_STREAM: &str = concat!(
        "data: {\"ApiVersion\": \"2.0.0\"}\n\n",
        "data: {\"BlockAdded\": {\"height\":10}}\n\n",
        "data: {\"BlockAdded\": {\"height\":11}}\n\n",
        "data: {\"BlockAdded\": {\"height\":12}}\n\n",
        "data: {\"TransactionProcessed\": \"test\"}\n\n",
        "data: {\"BlockAdded\": {\"height\":13}}\n\n",
        "data: {\"BlockAdded\": {\"height\":14}}\n\n",
    );

    async fn create_mock_server(event_stream: Option<&str>) -> ServerGuard {
        let event_stream = match event_stream {
            Some(e_stream) => e_stream,
            None => EVENT_STREAM,
        };
        let mut server = mockito::Server::new_async().await;
        server
            .mock("GET", "/")
            .with_header("content-type", "text/event-stream")
            .with_body(event_stream)
            .create_async()
            .await;
        server
    }

    #[tokio::test]
    async fn test_client_connection() {
        let server = create_mock_server(None).await;
        let mut client = Client::new(&server.url());

        client
            .connect()
            .await
            .expect("Failed to connect to SSE endpoint");

        assert!(client.event_stream.is_some());
    }

    #[test]
    fn test_on_event_add_handlers() {
        let mut client = Client::new("test_url");

        let test_handler = || {
            println!("Test handler called!");
        };

        let event_type = EventType::BlockAdded;
        let mut handler_id = client.on_event(event_type, test_handler);

        let mut handlers = client.event_handlers.get(&event_type).unwrap();
        assert_eq!(handlers.len(), 1);
        assert!(handlers.contains_key(&handler_id));

        handler_id = client.on_event(event_type, test_handler);
        handlers = client.event_handlers.get(&event_type).unwrap();
        assert_eq!(handlers.len(), 2);
        assert!(handlers.contains_key(&handler_id));
    }

    #[test]
    fn test_remove_handler() {
        let mut client = Client::new("test_url");

        let test_handler = || {
            println!("Test handler called!");
        };

        let event_type = EventType::BlockAdded;
        let mut handler_id = client.on_event(event_type, test_handler);

        let mut handlers = client.event_handlers.get(&event_type).unwrap();
        assert_eq!(handlers.len(), 1);
        assert!(handlers.contains_key(&handler_id));

        handler_id = client.on_event(event_type, test_handler);
        handlers = client.event_handlers.get(&event_type).unwrap();
        assert_eq!(handlers.len(), 2);
        assert!(handlers.contains_key(&handler_id));

        let removed = client.remove_handler(event_type, handler_id);

        assert!(removed, "Handler should have been removed");
        handlers = client.event_handlers.get(&event_type).unwrap();
        assert_eq!(handlers.len(), 1);
        assert!(
            !handlers.contains_key(&handler_id),
            "Handler should have been removed"
        );
    }

    #[tokio::test]
    async fn test_run_invokes_handlers() {
        let server = create_mock_server(None).await;
        let mut client = Client::new(&server.url());
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

        client.on_event(EventType::BlockAdded, block_added_handler);
        client.on_event(EventType::TransactionProcessed, tx_processed_handler);

        let _result = client.run().await;

        assert_eq!(*block_added_count.lock().unwrap(), 5);
        assert_eq!(*tx_processed_count.lock().unwrap(), 1);
    }

    #[tokio::test]
    async fn test_wait_for_event_success() {
        let server = create_mock_server(None).await;
        let mut client = Client::new(&server.url());
        client.connect().await.unwrap();

        let predicate = |data: &EventInfo| {
            if let EventInfo::BlockAdded(block_data) = data {
                if let Some(height) = block_data["height"].as_u64() {
                    return height == 13;
                }
            }
            false
        };

        let timeout = Duration::from_secs(5);
        let result = client
            .wait_for_event(EventType::BlockAdded, predicate, timeout)
            .await;

        match result {
            Ok(EventInfo::BlockAdded(block_data)) => {
                assert_eq!(block_data["height"].as_u64().unwrap(), 13);
            }
            Ok(other_event) => panic!("Expected BlockAdded, got {:?}", other_event),
            Err(err) => panic!("Unexpected error: {}", err),
        }
    }
}
