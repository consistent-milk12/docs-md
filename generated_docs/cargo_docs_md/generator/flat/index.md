*[cargo_docs_md](../../index.md) / [generator](../index.md) / [flat](index.md)*

---

# Module `flat`

Flat format documentation generation.

This module provides the [`FlatGenerator`](#flatgenerator) struct which generates markdown
documentation with a flat file structure where all files are in a single
directory and module hierarchy is encoded in filenames.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlatGenerator`](#flatgenerator) | struct | Generates documentation with flat file structure. |

## Structs

### `FlatGenerator<'a>`

```rust
struct FlatGenerator<'a> {
    ctx: &'a crate::generator::context::GeneratorContext<'a>,
    output_dir: &'a std::path::Path,
    progress: &'a indicatif::ProgressBar,
}
```

*Defined in `src/generator/flat.rs:32-41`*

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

- <span id="flatgenerator-new"></span>`const fn new(ctx: &'a GeneratorContext<'a>, output_dir: &'a Path, progress: &'a ProgressBar) -> Self` — [`GeneratorContext`](../context/index.md#generatorcontext)

- <span id="flatgenerator-generate"></span>`fn generate(&self, root: &Item) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="flatgenerator-generate-module"></span>`fn generate_module(&self, item: &Item) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

- <span id="flatgenerator-generate-module-recursive"></span>`fn generate_module_recursive(&self, item: &Item, prefix: &str) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

#### Trait Implementations

##### `impl Instrument for FlatGenerator<'a>`

##### `impl IntoEither for FlatGenerator<'a>`

##### `impl OwoColorize for FlatGenerator<'a>`

##### `impl Pointable for FlatGenerator<'a>`

- <span id="flatgenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatgenerator-pointable-type-init"></span>`type Init = T`

- <span id="flatgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl WithSubscriber for FlatGenerator<'a>`

