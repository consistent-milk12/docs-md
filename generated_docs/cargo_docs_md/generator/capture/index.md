*[cargo_docs_md](../../index.md) / [generator](../index.md) / [capture](index.md)*

---

# Module `capture`

In-memory markdown capture for testing.

This module provides [`MarkdownCapture`](../../index.md) for capturing generated markdown
in memory instead of writing to disk, enabling snapshot testing.

## Structs

### `MarkdownCapture`

```rust
struct MarkdownCapture {
    files: std::collections::HashMap<String, String>,
}
```

Captures generated markdown in memory for testing.

Instead of writing files to disk, this struct stores all generated
markdown content in a `HashMap`, keyed by relative file path. This
enables snapshot testing and verification of output without filesystem
side effects.

#### Fields

- **`files`**: `std::collections::HashMap<String, String>`

  Maps file paths (relative to output directory) to their generated content.

#### Implementations

- `fn new() -> Self`

- `fn insert(self: &mut Self, path: String, content: String)`

- `fn get(self: &Self, path: &str) -> Option<&String>`

- `fn paths(self: &Self) -> Vec<&String>`

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn to_snapshot_string(self: &Self) -> String`

- `fn into_inner(self: Self) -> HashMap<String, String>`

#### Trait Implementations

##### `impl Debug for MarkdownCapture`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MarkdownCapture`

- `fn default() -> MarkdownCapture` — [`MarkdownCapture`](../../index.md)

##### `impl<T> Instrument for MarkdownCapture`

##### `impl<T> IntoEither for MarkdownCapture`

##### `impl<D> OwoColorize for MarkdownCapture`

##### `impl<T> Pointable for MarkdownCapture`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for MarkdownCapture`

