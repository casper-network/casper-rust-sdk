#[cfg(test)]
mod tests {
    use casper_sdk_rs::api::node::sse::{types::EventFilter, EventClient, SseData};
    use core::panic;
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

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

    #[tokio::test]
    async fn test_wait_for_event_success() {
        let mut server = mockito::Server::new_async().await;
        server
            .mock("GET", "/")
            .with_header("content-type", "text/event-stream")
            .with_body(concat!(
                "data: {\"ApiVersion\": \"1.5.6\"}\n\n",
                "data: {\"BlockAdded\": {\"height\":10}}\n\n",
                "data: {\"BlockAdded\": {\"height\":11}}\n\n",
                "data: {\"BlockAdded\": {\"height\":12}}\n\n",
                "data: {\"TransactionProcessed\": \"test\"}\n\n",
                "data: {\"BlockAdded\": {\"height\":13}}\n\n",
                "data: {\"BlockAdded\": {\"height\":14}}\n\n",
            ))
            .create_async()
            .await;

        let mut client = EventClient::new(&server.url());
        client.connect().await.unwrap();

        let predicate = |data: &SseData| {
            if let SseData::BlockAdded(block_data) = data {
                if let Some(height) = block_data["height"].as_u64() {
                    return height == 13;
                }
            }
            false
        };

        let timeout = Duration::from_secs(5);
        let result = client
            .wait_for_event(EventFilter::BlockAdded, predicate, timeout)
            .await;

        match result {
            Ok(SseData::BlockAdded(block_data)) => {
                assert_eq!(block_data["height"].as_u64().unwrap(), 13);
            }
            Ok(other_event) => panic!("Expected BlockAdded, got {:?}", other_event),
            Err(err) => panic!("Unexpected error: {}", err),
        }
    }
}
