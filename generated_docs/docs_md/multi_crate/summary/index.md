*[docs_md](../../index.md) / [multi_crate](../index.md) / [summary](index.md)*

---

# Module `summary`

mdBook SUMMARY.md generator.

This module provides [`SummaryGenerator`](../index.md) which creates a SUMMARY.md file
compatible with mdBook for multi-crate documentation.

## Structs

### `SummaryGenerator<'a>`

```rust
struct SummaryGenerator<'a> {
    crates: &'a crate::multi_crate::CrateCollection,
    output_dir: &'a std::path::Path,
    include_private: bool,
}
```

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

- `const fn new(crates: &'a CrateCollection, output_dir: &'a Path, include_private: bool) -> Self` — [`CrateCollection`](../../index.md)

- `fn generate(self: &Self) -> Result<(), Error>` — [`Error`](../../error/index.md)

- `fn add_modules(self: &Self, content: &mut String, krate: &rustdoc_types::Crate, items: &[rustdoc_types::Id], path_prefix: &str, indent: usize)`

#### Trait Implementations

##### `impl<T> Instrument for SummaryGenerator<'a>`

##### `impl<T> IntoEither for SummaryGenerator<'a>`

##### `impl<D> OwoColorize for SummaryGenerator<'a>`

##### `impl<T> Pointable for SummaryGenerator<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for SummaryGenerator<'a>`

