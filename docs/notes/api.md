# SDK Development Notes

## API :: JSON-RPC

### Sidecar Servers

A sidecar operator may choose to open ports to the following JSON-RPC servers:

- main
- speculative-execution

An SDK must therefore support interacting with both servers and so will implement the following modules:

- `api.jsonrpc.main`
- `api.jsonrpc.spec_exec`

The jsonrpc service request/response types, can either be forward engineered from the json-rpc schema, or imported from the casper-types crate.  The former approach depends upon adequate codegen tooling.  The `api.jsonrpc.spec_exec` module will import request/response types exposed by `api.jsonrpc.main` thus avoiding unecessary duplication.

### Proxy Utility

Both of the above modules will share utilities, in particular a proxy, therefore the following module will be implemented:

- `api.jsonrpc.utils.proxy`

The proxy will handle the following errors:

  - A sidecar panic.

  - A sidecar connection error.

  - A sidecar JSON-RPC response error, i.e. an error caused by either:

      - request validation error:

          - malformed requests

          - incoherent requests

      - request execution error

  - All such errrors will raise a `ProxyError<E>`

For each remote endpoint to be supported, the proxy will:

- convert input parameters to a valid json-rpc request

- dispatch request to remote server & handling any dispatch errors

- return raw response data to be processed by caller
