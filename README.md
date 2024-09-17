# Babelmark Dingus server for Rust markdown crates

[Babelmark](https://babelmark.github.io/) is a tool to compare the output of markdown implementations.

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
    // ...
    "crate_name", module_name, false, "https://github.com/example/repo"
}
```

  - `crate_name` MUST be the exact name of the crate as per Cargo.toml
  - `module_name` is the filename of the .rs created in step 3
  - the boolean indicates commonmark compliance
  - the URL must be to the repo of the markdown implementation

5. Submit a pull request!
6. Once approved and deployed, go to <https://markdown-dingus.shuttleapp.rs/registry>
7. Submit your crate's new entry in the registry to <https://github.com/babelmark/babelmark-registry>
