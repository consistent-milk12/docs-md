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

*Defined in `src/generator/nested.rs:35-44`*

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

- <span id="nestedgenerator-new"></span>`const fn new(ctx: &'a GeneratorContext<'a>, output_dir: &'a Path, progress: &'a ProgressBar) -> Self` — [`GeneratorContext`](../context/index.md#generatorcontext)

  Create a new nested generator.

  

  # Arguments

  

  * `ctx` - Shared generator context

  * `output_dir` - Directory to write markdown files to

  * `progress` - Progress bar for user feedback

- <span id="nestedgenerator-generate"></span>`fn generate(&self, root: &Item) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

  Generate all documentation files in nested format.

  

  Generates `index.md` for the root module in the output directory,

  then recursively creates subdirectories for each submodule.

- <span id="nestedgenerator-generate-module"></span>`fn generate_module(&self, item: &Item, parent_dir: &Path, module_path: Vec<String>) -> Result<(), Error>` — [`Error`](../../error/index.md#error)

  Generate a single module directory with index.md and child modules.

  

  Creates a directory for the module and an `index.md` file inside it.

  Recursively creates subdirectories for child modules. Now tracks the

  full module path for breadcrumb generation.

  

  # Arguments

  

  * `item` - The module item to generate

  * `parent_dir` - Parent directory to create module directory in

  * `module_path` - Accumulated module path segments for breadcrumbs

#### Trait Implementations

##### `impl Any for NestedGenerator<'a>`

- <span id="nestedgenerator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NestedGenerator<'a>`

- <span id="nestedgenerator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NestedGenerator<'a>`

- <span id="nestedgenerator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> From for NestedGenerator<'a>`

- <span id="nestedgenerator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Instrument for NestedGenerator<'a>`

##### `impl<U> Into for NestedGenerator<'a>`

- <span id="nestedgenerator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoEither for NestedGenerator<'a>`

##### `impl OwoColorize for NestedGenerator<'a>`

##### `impl Pointable for NestedGenerator<'a>`

- <span id="nestedgenerator-pointable-const-align"></span>`const ALIGN: usize`

- <span id="nestedgenerator-pointable-type-init"></span>`type Init = T`

- <span id="nestedgenerator-pointable-init"></span>`unsafe fn init(init: <T as Pointable>::Init) -> usize`

- <span id="nestedgenerator-pointable-deref"></span>`unsafe fn deref<'a>(ptr: usize) -> &'a T`

- <span id="nestedgenerator-pointable-deref-mut"></span>`unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- <span id="nestedgenerator-pointable-drop"></span>`unsafe fn drop(ptr: usize)`

##### `impl<U> TryFrom for NestedGenerator<'a>`

- <span id="nestedgenerator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nestedgenerator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NestedGenerator<'a>`

- <span id="nestedgenerator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nestedgenerator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl WithSubscriber for NestedGenerator<'a>`

