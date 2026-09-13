#!/usr/bin/env bash
set -euo pipefail

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
fi
source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  cargo install wasm-bindgen-cli --version 0.2.114
fi

rm -rf dist target
cargo build --release --target wasm32-unknown-unknown
mkdir -p dist/wasm
wasm-bindgen --target web --out-dir dist/wasm target/wasm32-unknown-unknown/release/la.wasm
cp web/index.html dist/index.html
cp web/styles.css dist/styles.css
