# vega-testkit

[![Travis Build Status](https://img.shields.io/travis/vega/vega/master.svg?label=Linux%20Build)](https://travis-ci.com/vega/vega)
[![Docs.rs](https://docs.rs/vega-testkit/badge.svg)](https://docs.rs/vega-testkit)
[![License: Apache-2.0](https://img.shields.io/github/license/vega/vega.svg)](https://github.com/vega/vega/blob/master/LICENSE)
![rust 1.36.0+ required](https://img.shields.io/badge/rust-1.36.0+-blue.svg?label=Required%20Rust)

Testkit for Vega blockchain is a framework that allows to test operation
of the whole service. Specifically, it allows to test transaction execution
and APIs in the synchronous environment (without consensus algorithm)
and in the same system process.

## Usage

Just add the following line to the `Cargo.toml`:

```toml
[dev-dependencies]
vega-testkit = "0.1.0"
```

[For more details, see Vega documentation][documentation]

## Examples

See the [**tests**](tests) and [**examples**](examples) folders for examples
of building a service and then testing it with the testkit.

## License

Licensed under the Apache License (Version 2.0). See [LICENSE](LICENSE) for details.

[documentation]: https://vega.com/doc/version/latest/advanced/service-testing/
