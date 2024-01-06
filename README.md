## Buffalo

---


## Recommended IDE Setup


```

rustup target add wasm32-unknown-unknown
cargo build -Z build-std --target wasm32-unknown-unknown

cargo install cargo-wasi

cargo tauri info
rustup target add wasm32-unknown-unknown
cargo install trunk
cargo install wasm-bindgen-cli

```