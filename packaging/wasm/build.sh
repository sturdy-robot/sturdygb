#!/usr/bin/env bash
set -e

echo "Building sturdygb for WebAssembly..."

# Move to the root of the project
cd "$(dirname "$0")/../.."

# Build the WASM binary using cargo
cargo build --release --bin sturdygb_bin --target wasm32-unknown-unknown

echo "Running wasm-bindgen..."
# Use wasm-bindgen to generate the JavaScript bindings
# Note: Ensure you have `wasm-bindgen-cli` installed matching the version in Cargo.toml.
# e.g., cargo install -f wasm-bindgen-cli --version 0.2.x
wasm-bindgen target/wasm32-unknown-unknown/release/sturdygb_bin.wasm \
  --out-dir packaging/wasm/pkg \
  --target web \
  --no-typescript

echo "Done! You can serve the packaging/wasm directory using a local web server."
echo "Example: python3 -m http.server -d packaging/wasm"
