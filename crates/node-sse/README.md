## Node SSE API

A node operator may decide to expose a port to the node's Server Sent Events (SSE) server. The SSE server exposes a single event channel, Main, over which is emitted a set of events such as TransactionProcessed, BlockAdded ... etc. Each event type is associated with a JSON encoded data structure.

The SSE server emits events in respect of blocks, transactions, and smart contracts.  It is an essential element of low-latency off-chain orchestration agents.

The SSE server exposes a single channel, Main, over which is emitted a set of events such as TransactionProcessed, BlockAdded ... etc.  Each event type is associated with a JSON encoded data structure.

An SDK MUST support asynchronous interaction with the SSE server.  This feature is an essential element of low-latency off-chain orchestration agents. For example a stablecoin operator may construct such an an agent so as to reduce latency around consensus monitoring.  Therefore following module should be implemented:
