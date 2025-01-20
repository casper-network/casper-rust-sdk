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
    pub fn new(url: String) -> Self {
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

impl ProxyConfig {
    pub fn backoff_factor<'a>(&'a mut self, value: u32) -> &'a mut ProxyConfig {
        self.backoff_factor = value;
        self
    }

    pub fn delay_on_retry<'a>(&'a mut self, value: Duration) -> &'a mut ProxyConfig {
        self.delay_on_retry = value;
        self
    }

    pub fn max_delay_between_reconnects<'a>(&'a mut self, value: Duration) -> &'a mut ProxyConfig {
        self.max_delay_between_reconnects = value;
        self
    }

    pub fn reconnect_on_error<'a>(&'a mut self, value: bool) -> &'a mut ProxyConfig {
        self.reconnect_on_error = value;
        self
    }

    pub fn retry_initial_connection<'a>(&'a mut self, value: bool) -> &'a mut ProxyConfig {
        self.retry_initial_connection = value;
        self
    }
}
