use async_trait::async_trait;
use tokio_util::codec::Framed;

use l1_binary_port::{BinaryRequest, BinaryResponseAndRequest};

use config::ProxyConfig;

#[async_trait]
pub trait Proxy: Send + Sync {
    async fn dispatch(&self, req: BinaryRequest) -> Result<BinaryResponseAndRequest, Error>;
}

pub struct MockProxy {
    config: ProxyConfig,
}

pub struct RemoteProxy {
    config: ProxyConfig,
}

impl Proxy for MockProxy {
    async fn dispatch(&self, req: BinaryRequest) -> Result<BinaryResponseAndRequest, Error> {
        println!("MockProxy :: dispatch");
    }
}

impl Proxy for RemoteProxy {
    async fn dispatch(&self, req: BinaryRequest) -> Result<BinaryResponseAndRequest, Error> {
        println!("RemoteProxy :: dispatch");
    }
}
