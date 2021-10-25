# kiibohd-core-ffi

FFI interface for kiibohd-core crates.
This is useful when you want to encompass all (or a set) of kiibohd-core crates as a single static library to import into your C firmware.

This library is meant to be built for many platforms.
Generally these are tested:

* thumbv7em-none-eabi
* x86_64-unknown-linux-gnu


## Building

You'll need to be using Rust nightly and cargo-c.

```bash
rustup install nightly
cargo install cargo-c --force

cargo +nightly cbuild
cargo +nightly cbuild --target thumbv7em-none-eabi --release
```

