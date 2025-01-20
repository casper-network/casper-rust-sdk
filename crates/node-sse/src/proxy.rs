use std::time::Duration;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Proxy configuration information.
pub struct ProxyConfig {
    // Factor by which delays between reconnect attempts will exponentially increase.
    backoff_factor: u32,

    // Initial delay before trying to reconnect.
    delay_on_retry: Duration,

    // Maximum delay between reconnects.
    max_delay_between_reconnects: Duration,

    // Flag enabling or disabling reconnection on stream error.
    reconnect_on_error: bool,

    // Flag enabling or disabling retry if initial server connection fails.
    retry_initial_connection: bool,

    // URL of remote SSE server.
    url: String,
}

// Proxy to remote SSE server.
pub struct Proxy {
    // Associated configuration information.
    config: ProxyConfig,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Proxy {
    pub fn new(config: ProxyConfig) -> Self {
        Self { config }
    }
}

impl ProxyConfig {
    pub fn new(
        backoff_factor: u32,
        delay_on_retry: Duration,
        max_delay_between_reconnects: Duration,
        reconnect_on_error: bool,
        retry_initial_connection: bool,
        url: String,
    ) -> Self {
        Self {
            backoff_factor,
            delay_on_retry,
            max_delay_between_reconnects,
            reconnect_on_error,
            retry_initial_connection,
            url,
        }
    }

    pub fn new_from_defaults(url: String) -> Self {
        Self {
            backoff_factor: 2,
            delay_on_retry: Duration::from_secs(2),
            max_delay_between_reconnects: Duration::from_secs(60),
            reconnect_on_error: true,
            retry_initial_connection: true,
            url,
        }
    }
}
