# Blockchain Explorer Utils for Vega

[![Travis Build Status](https://img.shields.io/travis/vega/vega/master.svg?label=Linux%20Build)](https://travis-ci.com/vega/vega)
[![License: Apache-2.0](https://img.shields.io/github/license/vega/vega.svg)](https://github.com/vega/vega/blob/master/LICENSE)
![rust 1.36.0+ required](https://img.shields.io/badge/rust-1.36.0+-blue.svg?label=Required%20Rust)

`vega-explorer` provides explorer API
for the [Vega blockchain framework](https://vega.com/). For example,
it allows to request transactions from a block together with the execution
statuses, iterate over blocks, etc.

This crate is distinct from the [explorer *service*][explorer-service] crate.
While this crate provides Rust language APIs for retrieving info from the blockchain,
the explorer service translates these APIs into REST and WebSocket endpoints.
Correspondingly, this crate is primarily useful for Rust-language client apps.
Another use case is testing; the [testkit] relies on types in this crate
and re-exports it as the `explorer` module.

Consult [the crate docs](https://docs.rs/vega-explorer)
and [examples](examples) for more details about the service API.

## Usage

Include `vega-explorer` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
vega = "0.1.0"
vega-explorer = "0.1.0"
```

## License

`vega-explorer` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.

[explorer-service]: https://crates.io/crates/vega-explorer-service/
[testkit]: https://crates.io/crate/vega-testkit/
