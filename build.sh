#!/usr/bin/env bash
set -euo pipefail

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
fi

source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown

WASM_BINDGEN_VERSION="0.2.114"
WASM_BINDGEN_DIR="$HOME/.local/bin"
WASM_BINDGEN="$WASM_BINDGEN_DIR/wasm-bindgen"

if [ ! -x "$WASM_BINDGEN" ]; then
  mkdir -p "$WASM_BINDGEN_DIR"
  tmp_dir="$(mktemp -d)"
  trap 'rm -rf "$tmp_dir"' EXIT
  curl -fsSL "https://github.com/rustwasm/wasm-bindgen/releases/download/${WASM_BINDGEN_VERSION}/wasm-bindgen-${WASM_BINDGEN_VERSION}-x86_64-unknown-linux-musl.tar.gz" \
    | tar -xz -C "$tmp_dir"
  install -m 755 "$tmp_dir/wasm-bindgen-${WASM_BINDGEN_VERSION}-x86_64-unknown-linux-musl/wasm-bindgen" "$WASM_BINDGEN"
fi

export PATH="$WASM_BINDGEN_DIR:$PATH"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"

rm -rf dist
echo "Building Workshop Rust core..."
cargo build --release --target wasm32-unknown-unknown

echo "Generating browser WASM bindings..."
mkdir -p dist/wasm
wasm-bindgen --target web --out-dir dist/wasm target/wasm32-unknown-unknown/release/la.wasm

cp web/index.html dist/index.html
cp web/styles.css dist/styles.css

echo "Workshop build complete."
