use std::net::SocketAddr;

/// Proxy configuration.
#[derive(Clone, DataSize, Debug, Deserialize, PartialEq, Eq)]
// Disallow unknown fields to ensure config files and command-line overrides contain valid keys.
#[serde(deny_unknown_fields)]
pub struct ProxyConfig {
    /// Address of the node.
    pub address: SocketAddr,
    /// Maximum size of a message in bytes.
    pub max_message_size_bytes: u32,
    /// Message transfer timeout in seconds.
    pub message_timeout_secs: u64,
    /// Timeout specifying how long to wait for binary port client to be available.
    // Access to the client is synchronized.
    pub client_access_timeout_secs: u64,
    /// Maximum number of in-flight node requests.
    pub request_limit: u16,
    /// Number of node requests that can be buffered.
    pub request_buffer_size: usize,
    /// Configuration for exponential backoff to be used for re-connects.
    pub exponential_backoff: ExponentialBackoffConfig,
}

/// Exponential backoff configuration for re-connects.
#[derive(Clone, DataSize, Debug, Deserialize, PartialEq, Eq)]
// Disallow unknown fields to ensure config files and command-line overrides contain valid keys.
#[serde(deny_unknown_fields)]
pub struct ExponentialBackoffConfig {
    /// Initial wait time before the first re-connect attempt.
    pub initial_delay_ms: u64,
    /// Maximum wait time between re-connect attempts.
    pub max_delay_ms: u64,
    /// The multiplier to apply to the previous delay to get the next delay.
    pub coefficient: u64,
    /// Maximum number of connection attempts.
    pub max_attempts: MaxAttempts,
}

#[derive(Clone, DataSize, Debug, Deserialize, PartialEq, Eq)]
pub enum MaxAttempts {
    Infinite,
    Finite(usize),
}

impl MaxAttempts {
    pub fn can_attempt(&self, current_attempt: usize) -> bool {
        match self {
            MaxAttempts::Infinite => true,
            MaxAttempts::Finite(max_attempts) => *max_attempts >= current_attempt,
        }
    }
}
