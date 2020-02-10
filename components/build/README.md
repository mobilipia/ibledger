# Build scripts utility for Vega

[![Travis Build Status](https://img.shields.io/travis/vega/vega/master.svg?label=Linux%20Build)](https://travis-ci.com/vega/vega)
[![License: Apache-2.0](https://img.shields.io/github/license/vega/vega.svg)](https://github.com/vega/vega/blob/master/LICENSE)
![rust 1.36.0+ required](https://img.shields.io/badge/rust-1.36.0+-blue.svg?label=Required%20Rust)

This crate simplifies writing build scripts for Vega and Vega services.

Since Protobuf is the Vega default serialization format, build scripts
are mostly used to compile Protobuf files and generate corresponding code.
Generated code is used later by the Vega core and services.

There are several predefined sets of protobuf sources available for use.
Currently presented sets:

- Crypto sources: all the necessary crypto types used in services
  and system proto-files. These types are Hash, PublicKey and Signature.
- Vega sources: types used in core and in system services such
  as `Supervisor`.
- Common sources: types that can be used by various parts of Vega.
- MerkleDB sources: types representing proofs of existence of element
  in database.

Consult [the crate docs](https://docs.rs/vega-build) for more details.

## Examples

Sample `build.rs` using `vega-build`:

```rust
use vega_build::ProtobufGenerator;
use std::env;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current dir.");
    let protos = current_dir.join("src/proto");

    ProtobufGenerator::with_mod_name("protobuf_mod.rs")
        .with_input_dir("src/proto")
        .with_crypto()
        .generate();
}
```

## Usage

Include `vega-build` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
vega-build = "0.1.0"
```

## License

`vega-build` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
