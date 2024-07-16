// tests/sse_client.rs

use casper_sdk_rs::api::node::sse::{types::EventFilter, CasperEventClient};

#[tokio::test]
async fn test_sse_client_connection() {
    // Create the client
    let mut client = CasperEventClient::default();

    // Connect to the SSE endpoint
    client
        .connect()
        .await
        .expect("Failed to connect to SSE endpoint");

    // Assert that the event_stream is not None after connecting
    assert!(client.event_stream.is_some());
}

#[tokio::test]
async fn test_sse_wait_for_event() {
    let mut client = CasperEventClient::default();
    client
        .connect()
        .await
        .expect("Failed to connect to SSE endpoint");

    let event = client.wait_for_event(EventFilter::BlockAdded).await;
    print!("event: {:?}", event);
}
