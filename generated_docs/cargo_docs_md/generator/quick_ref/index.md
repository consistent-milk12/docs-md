*[cargo_docs_md](../../index.md) / [generator](../index.md) / [quick_ref](index.md)*

---

# Module `quick_ref`

Quick Reference table generation for module documentation.

This module provides [`QuickRefGenerator`](../index.md) which generates a markdown table
summarizing all public items in a module at a glance. The table shows item name,
kind, and first-sentence description.

# Example Output

```markdown
## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parser`](#parser) | struct | JSON parser with streaming support |
| [`Value`](#value) | enum | Dynamic JSON value type |
| [`parse`](#parse) | fn | Parses a JSON string into a Value |
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`QuickRefEntry`](#quickrefentry) | struct | An entry in the quick reference table. |
| [`QuickRefGenerator`](#quickrefgenerator) | struct | Generator for markdown quick reference tables. |
| [`extract_summary`](#extract_summary) | fn | Extract the first sentence from a documentation string. |
| [`extract_first_sentence`](#extract_first_sentence) | fn | Extract the first sentence from a line of text. |

## Structs

### `QuickRefEntry`

```rust
struct QuickRefEntry {
    pub name: String,
    pub kind: &'static str,
    pub anchor: String,
    pub summary: String,
}
```

An entry in the quick reference table.

Each entry represents a single public item with its name, kind,
anchor link, and first-sentence summary.

#### Fields

- **`name`**: `String`

  Display name for this entry.

- **`kind`**: `&'static str`

  Item kind (struct, enum, trait, fn, etc.).

- **`anchor`**: `String`

  Anchor link target (without the `#` prefix).

- **`summary`**: `String`

  First-sentence summary from doc comment.

#### Implementations

- <span id="quickrefentry-new"></span>`fn new(name: impl Into<String>, kind: &'static str, anchor: impl Into<String>, summary: impl Into<String>) -> Self`

#### Trait Implementations

##### `impl Clone for QuickRefEntry`

- <span id="quickrefentry-clone"></span>`fn clone(&self) -> QuickRefEntry` — [`QuickRefEntry`](../index.md)

##### `impl Debug for QuickRefEntry`

- <span id="quickrefentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Instrument for QuickRefEntry`

##### `impl<T> IntoEither for QuickRefEntry`

##### `impl<D> OwoColorize for QuickRefEntry`

##### `impl<T> Pointable for QuickRefEntry`

- <span id="quickrefentry-align"></span>`const ALIGN: usize`

- <span id="quickrefentry-init"></span>`type Init = T`

- <span id="quickrefentry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="quickrefentry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="quickrefentry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="quickrefentry-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for QuickRefEntry`

### `QuickRefGenerator`

```rust
struct QuickRefGenerator;
```

Generator for markdown quick reference tables.

The generator creates a table summarizing all items with links,
kinds, and first-sentence descriptions.

#### Implementations

- <span id="quickrefgenerator-new"></span>`const fn new() -> Self`

- <span id="quickrefgenerator-generate"></span>`fn generate(&self, entries: &[QuickRefEntry]) -> String` — [`QuickRefEntry`](../index.md)

#### Trait Implementations

##### `impl Clone for QuickRefGenerator`

- <span id="quickrefgenerator-clone"></span>`fn clone(&self) -> QuickRefGenerator` — [`QuickRefGenerator`](../index.md)

##### `impl Debug for QuickRefGenerator`

- <span id="quickrefgenerator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for QuickRefGenerator`

- <span id="quickrefgenerator-default"></span>`fn default() -> QuickRefGenerator` — [`QuickRefGenerator`](../index.md)

##### `impl<T> Instrument for QuickRefGenerator`

##### `impl<T> IntoEither for QuickRefGenerator`

##### `impl<D> OwoColorize for QuickRefGenerator`

##### `impl<T> Pointable for QuickRefGenerator`

- <span id="quickrefgenerator-align"></span>`const ALIGN: usize`

- <span id="quickrefgenerator-init"></span>`type Init = T`

- <span id="quickrefgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="quickrefgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="quickrefgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="quickrefgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for QuickRefGenerator`

## Functions

### `extract_summary`

```rust
fn extract_summary(docs: Option<&str>) -> String
```

Extract the first sentence from a documentation string.

This function finds the first non-empty line and extracts the first
sentence (text up to the first `. ` or end of line).

# Arguments

* `docs` - Optional documentation string

# Returns

The first sentence, or an empty string if no docs.

# Examples

```ignore
assert_eq!(extract_summary(Some("A parser. With more.")), "A parser");
assert_eq!(extract_summary(Some("Single sentence")), "Single sentence");
assert_eq!(extract_summary(None), "");
```

### `extract_first_sentence`

```rust
fn extract_first_sentence(text: &str) -> &str
```

Extract the first sentence from a line of text.

Handles edge cases like:
- "e.g." and "i.e." abbreviations
- Version numbers like "1.0"
- URLs with dots

