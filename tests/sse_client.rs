#[cfg(test)]
mod tests {
    use casper_sdk_rs::api::node::sse::{client::Client, types::EventType, ClientCore, SseData};
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
        "data: {\"FinalitySignature\": \"test\"}\n\n",
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
        let client = Client::new(&server.url()).await;

        client
            .connect()
            .await
            .expect("Failed to connect to SSE endpoint");
    }

    #[tokio::test]
    async fn test_on_event_add_handlers() {
        let mut client = ClientCore::new("test_url").await;

        let test_handler = Box::new(|_event_info| {
            println!("Test handler called!");
        });

        let event_type = EventType::BlockAdded;
        let mut handler_id = client.add_on_event_handler(event_type, test_handler.clone());
        assert_eq!(handler_id, 0);
        let mut handlers = client.event_handlers.get(&event_type).unwrap();
        assert!(
            client.id_types.contains_key(&handler_id),
            "Failed to add to id_types map"
        );
        assert_eq!(handlers.len(), 1);
        assert!(handlers.contains_key(&handler_id));
        handler_id = client.add_on_event_handler(event_type, test_handler);
        assert_eq!(handler_id, 1);
        handlers = client.event_handlers.get(&event_type).unwrap();
        assert_eq!(handlers.len(), 2);
        assert!(handlers.contains_key(&handler_id));
    }

    #[tokio::test]
    async fn test_remove_handler() {
        let mut client = ClientCore::new("test_url").await;

        let test_handler = Box::new(|_event_info| {
            println!("Test handler called!");
        });

        let event_type = EventType::BlockAdded;
        let mut handler_id = client.add_on_event_handler(event_type, test_handler.clone());

        let mut handlers = client.event_handlers.get(&event_type).unwrap();
        assert_eq!(handlers.len(), 1);
        assert!(handlers.contains_key(&handler_id));

        handler_id = client.add_on_event_handler(event_type, test_handler);
        handlers = client.event_handlers.get(&event_type).unwrap();
        assert_eq!(handlers.len(), 2);
        assert!(handlers.contains_key(&handler_id));

        let removed = client.remove_handler(handler_id);

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
        let mut client = Client::new(&server.url()).await;

        let block_added_count = Arc::new(Mutex::new(0));
        let finality_signature_count = Arc::new(Mutex::new(0));

        let block_added_handler = {
            let block_added_count = Arc::clone(&block_added_count);
            move |_event_info| {
                let mut count = block_added_count.lock().unwrap();
                *count += 1;
                println!("Block added handler");
            }
        };

        let finality_signature_handler = {
            let finality_signature_count = Arc::clone(&finality_signature_count);
            move |_event_info| {
                let mut count = finality_signature_count.lock().unwrap();
                *count += 1;
                println!("Finality Signature handler");
            }
        };

        let _handler_id = client
            .on_event(EventType::BlockAdded, block_added_handler)
            .await;

        let _handler_id = client
            .on_event(EventType::FinalitySignature, finality_signature_handler)
            .await;

        client.connect().await.unwrap();
        assert_eq!(*block_added_count.lock().unwrap(), 5);
        assert_eq!(*finality_signature_count.lock().unwrap(), 1);
    }

    #[ignore = "this will not work unitl proper sse mock is implemented"]
    #[tokio::test]
    async fn test_wait_for_event_success() {
        let server = create_mock_server(None).await;
        let mut client = Client::new(&server.url()).await;

        let predicate = |data: SseData| {
            if let SseData::BlockAdded(block_data) = data {
                if let Some(height) = block_data["height"].as_u64() {
                    return height == 13;
                }
            }
            false
        };

        client.connect().await.unwrap();

        let timeout = Duration::from_secs(2);
        let result = client
            .wait_for_event(EventType::BlockAdded, predicate, timeout)
            .await;

        match result {
            Ok(Some(SseData::BlockAdded(block_data))) => {
                assert_eq!(block_data["height"].as_u64().unwrap(), 13);
            }
            Ok(Some(event)) => panic!("Expected BlockAdded event, got {:?}", event),
            Ok(None) => panic!("Timed out waiting for event"),
            Err(err) => panic!("Unexpected error: {:?}", err),
        }
    }
}
