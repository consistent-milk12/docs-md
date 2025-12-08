*[cargo_docs_md](../../index.md) / [generator](../index.md) / [nested](index.md)*

---

# Module `nested`

Nested format documentation generation.

This module provides the [`NestedGenerator`](#nestedgenerator) struct which generates markdown
documentation with a nested directory structure that mirrors the Rust module
hierarchy.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NestedGenerator`](#nestedgenerator) | struct | Generates documentation with nested directory structure. |

## Structs

### `NestedGenerator<'a>`

```rust
struct NestedGenerator<'a> {
    ctx: &'a crate::generator::context::GeneratorContext<'a>,
    output_dir: &'a std::path::Path,
    progress: &'a indicatif::ProgressBar,
}
```

Generates documentation with nested directory structure.

Each module gets its own directory with an `index.md` file.
This mirrors the Rust module hierarchy in the filesystem.

# Output Structure

```text
output/
├── index.md                  # Crate root
├── module_a/
│   ├── index.md              # module_a docs
│   └── child/
│       └── index.md          # module_a::child docs
└── module_b/
    └── index.md              # module_b docs
```

#### Fields

- **`ctx`**: `&'a crate::generator::context::GeneratorContext<'a>`

  Shared generator context.

- **`output_dir`**: `&'a std::path::Path`

  Output directory path.

- **`progress`**: `&'a indicatif::ProgressBar`

  Progress bar for user feedback.

#### Implementations

- <span id="nestedgenerator-new"></span>`const fn new(ctx: &'a GeneratorContext<'a>, output_dir: &'a Path, progress: &'a ProgressBar) -> Self` — [`GeneratorContext`](../index.md)

- <span id="nestedgenerator-generate"></span>`fn generate(&self, root: &Item) -> Result<(), Error>` — [`Error`](../../error/index.md)

- <span id="nestedgenerator-generate-module"></span>`fn generate_module(&self, item: &Item, parent_dir: &Path, module_path: Vec<String>) -> Result<(), Error>` — [`Error`](../../error/index.md)

#### Trait Implementations

##### `impl<T> Instrument for NestedGenerator<'a>`

##### `impl<T> IntoEither for NestedGenerator<'a>`

##### `impl<D> OwoColorize for NestedGenerator<'a>`

##### `impl<T> Pointable for NestedGenerator<'a>`

- <span id="nestedgenerator-align"></span>`const ALIGN: usize`

- <span id="nestedgenerator-init"></span>`type Init = T`

- <span id="nestedgenerator-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="nestedgenerator-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="nestedgenerator-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="nestedgenerator-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<T> WithSubscriber for NestedGenerator<'a>`

