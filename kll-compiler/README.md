# kll-compiler

Rust implementation of the KLL compiler.
Designed to be integrated into build.rs as a library or as a stand-alone utility.


## Usage

TODO


## Testing

```bash
cargo test

# To see verbose test output when debugging
RUST_LOG=trace cargo test emitters::kllcore::test::layer_lookup_simple -- --nocapture
```
