# Workshop / Rust + WebAssembly

A visual prototype of the Workshop idea built with Rust and WebAssembly.

The browser shell is intentionally small. The application state, rendering and interactions are driven from Rust compiled to WebAssembly.

## Local development

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.114
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir dist/wasm target/wasm32-unknown-unknown/release/la.wasm
cp web/index.html dist/index.html
cp web/styles.css dist/styles.css
python3 -m http.server 8080 -d dist
```

Open `http://localhost:8080`.
