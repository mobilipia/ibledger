# vega-time

[![Travis Build Status](https://img.shields.io/travis/vega/vega/master.svg?label=Linux%20Build)](https://travis-ci.com/vega/vega)
[![Docs.rs](https://docs.rs/vega-time/badge.svg)](https://docs.rs/vega-time)
[![License: Apache-2.0](https://img.shields.io/github/license/vega/vega.svg)](https://github.com/vega/vega/blob/master/LICENSE)
![rust 1.36.0+ required](https://img.shields.io/badge/rust-1.36.0+-blue.svg?label=Required%20Rust)

`vega-time` is a time oracle service for [Vega blockchain framework](https://vega.com/).
This service allows to determine time,
import it from the external world to the blockchain
and keep its current value in the blockchain.

## Usage

Include `vega-time` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
vega = "0.1.0"
vega-cli = "0.1.0"
vega-time = "0.1.0"
```

## Examples

Examples of the node with `vega-time` service, and service using
`vega-time` service to obtain current time can be found in
the [examples](examples) folder:

- [node example]
- [service example]

## Further Reading

Consult the [service description in Vega docs](https://vega.com/doc/version/latest/advanced/time)
for a more high-level perspective, in particular, the design rationale
and the proof of correctness.

## Other languages support

- [Java Time Oracle](https://github.com/vega/vega-java-binding/tree/master/vega-java-binding/time-oracle)

## License

`vega-time` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.

[node example]: examples/vega_time.rs
[service example]: examples/simple_service/main.rs
