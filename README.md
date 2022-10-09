# wasm ssr

### stack

- rust
- actix
- web_sys

### dev start

- download cargo-watch

- dev start

```
cargo watch -x run
```

- prod start

```
cargo run
```

### wasm

- reference: https://rustwasm.github.io/wasm-bindgen/introduction.html

```
wasm-pack build --release --target web
```
