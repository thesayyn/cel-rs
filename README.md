# CEL Rust
[playground](https://thesayyn.github.io/cel-rs/)

# Requirements
- NodeJS
- wasm-pack
- Rust toolchain

# Test

```sh
cargo test
```

## WASM 

```sh
# build
wasm-pack build web --target web
# tests
npx http-server web
```
