# System API for Vega node

[![Travis Build Status](https://img.shields.io/travis/vega/vega/master.svg?label=Linux%20Build)](https://travis-ci.com/vega/vega)
[![License: Apache-2.0](https://img.shields.io/github/license/vega/vega.svg)](https://github.com/vega/vega/blob/master/LICENSE)
![rust 1.36.0+ required](https://img.shields.io/badge/rust-1.36.0+-blue.svg?label=Required%20Rust)

Plugin extending HTTP API of the node to return information about node state.

## Usage

Include `vega-system-api` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
vega-system-api = "0.1.0"
```

`SystemApiPlugin` type, located at the root of the crate, should be used
as a node plugin during node creation.
Consult [the crate docs](https://docs.rs/vega-system-api) for more details.

## License

`vega-api` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
