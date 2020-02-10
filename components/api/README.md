# HTTP API engine for Vega

[![Travis Build Status](https://img.shields.io/travis/vega/vega/master.svg?label=Linux%20Build)](https://travis-ci.com/vega/vega)
[![License: Apache-2.0](https://img.shields.io/github/license/vega/vega.svg)](https://github.com/vega/vega/blob/master/LICENSE)
![rust 1.36.0+ required](https://img.shields.io/badge/rust-1.36.0+-blue.svg?label=Required%20Rust)

`vega-api` crate provides an extensible interface for building backend-agnostic
HTTP APIs.

Within Vega, this crate is used by [Rust services][rust-runtime] and in
plugins for the Vega node.

Under the hood `vega-api` uses [`actix`].

Consult [the crate docs](https://docs.rs/vega-api) for more details.

## Examples

Providing HTTP API for a plugin:

```rust
use vega_api::{ApiBuilder};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SomeQuery {
    pub first: u64,
    pub second: u64,
}

fn create_api() -> ApiBuilder {
    let mut builder = ApiBuilder::new();
    builder
        .public_scope()
        .endpoint("some", |query: SomeQuery| {
            Ok(query.first + query.second)
        });
    builder
}

let builder = create_api();
// `builder` can now be passed to the node via plugin interface
// or via node channel.
```

## Usage

Include `vega-api` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
vega-api = "0.1.0"
```

Note that the crate rarely needs to be imported directly; it is re-exported
by the `vega` crate.

## License

`vega-api` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.

[`actix`]: https://crates.io/crates/actix
[rust-runtime]: ../../runtimes/rust
