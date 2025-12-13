*[cargo_docs_md](../../index.md) / [generator](../index.md) / [quick_ref](index.md)*

---

# Module `quick_ref`

Quick Reference table generation for module documentation.

This module provides [`QuickRefGenerator`](#quickrefgenerator) which generates a markdown table
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
| [`extract_summary`](#extract-summary) | fn | Extract the first sentence from a documentation string. |
| [`try_extract_sentence`](#try-extract-sentence) | fn | Try to extract a complete first sentence from text. |
| [`ABBREVIATIONS`](#abbreviations) | const | Common abbreviations that shouldn't end sentences. |

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

*Defined in `src/generator/quick_ref.rs:26-38`*

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

  Create a new quick reference entry.

  

  # Arguments

  

  * `name` - Display name for the entry

  * `kind` - Item kind (struct, enum, fn, etc.)

  * `anchor` - Anchor link target (without `#`)

  * `summary` - First-sentence summary

#### Trait Implementations

##### `impl Any for QuickRefEntry`

- <span id="quickrefentry-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for QuickRefEntry`

- <span id="quickrefentry-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for QuickRefEntry`

- <span id="quickrefentry-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for QuickRefEntry`

- <span id="quickrefentry-clone"></span>`fn clone(&self) -> QuickRefEntry` — [`QuickRefEntry`](#quickrefentry)

##### `impl CloneToUninit for QuickRefEntry`

- <span id="quickrefentry-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for QuickRefEntry`

- <span id="quickrefentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for QuickRefEntry`

- <span id="quickrefentry-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for QuickRefEntry`

##### `impl<U> Into for QuickRefEntry`

- <span id="quickrefentry-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for QuickRefEntry`

##### `impl OwoColorize for QuickRefEntry`

##### `impl Pointable for QuickRefEntry`

- <span id="quickrefentry-pointable-const-align"></span>`const ALIGN: usize`

- <span id="quickrefentry-pointable-type-init"></span>`type Init = T`

- <span id="quickrefentry-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="quickrefentry-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="quickrefentry-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="quickrefentry-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for QuickRefEntry`

- <span id="quickrefentry-toowned-type-owned"></span>`type Owned = T`

- <span id="quickrefentry-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="quickrefentry-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for QuickRefEntry`

- <span id="quickrefentry-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="quickrefentry-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for QuickRefEntry`

- <span id="quickrefentry-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="quickrefentry-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for QuickRefEntry`

### `QuickRefGenerator`

```rust
struct QuickRefGenerator;
```

*Defined in `src/generator/quick_ref.rs:70`*

Generator for markdown quick reference tables.

The generator creates a table summarizing all items with links,
kinds, and first-sentence descriptions.

#### Implementations

- <span id="quickrefgenerator-new"></span>`const fn new() -> Self`

  Create a new quick reference generator.

- <span id="quickrefgenerator-generate"></span>`fn generate(&self, entries: &[QuickRefEntry]) -> String` — [`QuickRefEntry`](#quickrefentry)

  Generate a markdown quick reference table from the given entries.

  

  Returns an empty string if there are no entries.

  

  # Arguments

  

  * `entries` - Quick reference entries to include in the table

  

  # Returns

  

  A formatted markdown table string.

#### Trait Implementations

##### `impl Any for QuickRefGenerator`

- <span id="quickrefgenerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for QuickRefGenerator`

- <span id="quickrefgenerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for QuickRefGenerator`

- <span id="quickrefgenerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for QuickRefGenerator`

- <span id="quickrefgenerator-clone"></span>`fn clone(&self) -> QuickRefGenerator` — [`QuickRefGenerator`](#quickrefgenerator)

##### `impl CloneToUninit for QuickRefGenerator`

- <span id="quickrefgenerator-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for QuickRefGenerator`

- <span id="quickrefgenerator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for QuickRefGenerator`

- <span id="quickrefgenerator-default"></span>`fn default() -> QuickRefGenerator` — [`QuickRefGenerator`](#quickrefgenerator)

##### `impl<T> From for QuickRefGenerator`

- <span id="quickrefgenerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for QuickRefGenerator`

##### `impl<U> Into for QuickRefGenerator`

- <span id="quickrefgenerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for QuickRefGenerator`

##### `impl OwoColorize for QuickRefGenerator`

##### `impl Pointable for QuickRefGenerator`

- <span id="quickrefgenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="quickrefgenerator-pointable-type-init"></span>`type Init = T`

- <span id="quickrefgenerator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="quickrefgenerator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="quickrefgenerator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="quickrefgenerator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl ToOwned for QuickRefGenerator`

- <span id="quickrefgenerator-toowned-type-owned"></span>`type Owned = T`

- <span id="quickrefgenerator-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="quickrefgenerator-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for QuickRefGenerator`

- <span id="quickrefgenerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="quickrefgenerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for QuickRefGenerator`

- <span id="quickrefgenerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="quickrefgenerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for QuickRefGenerator`

## Functions

### `extract_summary`

```rust
fn extract_summary(docs: Option<&str>) -> String
```

*Defined in `src/generator/quick_ref.rs:146-187`*

Extract the first sentence from a documentation string.

This function handles sentences that span multiple lines by joining
consecutive non-empty lines until a sentence boundary is found.
A blank line always terminates the paragraph.

# Arguments

* `docs` - Optional documentation string

# Returns

The first sentence, or an empty string if no docs.

# Examples

```ignore
assert_eq!(extract_summary(Some("A parser. With more.")), "A parser.");
assert_eq!(extract_summary(Some("Single sentence")), "Single sentence");
assert_eq!(extract_summary(None), "");
// Handles wrapped sentences:
assert_eq!(
    extract_summary(Some("A long sentence that\nspans multiple lines. More.")),
    "A long sentence that spans multiple lines."
);
```

### `try_extract_sentence`

```rust
fn try_extract_sentence(text: &str) -> Option<String>
```

*Defined in `src/generator/quick_ref.rs:199-231`*

Try to extract a complete first sentence from text.

Returns `Some(sentence)` if a sentence boundary (`. ` not part of abbreviation
or version number) is found, otherwise `None`.

## Constants

### `ABBREVIATIONS`
```rust
const ABBREVIATIONS: &[&str];
```

*Defined in `src/generator/quick_ref.rs:190-193`*

Common abbreviations that shouldn't end sentences.

