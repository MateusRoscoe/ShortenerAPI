# ShortenerAPI

## Setting development environment

- Install Rust
- Install Clippy `rustup component add clippy`
- Install cargo-watch `cargo install cargo-watch`
- Run with `cargo-watch -qc -x run -x clippy`
- Recommended VS Code settings

  ```
  "[rust]": {
    "editor.formatOnPaste": true,
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
  ```

## Recommended VS Code Extensions

- Crates
- Even Better TOML
- rust-analyzer
