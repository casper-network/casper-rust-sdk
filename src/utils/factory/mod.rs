// TODO: proxy wrapper around set of JSON RPC endpoints exposed by sidecar.rpc-server.main.
//
// NOTES
// 1. For each target endpoint, proxy performs simple response decoding,
//    i.e. it does not decode domain types - that is done upstream by the client.
//
// 2. Proxy will handle the following errors:
//    - Sidecar panics.
//    - Sidecar connection errors.
//    - Sidecar JSON-RPC response errors, i.e. errors caused by either:
//          - malformed requests;
//          - requests that fail business validation logic;
//          - requests that fail business execution logic;
//
//    - All such errrors will result in ProxyError<E>
//
// 3. For each request sucessfully processed by the sidecar:
//    - proxy accepts endpoint parameters encoded as a HashMap (ready to be encoded as JSON string)
//    - proxy processes raw JSON-RPC response and returns a HashMap (ready to be encoded as a domain type instance).
//
