*[cargo_docs_md](../../index.md) / [generator](../index.md) / [capture](index.md)*

---

# Module `capture`

In-memory markdown capture for testing.

This module provides [`MarkdownCapture`](#markdowncapture) for capturing generated markdown
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

*Defined in `src/generator/capture.rs:15-18`*

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

  Create a new empty capture.

- <span id="markdowncapture-insert"></span>`fn insert(&mut self, path: String, content: String)`

  Add a file to the capture.

  

  # Arguments

  * `path` - Relative path of the file (e.g., "index.md" or "span/index.md")

  * `content` - The markdown content for this file

- <span id="markdowncapture-get"></span>`fn get(&self, path: &str) -> Option<&String>`

  Get the content of a specific file.

- <span id="markdowncapture-paths"></span>`fn paths(&self) -> Vec<&String>`

  Get all file paths in sorted order.

- <span id="markdowncapture-len"></span>`fn len(&self) -> usize`

  Get the number of captured files.

- <span id="markdowncapture-is-empty"></span>`fn is_empty(&self) -> bool`

  Check if the capture is empty.

- <span id="markdowncapture-to-snapshot-string"></span>`fn to_snapshot_string(&self) -> String`

  Convert all captured files to a single string for snapshot testing.

  

  Files are sorted by path and separated with clear headers.

- <span id="markdowncapture-into-inner"></span>`fn into_inner(self) -> HashMap<String, String>`

  Consume self and return the underlying `HashMap`.

#### Trait Implementations

##### `impl Any for MarkdownCapture`

- <span id="markdowncapture-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MarkdownCapture`

- <span id="markdowncapture-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MarkdownCapture`

- <span id="markdowncapture-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for MarkdownCapture`

- <span id="markdowncapture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MarkdownCapture`

- <span id="markdowncapture-default"></span>`fn default() -> MarkdownCapture` — [`MarkdownCapture`](#markdowncapture)

##### `impl<T> From for MarkdownCapture`

- <span id="markdowncapture-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for MarkdownCapture`

##### `impl<U> Into for MarkdownCapture`

- <span id="markdowncapture-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for MarkdownCapture`

##### `impl OwoColorize for MarkdownCapture`

##### `impl Pointable for MarkdownCapture`

- <span id="markdowncapture-pointable-const-align"></span>`const ALIGN: usize`

- <span id="markdowncapture-pointable-type-init"></span>`type Init = T`

- <span id="markdowncapture-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="markdowncapture-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="markdowncapture-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="markdowncapture-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for MarkdownCapture`

- <span id="markdowncapture-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="markdowncapture-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MarkdownCapture`

- <span id="markdowncapture-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="markdowncapture-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for MarkdownCapture`

