# Cryptography primitives for Vega

[![Travis Build Status](https://img.shields.io/travis/vega/vega/master.svg?label=Linux%20Build)](https://travis-ci.com/vega/vega)
[![License: Apache-2.0](https://img.shields.io/github/license/vega/vega.svg)](https://github.com/vega/vega/blob/master/LICENSE)
![rust 1.36.0+ required](https://img.shields.io/badge/rust-1.36.0+-blue.svg?label=Required%20Rust)

`vega-crypto` provides a high-level API for work with various cryptography tasks.

Capabilities of `vega-crypto` include:

- Calculating the hash of data;
- Generating key pairs for work with digital signatures;
- Creating and verifying of digital signatures.

The main backend for `vega-crypto` is `sodiumoxide`, and used algorithms are:

- `SHA256` for hashing.
- `Ed25519` for digital signatures.

Consult [the crate docs](https://docs.rs/vega-crypto) for more details.

## Examples

Signing data and verifying the signature:

```rust
fn main() {
    vega_crypto::init();
    let (public_key, secret_key) = vega_crypto::gen_keypair();
    let data = [1, 2, 3];
    let signature = vega_crypto::sign(&data, &secret_key);
    assert!(vega_crypto::verify(&signature, &data, &public_key));
}
```

Hashing fixed amount of data:

```rust
fn main() {
    vega_crypto::init();
    let data = [1, 2, 3];
    let hash = vega_crypto::hash(&data);
}
```

Hashing data by chunks:

```rust
use vega_crypto::HashStream;

fn main() {
    vega_crypto::init();

    let data: Vec<[u8; 5]> = vec![[1, 2, 3, 4, 5], [6, 7, 8, 9, 10]];
    let mut hash_stream = HashStream::new();
    for chunk in data {
        hash_stream = hash_stream.update(&chunk);
    }
    let _ = hash_stream.hash();
}
```

## Usage

Include `vega-crypto` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
vega-crypto = "0.1.0"
```

## License

`vega-crypto` is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
