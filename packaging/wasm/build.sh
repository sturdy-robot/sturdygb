#!/usr/bin/env bash
set -e

echo "Building sturdygb for WebAssembly..."

# Move to the repository root.
cd "$(dirname "$0")/../.."

find_cmd() {
  local name="$1"
  local candidate

  if command -v "$name" >/dev/null 2>&1; then
    command -v "$name"
    return 0
  fi

  for candidate in \
    "${USERPROFILE:-}/.cargo/bin/${name}.exe" \
    "${HOME:-}/.cargo/bin/${name}.exe" \
    "/c/Users/${USER:-}/.cargo/bin/${name}.exe" \
    "/mnt/c/Users/${USER:-}/.cargo/bin/${name}.exe"
  do
    if [ -n "$candidate" ] && [ -x "$candidate" ]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done

  return 1
}

CARGO_CMD="$(find_cmd cargo)" || {
  echo "cargo was not found on PATH or in a known cargo bin directory" >&2
  exit 1
}

WASM_BINDGEN_CMD="$(find_cmd wasm-bindgen)" || {
  echo "wasm-bindgen was not found on PATH or in a known cargo bin directory" >&2
  exit 1
}

# Build the WASM library using cargo so wasm-bindgen can expose the manual JS API.
"${CARGO_CMD}" build --release --lib --target wasm32-unknown-unknown

echo "Running wasm-bindgen..."
mkdir -p packaging/wasm/pkg
rm -f \
  packaging/wasm/pkg/sturdygb.js \
  packaging/wasm/pkg/sturdygb_bg.wasm \
  packaging/wasm/pkg/sturdygb_bin.js \
  packaging/wasm/pkg/sturdygb_bin_bg.wasm

# Use wasm-bindgen to generate the JavaScript bindings
# Note: Ensure you have `wasm-bindgen-cli` installed matching the version in Cargo.toml.
# e.g., cargo install -f wasm-bindgen-cli --version 0.2.x
"${WASM_BINDGEN_CMD}" target/wasm32-unknown-unknown/release/sturdygb.wasm \
  --out-dir packaging/wasm/pkg \
  --out-name sturdygb \
  --target web \
  --no-typescript

echo "Done! You can serve the packaging/wasm directory using a local web server."
echo "Example: python3 -m http.server -d packaging/wasm"
