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

  Create a new flat generator.

  

  # Arguments

  

  * `ctx` - Shared generator context

  * `output_dir` - Directory to write markdown files to

  * `progress` - Progress bar for user feedback

- <span id="flatgenerator-generate"></span>`fn generate(&self, root: &Item) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

  Generate all documentation files in flat format.

  

  Generates `index.md` for the root module, then recursively generates

  files for all submodules with flattened names.

- <span id="flatgenerator-generate-module"></span>`fn generate_module(&self, item: &Item) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

  Generate a single module file and its children.

  

  Creates `{module_name}.md` in the output directory and recursively

  generates child modules with flattened names (e.g., `parent__child.md`).

- <span id="flatgenerator-generate-module-recursive"></span>`fn generate_module_recursive(&self, item: &Item, prefix: &str) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

  Recursively generate nested module files with flattened names.

  

  # Arguments

  

  * `item` - The module item to generate

  * `prefix` - Accumulated path prefix (e.g., "`parent__child`")

#### Trait Implementations

##### `impl Any for FlatGenerator<'a>`

- <span id="flatgenerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FlatGenerator<'a>`

- <span id="flatgenerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FlatGenerator<'a>`

- <span id="flatgenerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for FlatGenerator<'a>`

- <span id="flatgenerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for FlatGenerator<'a>`

##### `impl<U> Into for FlatGenerator<'a>`

- <span id="flatgenerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for FlatGenerator<'a>`

##### `impl OwoColorize for FlatGenerator<'a>`

##### `impl Pointable for FlatGenerator<'a>`

- <span id="flatgenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="flatgenerator-pointable-type-init"></span>`type Init = T`

- <span id="flatgenerator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="flatgenerator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="flatgenerator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="flatgenerator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for FlatGenerator<'a>`

- <span id="flatgenerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="flatgenerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FlatGenerator<'a>`

- <span id="flatgenerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="flatgenerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for FlatGenerator<'a>`

