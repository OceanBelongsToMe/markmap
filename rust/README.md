# Rust PoC workspace for markmap

This folder contains a small Cargo workspace for prototyping Rust -> WebAssembly modules
that can be consumed by the existing JS packages. Current crate:

- `common` — a tiny PoC that exports a `process_tree` function which accepts a JSON-like
  markmap tree and annotates each node with `payload.content_len`.

Usage and build

This workspace contains a pure Rust crate `common` meant for back-end use (e.g. Tauri)
and as a small CLI for testing. It no longer targets WebAssembly in this branch.

Build the library and CLI:

```bash
cd rust/common
cargo build --release
```

Run the CLI (reads stdin or file):

```bash
# read from file
target/release/common /path/to/tree.json

# read from stdin
cat /path/to/tree.json | target/release/common
```

Tauri integration (example)

In a Tauri application you can call this library directly from the Rust backend.
Add `markmap_common` as a workspace dependency in the Tauri `Cargo.toml` and call
`markmap_common::process_tree_value` or `process_tree_json` inside your command handler.

Example Tauri command (Rust):

```rust
#[tauri::command]
fn annotate_tree(json: String) -> Result<String, String> {
  markmap_common::process_tree_json(&json).map_err(|e| e.to_string())
}
```

Notes:

- This is a prototype: the intended next steps are to refine the JSON schema,
  add thorough tests, and wire the produced wasm into `packages/markmap-common` build.
