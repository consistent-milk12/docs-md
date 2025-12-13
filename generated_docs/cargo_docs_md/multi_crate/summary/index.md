*[cargo_docs_md](../../index.md) / [multi_crate](../index.md) / [summary](index.md)*

---

# Module `summary`

mdBook SUMMARY.md generator.

This module provides [`SummaryGenerator`](#summarygenerator) which creates a SUMMARY.md file
compatible with mdBook for multi-crate documentation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SummaryGenerator`](#summarygenerator) | struct | Generates mdBook-compatible SUMMARY.md file. |

## Structs

### `SummaryGenerator<'a>`

```rust
struct SummaryGenerator<'a> {
    crates: &'a crate::multi_crate::CrateCollection,
    output_dir: &'a std::path::Path,
    include_private: bool,
}
```

*Defined in `src/multi_crate/summary.rs:31-40`*

Generates mdBook-compatible SUMMARY.md file.

Creates a table of contents linking all crates and their modules,
allowing the documentation to be built as an mdBook site.

# Output Format

```markdown
Summary

- [tracing](tracing/index.md)
  - [span](tracing/span/index.md)
  - [field](tracing/field/index.md)
- [tracing_core](tracing_core/index.md)
  - [subscriber](tracing_core/subscriber/index.md)
```

#### Fields

- **`crates`**: `&'a crate::multi_crate::CrateCollection`

  Collection of crates to document.

- **`output_dir`**: `&'a std::path::Path`

  Output directory for SUMMARY.md.

- **`include_private`**: `bool`

  Whether to include private items.

#### Implementations

- <span id="summarygenerator-new"></span>`const fn new(crates: &'a CrateCollection, output_dir: &'a Path, include_private: bool) -> Self` — [`CrateCollection`](../collection/index.md#cratecollection)

  Create a new summary generator.

  

  # Arguments

  

  * `crates` - Collection of parsed crates

  * `output_dir` - Directory to write SUMMARY.md

  * `include_private` - Whether to include private modules

- <span id="summarygenerator-generate"></span>`fn generate(&self) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

  Generate the SUMMARY.md file.

  

  # Errors

  

  Returns an error if the file cannot be written.

- <span id="summarygenerator-add-modules"></span>`fn add_modules(&self, content: &mut String, krate: &rustdoc_types::Crate, items: &[rustdoc_types::Id], path_prefix: &str, indent: usize)`

  Add module entries recursively.

#### Trait Implementations

##### `impl Any for SummaryGenerator<'a>`

- <span id="summarygenerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SummaryGenerator<'a>`

- <span id="summarygenerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SummaryGenerator<'a>`

- <span id="summarygenerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for SummaryGenerator<'a>`

- <span id="summarygenerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for SummaryGenerator<'a>`

##### `impl<U> Into for SummaryGenerator<'a>`

- <span id="summarygenerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for SummaryGenerator<'a>`

##### `impl OwoColorize for SummaryGenerator<'a>`

##### `impl Pointable for SummaryGenerator<'a>`

- <span id="summarygenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="summarygenerator-pointable-type-init"></span>`type Init = T`

- <span id="summarygenerator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="summarygenerator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="summarygenerator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="summarygenerator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for SummaryGenerator<'a>`

- <span id="summarygenerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="summarygenerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SummaryGenerator<'a>`

- <span id="summarygenerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="summarygenerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for SummaryGenerator<'a>`

