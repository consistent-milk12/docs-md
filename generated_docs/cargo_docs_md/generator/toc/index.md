*[cargo_docs_md](../../index.md) / [generator](../index.md) / [toc](index.md)*

---

# Module `toc`

Table of Contents generation for module documentation.

This module provides [`TocGenerator`](#tocgenerator) which generates a markdown table of contents
for modules that exceed a configurable item threshold. The TOC provides navigation
to major sections (Types, Traits, Functions, etc.) with nested links to individual items.

# Example Output

```markdown
## Contents

- [Structs](#structs)
  - [`Parser`](#parser)
  - [`Config`](#config)
- [Enums](#enums)
  - [`Error`](#error)
- [Functions](#functions)
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TocEntry`](#tocentry) | struct | An entry in the table of contents. |
| [`TocGenerator`](#tocgenerator) | struct | Generator for markdown table of contents. |

## Structs

### `TocEntry`

```rust
struct TocEntry {
    pub title: String,
    pub anchor: String,
    pub children: Vec<Self>,
}
```

*Defined in `src/generator/toc.rs:28-37`*

An entry in the table of contents.

Each entry represents either a section heading (like "Structs") or an
individual item (like a specific struct name). Entries can have children
for nested navigation.

#### Fields

- **`title`**: `String`

  Display title for this entry.

- **`anchor`**: `String`

  Anchor link target (without the `#` prefix).

- **`children`**: `Vec<Self>`

  Child entries for nested navigation.

#### Implementations

- <span id="tocentry-new"></span>`fn new(title: impl Into<String>, anchor: impl Into<String>) -> Self`

- <span id="tocentry-with-children"></span>`fn with_children(title: impl Into<String>, anchor: impl Into<String>, children: Vec<Self>) -> Self`

- <span id="tocentry-count"></span>`fn count(&self) -> usize`

#### Trait Implementations

##### `impl Clone for TocEntry`

- <span id="tocentry-clone"></span>`fn clone(&self) -> TocEntry` — [`TocEntry`](#tocentry)

##### `impl Debug for TocEntry`

- <span id="tocentry-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for TocEntry`

##### `impl IntoEither for TocEntry`

##### `impl OwoColorize for TocEntry`

##### `impl Pointable for TocEntry`

- <span id="tocentry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tocentry-pointable-type-init"></span>`type Init = T`

- <span id="tocentry-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tocentry-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tocentry-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tocentry-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TocEntry`

### `TocGenerator`

```rust
struct TocGenerator {
    threshold: usize,
}
```

*Defined in `src/generator/toc.rs:88-91`*

Generator for markdown table of contents.

The generator only produces output when the total number of items
exceeds the configured threshold. This prevents cluttering small
modules with unnecessary navigation.

#### Fields

- **`threshold`**: `usize`

  Minimum items required to generate a TOC.

#### Implementations

- <span id="tocgenerator-new"></span>`const fn new(threshold: usize) -> Self`

- <span id="tocgenerator-generate"></span>`fn generate(&self, entries: &[TocEntry]) -> Option<String>` — [`TocEntry`](#tocentry)

- <span id="tocgenerator-render-entry"></span>`fn render_entry(md: &mut String, entry: &TocEntry, depth: usize)` — [`TocEntry`](#tocentry)

#### Trait Implementations

##### `impl Clone for TocGenerator`

- <span id="tocgenerator-clone"></span>`fn clone(&self) -> TocGenerator` — [`TocGenerator`](#tocgenerator)

##### `impl Debug for TocGenerator`

- <span id="tocgenerator-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Instrument for TocGenerator`

##### `impl IntoEither for TocGenerator`

##### `impl OwoColorize for TocGenerator`

##### `impl Pointable for TocGenerator`

- <span id="tocgenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tocgenerator-pointable-type-init"></span>`type Init = T`

- <span id="tocgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tocgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tocgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tocgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for TocGenerator`

