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

  Create a new TOC entry.

  

  # Arguments

  

  * `title` - Display title for the entry

  * `anchor` - Anchor link target (without `#`)

- <span id="tocentry-with-children"></span>`fn with_children(title: impl Into<String>, anchor: impl Into<String>, children: Vec<Self>) -> Self`

  Create a new TOC entry with children.

  

  # Arguments

  

  * `title` - Display title for the entry

  * `anchor` - Anchor link target (without `#`)

  * `children` - Child entries for nested items

- <span id="tocentry-count"></span>`fn count(&self) -> usize`

  Count total items in this entry and all descendants.

#### Trait Implementations

##### `impl Any for TocEntry`

- <span id="tocentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TocEntry`

- <span id="tocentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TocEntry`

- <span id="tocentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TocEntry`

- <span id="tocentry-clone"></span>`fn clone(&self) -> TocEntry` — [`TocEntry`](#tocentry)

##### `impl CloneToUninit for TocEntry`

- <span id="tocentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TocEntry`

- <span id="tocentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TocEntry`

- <span id="tocentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for TocEntry`

##### `impl<U> Into for TocEntry`

- <span id="tocentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TocEntry`

##### `impl OwoColorize for TocEntry`

##### `impl Pointable for TocEntry`

- <span id="tocentry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tocentry-pointable-type-init"></span>`type Init = T`

- <span id="tocentry-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tocentry-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tocentry-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tocentry-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TocEntry`

- <span id="tocentry-toowned-type-owned"></span>`type Owned = T`

- <span id="tocentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tocentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TocEntry`

- <span id="tocentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tocentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TocEntry`

- <span id="tocentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tocentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

  Create a new TOC generator with the given threshold.

  

  # Arguments

  

  * `threshold` - Minimum number of items required to generate a TOC

- <span id="tocgenerator-generate"></span>`fn generate(&self, entries: &[TocEntry]) -> Option<String>` — [`TocEntry`](#tocentry)

  Generate a markdown table of contents from the given entries.

  

  Returns `None` if the total item count is below the threshold.

  

  # Arguments

  

  * `entries` - Top-level TOC entries (typically section headings)

  

  # Returns

  

  A formatted markdown string with the TOC, or `None` if below threshold.

- <span id="tocgenerator-render-entry"></span>`fn render_entry(md: &mut String, entry: &TocEntry, depth: usize)` — [`TocEntry`](#tocentry)

  Render a single TOC entry with proper indentation.

#### Trait Implementations

##### `impl Any for TocGenerator`

- <span id="tocgenerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TocGenerator`

- <span id="tocgenerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TocGenerator`

- <span id="tocgenerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TocGenerator`

- <span id="tocgenerator-clone"></span>`fn clone(&self) -> TocGenerator` — [`TocGenerator`](#tocgenerator)

##### `impl CloneToUninit for TocGenerator`

- <span id="tocgenerator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TocGenerator`

- <span id="tocgenerator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for TocGenerator`

- <span id="tocgenerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for TocGenerator`

##### `impl<U> Into for TocGenerator`

- <span id="tocgenerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for TocGenerator`

##### `impl OwoColorize for TocGenerator`

##### `impl Pointable for TocGenerator`

- <span id="tocgenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="tocgenerator-pointable-type-init"></span>`type Init = T`

- <span id="tocgenerator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="tocgenerator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="tocgenerator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="tocgenerator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for TocGenerator`

- <span id="tocgenerator-toowned-type-owned"></span>`type Owned = T`

- <span id="tocgenerator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tocgenerator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TocGenerator`

- <span id="tocgenerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tocgenerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TocGenerator`

- <span id="tocgenerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tocgenerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for TocGenerator`

