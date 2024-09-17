# Babelmark Dingus server for Rust markdown crates

Babelmark is a tool to compare the output of markdown implementations.

This repository is the source of <https://markdown-dingus.shuttleapp.rs>, which provides Babelmark with various Rust-based markdown parsers/renderers.

## How to add an implementation

1. Clone this repo.
2. Add the markdown crate to [Cargo.toml](./Cargo.toml).
3. Create a render module named after the crate: `src/cratename.rs`:

```rust
use axum::response::Result;

pub async fn render(input: &str) -> Result<String> {
    Ok(cratename::render_markdown_to_html_somehow(input))
}
```

4. Add the implementation to the `main.rs` `rendefers!` macro:

```rust
renderers! {
    // name of the crate, name of the module (.rs file), version, implements commonmark?  crate repository
    "project name",       module_name,                   "1.2.3", false,                  "https://github.com/example/repo"
}
```

5. Submit a pull request!
6. Once approved and deployed, go to <https://markdown-dingus.shuttleapp.rs/registry">
7. Submit your crate's new entry in the registry to <https://github.com/babelmark/babelmark-registry>
