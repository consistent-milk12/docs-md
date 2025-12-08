*[cargo_docs_md](../../index.md) / [generator](../index.md) / [capture](index.md)*

---

# Module `capture`

In-memory markdown capture for testing.

This module provides [`MarkdownCapture`](../../index.md) for capturing generated markdown
in memory instead of writing to disk, enabling snapshot testing.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MarkdownCapture`](#markdowncapture) | struct | Captures generated markdown in memory for testing. |

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

- <span id="markdowncapture-new"></span>`fn new() -> Self`

- <span id="markdowncapture-insert"></span>`fn insert(&mut self, path: String, content: String)`

- <span id="markdowncapture-get"></span>`fn get(&self, path: &str) -> Option<&String>`

- <span id="markdowncapture-paths"></span>`fn paths(&self) -> Vec<&String>`

- <span id="markdowncapture-len"></span>`fn len(&self) -> usize`

- <span id="markdowncapture-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="markdowncapture-to-snapshot-string"></span>`fn to_snapshot_string(&self) -> String`

- <span id="markdowncapture-into-inner"></span>`fn into_inner(self) -> HashMap<String, String>`

#### Trait Implementations

##### `impl Debug for MarkdownCapture`

- <span id="markdowncapture-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MarkdownCapture`

- <span id="markdowncapture-default"></span>`fn default() -> MarkdownCapture` — [`MarkdownCapture`](../../index.md)

##### `impl<T> Instrument for MarkdownCapture`

##### `impl<T> IntoEither for MarkdownCapture`

##### `impl<D> OwoColorize for MarkdownCapture`

##### `impl<T> Pointable for MarkdownCapture`

- <span id="markdowncapture-align"></span>`const ALIGN: usize`

- <span id="markdowncapture-init"></span>`type Init = T`

- <span id="markdowncapture-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="markdowncapture-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="markdowncapture-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="markdowncapture-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for MarkdownCapture`

