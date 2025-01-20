use std::time::Duration;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

pub struct ProxyConfig {
    // Factor by which delays between reconnect attempts will exponentially increase.
    backoff_factor: u32,

    // Initial delay before trying to reconnect.
    delay_on_retry: Duration,

    // Flag enabling or disabling reconnection on stream error.
    reconnect_on_error: bool,

    // Flag enabling or disabling retry if initial server connection fails.
    retry_initial_connection: bool,

    // URL of remote SSE server.
    url: String,

    // Maximum delay between reconnects.
    max_delay_between_reconnects: Duration,
}
