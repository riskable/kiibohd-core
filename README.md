# kiibohd-core

kiibohd-core is a re-implementation of the kiibohd firmware as a collection of rust modules.
Initially kiibohd-core is meant to be used as an FFI interface to extend the C-based kiibohd firmware (https://github.com/kiibohd/controller).
Eventually the firmware will be converted entirely to rust (though this may take a while).

This library is meant to be built for many platforms.
Generally these are tested:

* thumbv7em-none-eabi
* x86_64-unknown-linux-gnu


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

```bash
cargo test --all --features std
```

**NOTE**: `--features std` has to be added to cargo test to get around unfortunate problems with how `no_std` and test code works.
