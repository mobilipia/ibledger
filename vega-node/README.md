# Vega Node Implementation

[![Travis Build Status](https://img.shields.io/travis/vega/vega/master.svg?label=Linux%20Build)](https://travis-ci.com/vega/vega)
[![License: Apache-2.0](https://img.shields.io/github/license/vega/vega.svg)](https://github.com/vega/vega/blob/master/LICENSE)
![rust 1.36.0+ required](https://img.shields.io/badge/rust-1.36.0+-blue.svg?label=Required%20Rust)

`vega-node` provides a node implementation for the [Vega](https://vega.com/)
blockchain framework. Nodes form the blockchain network, in which they reach
consensus as to the latest blockchain state and process transactions coming
from external users. Besides transactions, nodes expose HTTP API of Vega services
and node plugins.

## Usage

Include `vega-node` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
vega = "0.1.0"
vega-node = "0.1.0"
```

`vega-node` provides relatively low-level (but more fine-grained) control
over node lifecycle. See [`vega-cli`] for a more high-level alternative.

## License

`vega-node` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.

[`vega-cli`]: https://crates.io/crates/vega-cli
