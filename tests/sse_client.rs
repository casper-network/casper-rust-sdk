#[cfg(test)]
mod utils;
mod tests {
    use crate::utils::MockSse;
    use casper_sdk_rs::api::node::sse::error::ClientError;
    use casper_sdk_rs::api::node::sse::{client::Client, types::EventType, ClientCore, SseData};
    use casper_types::ProtocolVersion;
    use core::panic;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio::sync::mpsc;

    /*
     *client_core tests
     */

    #[tokio::test]
    async fn test_client_core_connect_and_handshake() {
        let mock_server = MockSse::start().await;
        let mut client_core = ClientCore::new(&mock_server.url()).await;

        // Test successful connection
        let result = client_core.connect().await;
        assert!(result.is_ok(), "Connection should succeed");
        assert!(client_core.is_connected(), "Should be marked as connected");

        mock_server
            .send_event(SseData::BlockAdded(serde_json::json!({
                "height": 1,
            })))
            .await
            .unwrap();

        let event = client_core.run_once().await.expect("Should get event");
        let event = event.expect("Event should not be None");
        assert!(event.data.contains("BlockAdded"));
        match client_core.handle_event(event) {
            Ok(()) => (), // Success
            Err(err) => panic!("Unexpected error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_client_core_double_handshake() {
        let mock_server = MockSse::start().await;
        let mut client_core = ClientCore::new(&mock_server.url()).await;

        client_core.connect().await.unwrap();

        let handshake = SseData::ApiVersion(ProtocolVersion::from_parts(2, 0, 0));
        mock_server.send_event(handshake.clone()).await.unwrap();
        let event = client_core.run_once().await.expect("Should get event");
        let event = event.expect("Event should not be None");
        match client_core.handle_event(event) {
            Ok(()) => panic!("Expected error"),
            Err(ClientError::UnexpectedHandshake) => (), // Success
            Err(err) => panic!("Unexpected error: {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_client_core_add_on_event_handler_remove_on_event_handler() {
        let mock_server = MockSse::start().await;
        let mut client_core = ClientCore::new(&mock_server.url()).await;

        client_core.connect().await.unwrap();

        let handler_invoked = Arc::new(Mutex::new(false));

        let flipper_handler = {
            let handler_invoked = Arc::clone(&handler_invoked);
            move |event_info: SseData| {
                let mut handler_invoked = handler_invoked.lock().unwrap();
                *handler_invoked = !*handler_invoked;
                println!("🏃 Running Handler: {:?}", event_info.event_type());
            }
        };

        let event_type = EventType::BlockAdded;
        let handler_id = client_core.add_on_event_handler(event_type, Box::new(flipper_handler));

        assert!(handler_id == 0, "Handler ID should be assigned");

        // Test 1: Handler invocation
        let block_added_event = SseData::BlockAdded(serde_json::json!({
            "height": 1,
        }));
        mock_server
            .send_event(block_added_event.clone())
            .await
            .unwrap();

        let event = client_core.run_once().await.unwrap().unwrap();
        client_core.handle_event(event).unwrap();

        assert!(
            *handler_invoked.lock().unwrap(),
            "Handler should have been called once"
        );

        // Test 2: Second invocation flips the flag back
        mock_server
            .send_event(block_added_event.clone())
            .await
            .unwrap();

        let event = client_core.run_once().await.unwrap().unwrap();
        client_core.handle_event(event).unwrap();
        assert!(
            !*handler_invoked.lock().unwrap(),
            "Handler should have been called twice (and flipped back)"
        );

        // Test 3: Removal of the handler, the flag should not be changed by new events
        let res = client_core.remove_handler(handler_id);
        assert!(res);

        mock_server.send_event(block_added_event).await.unwrap();

        let event = client_core.run_once().await.unwrap().unwrap();
        client_core.handle_event(event).unwrap();
        assert!(
            !*handler_invoked.lock().unwrap(),
            "Handler should not be called after removal"
        );
    }

    /*
     *client tests
     */

    #[tokio::test]
    async fn test_client_connect() {
        let mock_server = MockSse::start().await;
        let client = Client::new(&mock_server.url()).await;
        let result = client.connect().await;
        assert!(result.is_ok(), "Client should connect successfully");
    }

    #[tokio::test]
    async fn test_client_on_event() {
        let mock_server = MockSse::start().await;
        let mut client = Client::new(&mock_server.url()).await;

        let (tx_block_added, mut rx_block_added) = mpsc::channel(1); // Channel for BlockAdded events
        let (tx_tx_processed, mut rx_tx_processed) = mpsc::channel(1); // Channel for TransactionProcessed events

        let block_added_handler = move |event: SseData| {
            tx_block_added.try_send(event).unwrap();
        };

        let tx_processed_handler = move |event: SseData| {
            tx_tx_processed.try_send(event).unwrap();
        };

        // Test 1: Register handler before connect
        let block_added_handler_id = client
            .on_event(EventType::BlockAdded, block_added_handler)
            .await
            .unwrap();
        assert_eq!(block_added_handler_id, 0, "First handler should have ID 0");

        // Connect the client
        client.connect().await.unwrap();

        // Test 2: Register handler after connect
        let transaction_processed_handler_id = client
            .on_event(EventType::TransactionProcessed, tx_processed_handler)
            .await
            .unwrap();
        assert_eq!(
            transaction_processed_handler_id, 1,
            "Second handler should have ID 1"
        );

        // Test 3: TransactionProcessed event handling
        let transaction_processed_event = SseData::TransactionProcessed(serde_json::json!({
            "height": 1,
        }));
        mock_server
            .send_event(transaction_processed_event.clone())
            .await
            .unwrap();

        let received_event = rx_tx_processed
            .recv()
            .await
            .expect("Should receive TransactionProcessed event");
        assert_eq!(
            received_event, transaction_processed_event,
            "Received event should match"
        );

        // Test 4: BlockAdded event handling
        let block_added_event = SseData::BlockAdded(serde_json::json!({
            "height": 1,
        }));
        mock_server
            .send_event(block_added_event.clone())
            .await
            .unwrap();

        let received_event = rx_block_added
            .recv()
            .await
            .expect("Should receive BlockAdded event");
        assert_eq!(
            received_event, block_added_event,
            "Received event should match"
        );
    }

    #[tokio::test]
    async fn test_client_on_event_multiple_invocations() {
        let mock_server = MockSse::start().await;
        let mut client = Client::new(&mock_server.url()).await;
        client.connect().await.unwrap();

        let invocation_count = Arc::new(Mutex::new(0));

        let finality_signature_handler = {
            let invocation_count = Arc::clone(&invocation_count);
            move |_| {
                let mut count = invocation_count.lock().unwrap();
                *count += 1;
            }
        };

        let _handler_id = client
            .on_event(EventType::FinalitySignature, finality_signature_handler)
            .await
            .unwrap();

        // Send events
        for i in 0..5 {
            let finality_signature_event = SseData::FinalitySignature(serde_json::json!({
                "height": i,
            }));
            mock_server
                .send_event(finality_signature_event)
                .await
                .unwrap();

            // Short delay after each event to allow processing
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        let final_count = *invocation_count.lock().unwrap();
        assert_eq!(final_count, 5, "Handler should have been called 5 times");
    }

    #[tokio::test]
    async fn test_client_wait_for_event() {
        let mock_server = MockSse::start().await;
        let mut client = Client::new(&mock_server.url()).await;
        client.connect().await.unwrap();

        let predicate = |data: SseData| {
            if let SseData::BlockAdded(block_data) = data {
                if let Some(height) = block_data["height"].as_u64() {
                    return height == 13;
                }
            }
            false
        };
        let timeout = Duration::from_millis(100);

        // Spawn a task to wait for the event
        let event_future = tokio::spawn(async move {
            client
                .wait_for_event(EventType::BlockAdded, predicate, timeout)
                .await
        });

        let block_added_event = SseData::BlockAdded(serde_json::json!({
            "height": 13,
        }));
        mock_server
            .send_event(block_added_event.clone())
            .await
            .unwrap();

        let result = event_future.await.unwrap();

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
