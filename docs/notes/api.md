# SDK Development Notes

## API :: NODE :: BIN

A node operator may decide to expose a port to the node's Binary server. The Binary server is built over a pure TCP/IP channel and exposes a large set of endpoints.  Each endpoint accepts binary encoded requests and returns binary encoded responses.

The node's Binary server API surface is subdivided into 2 high level endpoint types:

- Get -> processes 3 types of query:

  - Record -> historical data structures, e.g. blocks

  - Information -> information about the node, e.g. status

  - State -> data from the global state, e.g. account balance

- Try -> processes 2 types of transactions:

  - AcceptTransaction -> transaction to be processed by network

  - SpeculativeExec -> transaction to be processed locally for testing purposes

An SDK MUST support the node's Binary API.  Such support results in a client side performance optimisation as both serialization & network overheads are reduced.  Therefore the following module should be implemented:

- `api.binary`

## API :: NODE :: REST

A node operator may decide to expose a port to the node's REST server. The REST server exposes a small set of endpoints convenient to node operators in their day to day operations.

## API :: NODE :: SSE

A node operator may decide to expose a port to the node's Server Sent Events (SSE) server. The SSE server exposes a single event channel, Main, over which is emitted a set of events such as TransactionProcessed, BlockAdded ... etc. Each event type is associated with a JSON encoded data structure.

The SSE server emits events in respect of blocks, transactions, and smart contracts.  It is an essential element of low-latency off-chain orchestration agents.

The SSE server exposes a single channel, Main, over which is emitted a set of events such as TransactionProcessed, BlockAdded ... etc.  Each event type is associated with a JSON encoded data structure.

An SDK MUST support asynchronous interaction with the SSE server.  This feature is an essential element of low-latency off-chain orchestration agents. For example a stablecoin operator may construct such an an agent so as to reduce latency around consensus monitoring.  Therefore following module should be implemented:

- `api.sse`

## API :: SIDECAR :: JSON-RPC

### Servers

A sidecar operator may choose to open ports to the following JSON-RPC servers:

- main
- speculative-execution

An SDK SHOULD support both servers and therefore will implement the following modules:

- `api.jsonrpc.main`
- `api.jsonrpc.spec_exec`

A typical client / proxy design pattern will be implemented. Whilst the proxy handles node connection and communication, the client handles request/response serialisation and error propogation.

The request/response types will either be forward engineered from the json-rpc schema, or imported from the casper-types crate.  The former approach depends upon adequate codegen tooling.

### Errors

Error scenarios:

- Sidecar connection error.
- Sidecar panic.
- Sidecar JSON-RPC response error, i.e. an error caused by either:
    - request validation error:
        - malformed requests
        - incoherent requests
    - request execution error

### Behaviour

- map input parameters -> valid API request
- establish connection -> API server
- dispatch API request -> API server
- map API response -> Result<Response<T>, Error>

## Utils :: Cryptography

### Hashing

- Supported algos: blake2b & blake3

- Default algo = blake2b

- Default digest length = 32

- Functions operate over pure bytes, i.e. any domain type instance bytes encoding is done prior to hash computation

- Core functions:

  - get_hash

### Signature Schemes

- Supported algos: secp256k1 & ed25519

- Default algo = ed25519

- Functions operate over pure bytes, i.e. any domain type instance bytes encoding is done prior to signature computation

- Suggested core functions:

  - get_key_pair
  - get_signature
  - is_signature_valid

- Suggested helper factory functions:

  - get_key_pair
  - get_key_pair_from_base16
  - get_key_pair_from_base64
  - get_key_pair_from_bytes
  - get_key_pair_from_pem_file
  - get_key_pair_from_seed
  - get_signature_from_pem_file
