use casper_sdk_rs::api::node::sse::SseData;
use casper_types::ProtocolVersion;
use std::net::SocketAddr;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub struct MockSse {
    addr: String,
    tx: mpsc::Sender<SseData>,
}

impl MockSse {
    pub async fn start() -> Self {
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap(); // Use port 0 for dynamic allocation
        let listener = TcpListener::bind(addr).await.unwrap();
        let addr = format!("http://{}", listener.local_addr().unwrap().to_string());
        let (tx, mut rx) = mpsc::channel(32);

        tokio::spawn(async move {
            if let Ok((mut socket, _)) = listener.accept().await {
                let mut writer = BufWriter::new(&mut socket);
                let version_event = SseData::ApiVersion(ProtocolVersion::from_parts(2, 0, 0));
                let serialized_event = format!(
                    "data: {}\n\n",
                    serde_json::to_string(&version_event).unwrap()
                );
                writer
                    .write_all("HTTP/1.1 200 OK\r\n".as_bytes())
                    .await
                    .unwrap();
                writer
                    .write_all("content-type: text/event-stream\r\n".as_bytes())
                    .await
                    .unwrap();
                writer
                    .write_all("Cache-Control: no-cache\r\n".as_bytes())
                    .await
                    .unwrap();
                writer
                    .write_all("Connection: keep-alive\r\n\r\n".as_bytes())
                    .await
                    .unwrap();
                println!(
                    "🤝 Initializing SSE stream with API version handshake: {:?}",
                    serialized_event
                );
                writer.write_all(serialized_event.as_bytes()).await.unwrap();
                writer.flush().await.unwrap();

                while let Some(event) = rx.recv().await {
                    let serialized_event =
                        format!("data: {}\n\n", serde_json::to_string(&event).unwrap());
                    println!("📣 Broadcasting event: {:?}", serialized_event);
                    writer.write_all(serialized_event.as_bytes()).await.unwrap();
                    writer.flush().await.unwrap();
                }
            }
        });
        Self { addr, tx }
    }
    pub fn url(&self) -> String {
        self.addr.clone()
    }

    pub async fn send_event(&self, data: SseData) -> Result<(), Box<dyn std::error::Error>> {
        self.tx
            .send(data)
            .await
            .map_err(|_| "Failed to send event".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eventsource_stream::Eventsource;
    use futures::StreamExt;
    use reqwest::Client;
    use serde_json::json;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_mock_sse_start_and_connect() {
        let mock_server = MockSse::start().await;
        let url = mock_server.url();
        let client = Client::new();
        let response = client.get(&url).send().await.unwrap();

        assert!(response.status().is_success(), "Invalid HTTP status");
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/event-stream",
            "Invalid Content-Type"
        );
    }

    #[tokio::test]
    async fn test_mock_sse_handshake() {
        let mock_server = MockSse::start().await;
        let url = mock_server.url();
        let client = Client::new();
        let response = client.get(&url).send().await.unwrap();

        let mut stream = response.bytes_stream().eventsource();
        let event = stream.next().await.unwrap().unwrap();
        let expected_version = ProtocolVersion::from_parts(2, 0, 0);

        let handshake_data: SseData = serde_json::from_str(&event.data).unwrap();
        match handshake_data {
            SseData::ApiVersion(v) => assert_eq!(v, expected_version),
            _ => panic!("Expected ApiVersion event, got {:?}", handshake_data),
        }
    }

    #[tokio::test]
    async fn test_mock_sse_send_event() {
        let mock_server = MockSse::start().await;
        let url = mock_server.url();
        let client = Client::new();
        let response = client.get(&url).send().await.unwrap();

        // Skip the handshake
        let mut stream = response.bytes_stream().eventsource().skip(1);

        let test_event = SseData::BlockAdded(json!({
            "height": 100,
        }));
        mock_server.send_event(test_event.clone()).await.unwrap();

        let event = timeout(Duration::from_secs(2), stream.next())
            .await
            .expect("Should receive sent event")
            .expect("Event stream should not be empty")
            .unwrap();

        let received_data: SseData = serde_json::from_str(&event.data).unwrap();
        assert_eq!(received_data, test_event);

        let test_event = SseData::FinalitySignature(json!({
            "height": 100,
        }));
        mock_server.send_event(test_event.clone()).await.unwrap();

        let event = timeout(Duration::from_secs(2), stream.next())
            .await
            .expect("Should receive sent event")
            .expect("Event stream should not be empty")
            .unwrap();

        let received_data: SseData = serde_json::from_str(&event.data).unwrap();
        assert_eq!(received_data, test_event);
    }
}
