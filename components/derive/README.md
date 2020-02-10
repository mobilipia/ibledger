# Procedural macros for Vega

[![Travis Build Status](https://img.shields.io/travis/vega/vega/master.svg?label=Linux%20Build)](https://travis-ci.com/vega/vega)
[![License: Apache-2.0](https://img.shields.io/github/license/vega/vega.svg)](https://github.com/vega/vega/blob/master/LICENSE)
![rust 1.36.0+ required](https://img.shields.io/badge/rust-1.36.0+-blue.svg?label=Required%20Rust)

This crate provides several procedural macros for Vega core and Vega services.

Overview of presented macros:

- `BinaryValue`: derive macro for `BinaryValue` trait of MerkleDB.
  Depending on codec, implementation may use `ProtobufConvert` (default)
  trait as base, or `serde` traits using `bincode`.
- `ObjectHash`: derive macro for `ObjectHash` trait of MerkleDB.
  It can be used for any type that implements `BinaryValue` trait.
- `FromAccess`: derive macro for `FromAccess` trait for schemas of
  MerkleDB indexes.
- `ServiceDispatcher`: derive macro for generating dispatching mechanisms
  of Rust Vega services.
- `ServiceFactory`: derive macro for generating factory mechanisms
  of Rust Vega services.
- `vega_interface`: attribute macro for transforming trait into interface
  of Rust Vega service.
- `ExecutionFail`: derive macro similar to `failure::Fail`, implementing
  `ExecutionFail` trait for an enum.
- `RequireArtifact`: derive macro for `RequireArtifact` trait.

Consult [the crate docs](https://docs.rs/vega-derive) for more details.

## Usage

Include `vega-derive` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
vega-derive = "0.1.0"
```

## License

`vega-derive` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
