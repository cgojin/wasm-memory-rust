# wasm-memory

Webassembly memory management with Rust.

```sh
# Add wasm32-unknown-unknown target, first only
rustup target add wasm32-unknown-unknown

# Build wasm
cargo build --target wasm32-unknown-unknown

# Run wasm
cargo run --example app

# Run lib
cargo run

# Detect memory leaks on macOS
leaks --atExit -- target/debug/wasm-memory
leaks --atExit -- target/debug/examples/app

# Detect memory leaks on Linux
valgrind target/debug/wasm-memory
valgrind target/debug/examples/app
```