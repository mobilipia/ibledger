# Vega Explorer Service

[![Travis Build Status](https://img.shields.io/travis/vega/vega/master.svg?label=Linux%20Build)](https://travis-ci.com/vega/vega)
[![License: Apache-2.0](https://img.shields.io/github/license/vega/vega.svg)](https://github.com/vega/vega/blob/master/LICENSE)
![rust 1.36.0+ required](https://img.shields.io/badge/rust-1.36.0+-blue.svg?label=Required%20Rust)

`vega-explorer-service` provides HTTP endpoints for exploring
Vega blockchains.

This crate is distinct from the base [explorer][explorer] crate.
The base explorer provides Rust language APIs for retrieving info
from the blockchain, while this crate translates these APIs into
REST and WebSocket endpoints and packages this logic as an Vega service.
Thus, this crate is useful if you want to provide the way for external apps
to query the blockchain info.

## Description

The explorer service does not define transactions, but it has several
REST / WebSocket endpoints allowing to retrieve information from the
blockchain in a structured way.

Usually, the explorer service should be instantiated at the blockchain start
with the default identifiers. There may be no more than one explorer service
on a blockchain; an attempt to create a second service instance will lead to
an error in the service constructor.

The API types necessary to interact with the service HTTP API are defined in
a separate crate, [`vega-explorer`]. The base explorer provides Rust language
APIs for retrieving info from the blockchain, while this crate translates these
APIs into REST and WebSocket endpoints and packages this logic as an Vega
service.

Thus, this crate is useful if you want to provide the way for external apps to
query the blockchain info.

## HTTP API

REST API of the service is documented in the [`api` module](api-module), and its
WebSocket API in the [`api::websocket` module](websocket-module).

## Usage

Include `vega-explorer-service` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
vega = "0.1.0"
vega-explorer-service = "0.1.0"
```

The explorer service should usually be initialized at the blockchain start
with the default identifiers. The service will refuse to instantiate
if an explorer service is already instantiated on the blockchain.

Consult [the crate docs](https://docs.rs/vega-explorer-service)
for more details about the service API.

## License

`vega-explorer-service` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.

[explorer]: https://crates.io/crates/vega-explorer/
[api-module]: https://docs.rs/vega-explorer-service/latest/vega-explorer-service/api/index.html
[websocket-module]: https://docs.rs/vega-explorer-service/latest/vega-explorer-service/api/websocket/index.html
