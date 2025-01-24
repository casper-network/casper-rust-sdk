use async_trait::async_trait;
use eventsource_client::ReconnectOptions;
use eventsource_client::{self as es, Client};
use futures::{Stream, TryStreamExt};
use std::time::Duration;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

// Proxy to remote SSE server.
pub struct Proxy {
    // Associated node see port client.
    // client: es::Client,

    // Associated configuration information.
    config: ProxyConfig,

    // Associated callback handler.
    handler: Option<Box<dyn ProxyEventHandler>>,
}

// Proxy configuration information.
#[derive(Clone)]
pub struct ProxyConfig {
    // Factor by which delays between reconnect attempts will exponentially increase.
    backoff_factor: u32,

    // Delay to await before trying to reconnect.
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

// Proxy configuration builder.
pub struct ProxyConfigBuilder {
    // Built configuration instance.
    cfg: ProxyConfig,
}

// Event handler trait implemented by concrete event listeners.
#[async_trait]
pub trait ProxyEventHandler {
    // Invoked upon successful connection to remote server.
    async fn on_sse_connection(&self);

    // Invoked upon a processing error, local or remote.
    async fn on_sse_error(&self);

    // Invoked upon receipt of a remote SSE event.
    async fn on_sse_event(&self, event: &str, data: &str);
}

// Event information emitted by remote server.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProxyEventInfo {
    // Kind of event emitted over SSE channel.
    pub kind: String,

    // Associate event data.
    pub data: String,

    // Associate event identifier - normally a monotonically incerasing integer.
    pub id: Option<String>,

    // Retry attempts.
    pub retry: Option<u64>,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Proxy {
    pub fn new(config: ProxyConfig) -> Self {
        Self {
            config,
            handler: Option::None,
        }
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

impl ProxyConfigBuilder {
    pub fn new() -> Self {
        let cfg = ProxyConfig {
            ..Default::default()
        };
        Self { cfg }
    }
}

impl Default for ProxyConfig {
    // Default proxy configuration attempts to connect to node #1 within a local CCTL network.
    fn default() -> Self {
        Self {
            backoff_factor: 2,
            delay_on_retry: Duration::from_secs(2),
            max_delay_between_reconnects: Duration::from_secs(60),
            reconnect_on_error: true,
            retry_initial_connection: true,
            url: String::from("http://localhost:14101/events"),
        }
    }
}

// ------------------------------------------------------------------------
// Constructors: builders.
// ------------------------------------------------------------------------

impl ProxyConfig {
    // Sets factor by which delays between reconnect attempts will exponentially increase.
    pub fn backoff_factor<'a>(&'a mut self, value: u32) -> &'a mut ProxyConfig {
        self.backoff_factor = value;
        self
    }

    // Sets delay to await before trying to reconnect
    pub fn delay_on_retry<'a>(&'a mut self, value: Duration) -> &'a mut ProxyConfig {
        self.delay_on_retry = value;
        self
    }

    // Sets maximum delay between reconnects.
    pub fn max_delay_between_reconnects<'a>(&'a mut self, value: Duration) -> &'a mut ProxyConfig {
        self.max_delay_between_reconnects = value;
        self
    }

    // Sets flag enabling or disabling reconnection on stream error.
    pub fn reconnect_on_error<'a>(&'a mut self, value: bool) -> &'a mut ProxyConfig {
        self.reconnect_on_error = value;
        self
    }

    // Sets flag enabling or disabling retry if initial server connection fails.
    pub fn retry_initial_connection<'a>(&'a mut self, value: bool) -> &'a mut ProxyConfig {
        self.retry_initial_connection = value;
        self
    }
}

// ------------------------------------------------------------------------
// Methods.
// ------------------------------------------------------------------------

impl Proxy {
    pub async fn run<E: ProxyEventHandler + Send + Sync>(
        &self,
        _: &E,
    ) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    pub async fn run1(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
        // fn tail_events(
        //     proxy: &Proxy,
        //     client: impl es::Client,
        // ) -> impl Stream<Item = Result<(), ()>> {
        //     unimplemented!()
        // }

        // let g = ReconnectOptions::from(&self.config);

        // let client = es::ClientBuilder::for_url(&self.config.url)?
        //     .reconnect(ReconnectOptions::from(&self.config))
        //     .build();
        // let mut stream = tail_events(&self, client);

        // while let Ok(Some(_)) = stream.try_next().await {}

        // Ok(())
    }
}

impl From<&ProxyConfig> for ReconnectOptions {
    fn from(value: &ProxyConfig) -> Self {
        ReconnectOptions::reconnect(value.reconnect_on_error)
            .backoff_factor(value.backoff_factor)
            .delay(value.delay_on_retry)
            .delay_max(value.max_delay_between_reconnects)
            .retry_initial(value.retry_initial_connection)
            .build()
    }
}
