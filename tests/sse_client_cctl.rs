#[cfg(feature = "cctl-tests")]
#[cfg(test)]
mod cctl_integration_tests {
    use casper_sdk_rs::api::node::sse::{client::Client, types::EventType, SseData};
    use std::time::Duration;
    use tokio::sync::mpsc;

    const CCTL_SSE_URL: &str = "http://localhost:18101/events";

    #[tokio::test]
    async fn test_cctl_client_connect() {
        let client = Client::new(CCTL_SSE_URL).await;
        let result = client.connect().await;
        assert!(result.is_ok(), "Client should connect successfully to cctl");
    }

    #[tokio::test]
    async fn test_cctl_client_on_block_added_event() {
        let mut client = Client::new(CCTL_SSE_URL).await;
        client.connect().await.unwrap();

        let (tx_block_added, mut rx_block_added) = mpsc::channel(1);

        let block_added_handler = move |event: SseData| {
            tx_block_added.try_send(event).unwrap();
        };

        client
            .on_event(EventType::BlockAdded, block_added_handler)
            .await
            .unwrap();

        let timeout = Duration::from_secs(60);

        tokio::select! {
            received_event = rx_block_added.recv() => {
                assert!(received_event.is_some(), "Should receive a BlockAdded event within timeout");
                assert_eq!(received_event.unwrap().event_type(), EventType::BlockAdded, "Should recive event of type BlockAdded");
            }
            _ = tokio::time::sleep(timeout) => {
                panic!("Timed out waiting for BlockAdded event from cctl");
            }
        }
    }

    #[tokio::test]
    async fn test_cctl_client_on_finality_signature_event() {
        let mut client = Client::new(CCTL_SSE_URL).await;
        client.connect().await.unwrap();

        let (tx_block_added, mut rx_block_added) = mpsc::channel(1);

        let block_added_handler = move |event: SseData| {
            tx_block_added.try_send(event).unwrap();
        };

        client
            .on_event(EventType::FinalitySignature, block_added_handler)
            .await
            .unwrap();

        let timeout = Duration::from_secs(60);

        tokio::select! {
            received_event = rx_block_added.recv() => {
                assert!(received_event.is_some(), "Should receive a FinalitySignature event within timeout");
                assert_eq!(received_event.unwrap().event_type(), EventType::FinalitySignature, "Should recive event of type FinalitySignature");
            }
            _ = tokio::time::sleep(timeout) => {
                panic!("Timed out waiting for FinalitySignature event from cctl");
            }
        }
    }

    #[tokio::test]
    async fn test_cctl_client_on_step_event() {
        let mut client = Client::new(CCTL_SSE_URL).await;
        client.connect().await.unwrap();

        let (tx_block_added, mut rx_block_added) = mpsc::channel(1);

        let block_added_handler = move |event: SseData| {
            tx_block_added.try_send(event).unwrap();
        };

        client
            .on_event(EventType::Step, block_added_handler)
            .await
            .unwrap();

        let timeout = Duration::from_secs(60);

        tokio::select! {
            received_event = rx_block_added.recv() => {
                assert!(received_event.is_some(), "Should receive a Step event within timeout");
                assert_eq!(received_event.unwrap().event_type(), EventType::Step, "Should recive event of type Step");
            }
            _ = tokio::time::sleep(timeout) => {
                panic!("Timed out waiting for Step event from cctl");
            }
        }
    }
}
