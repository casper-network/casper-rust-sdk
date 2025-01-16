# Node binary API

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
