*[cargo_docs_md](../../index.md) / [generator](../index.md) / [flat](index.md)*

---

# Module `flat`

Flat format documentation generation.

This module provides the [`FlatGenerator`](#flatgenerator) struct which generates markdown
documentation with a flat file structure where all files are in a single
directory and module hierarchy is encoded in filenames.

## Structs

### `FlatGenerator<'a>`

```rust
struct FlatGenerator<'a> {
    ctx: &'a crate::generator::context::GeneratorContext<'a>,
    output_dir: &'a std::path::Path,
    progress: &'a indicatif::ProgressBar,
}
```

Generates documentation with flat file structure.

All markdown files are placed in a single directory. Module hierarchy
is encoded in filenames using double underscores as separators.

# Output Structure

```text
output/
├── index.md              # Crate root
├── module_a.md           # Top-level module
├── module_b.md           # Top-level module
├── module_a__child.md    # Nested module (module_a::child)
└── module_a__child__deep.md  # Deeply nested
```

#### Fields

- **`ctx`**: `&'a crate::generator::context::GeneratorContext<'a>`

  Shared generator context.

- **`output_dir`**: `&'a std::path::Path`

  Output directory path.

- **`progress`**: `&'a indicatif::ProgressBar`

  Progress bar for user feedback.

#### Implementations

- `const fn new(ctx: &'a GeneratorContext<'a>, output_dir: &'a Path, progress: &'a ProgressBar) -> Self` — [`GeneratorContext`](../index.md)

- `fn generate(self: &Self, root: &Item) -> Result<(), Error>` — [`Error`](../../error/index.md)

- `fn generate_module(self: &Self, item: &Item) -> Result<(), Error>` — [`Error`](../../error/index.md)

- `fn generate_module_recursive(self: &Self, item: &Item, prefix: &str) -> Result<(), Error>` — [`Error`](../../error/index.md)

#### Trait Implementations

##### `impl<T> Instrument for FlatGenerator<'a>`

##### `impl<T> IntoEither for FlatGenerator<'a>`

##### `impl<D> OwoColorize for FlatGenerator<'a>`

##### `impl<T> Pointable for FlatGenerator<'a>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for FlatGenerator<'a>`

