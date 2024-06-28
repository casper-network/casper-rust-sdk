use std::net::{IpAddr, Ipv4Addr};

/// Default address to connect to the node.
// Change this to SocketAddr, once SocketAddr::new is const stable.
const DEFAULT_NODE_CONNECT_ADDRESS: (IpAddr, u16) = (IpAddr::V4(Ipv4Addr::LOCALHOST), 28104);
/// Default maximum payload size.
const DEFAULT_MAX_PAYLOAD_SIZE: u32 = 4 * 1024 * 1024;
/// Default message timeout in seconds.
const DEFAULT_MESSAGE_TIMEOUT_SECS: u64 = 30;
/// Default timeout for client access.
const DEFAULT_CLIENT_ACCESS_TIMEOUT_SECS: u64 = 10;
/// Default request limit.
const DEFAULT_NODE_REQUEST_LIMIT: u16 = 3;
/// Default request buffer size.
const DEFAULT_REQUEST_BUFFER_SIZE: usize = 16;
/// Default exponential backoff base delay.
const DEFAULT_EXPONENTIAL_BACKOFF_BASE_MS: u64 = 1000;
/// Default exponential backoff maximum delay.
const DEFAULT_EXPONENTIAL_BACKOFF_MAX_MS: u64 = 64_000;
/// Default exponential backoff coefficient.
const DEFAULT_EXPONENTIAL_BACKOFF_COEFFICIENT: u64 = 2;
