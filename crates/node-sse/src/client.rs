use crate::proxy::{Proxy, ProxyConfig, ProxyEventHandler};
use async_trait::async_trait;

pub struct Client {
    proxy: Proxy,
}

impl Client {
    pub async fn new(config: &ProxyConfig) -> Self {
        Self {
            proxy: Proxy::new(config.clone()),
        }
    }
}

// #[async_trait]
// impl ProxyEventHandler for Client {
//     async fn on_sse_connection(&self) {
//         println!("Component received event: on_connection");
//     }
//     async fn on_sse_error(&self) {
//         println!("Component received event: on_error");
//     }
//     async fn on_sse_event(&self, _: &str, _: &str) {
//         println!("Component received event: on_event");
//     }
// }
