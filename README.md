# kiibohd-core

[![Rust](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml/badge.svg)](https://github.com/kiibohd/kiibohd-core/actions/workflows/rust.yml)

kiibohd-core is a re-implementation of the kiibohd firmware as a collection of rust modules.
Initially kiibohd-core is meant to be used as an FFI interface to extend the C-based kiibohd firmware (https://github.com/kiibohd/controller).
Eventually the firmware will be converted entirely to rust (though this may take a while).

This library is meant to be built for many platforms.
Generally these are tested:

* thumbv7em-none-eabi
* x86_64-unknown-linux-gnu

**NOTE**: Do not import kiibohd-core if working with rust, use the member crates instead. kiibohd-core is used as an ffi for other languages to use (e.g. C).

**NOTE**: Crates in this repo generally use nightly due to requirements of hid-io-protocol


## Docs

TODO


## Building

You'll need to be using Rust nightly and cargo-c.

```bash
rustup install nightly
cargo install cargo-c --force

cargo +nightly cbuild
cargo +nightly cbuild --target thumbv7em-none-eabi --release
```


## Testing

Can be used as a quick sanity for all the modules.

```bash
cargo build --all
cargo clippy --all
cargo fmt --all
cargo test --all
cargo doc --all
```
