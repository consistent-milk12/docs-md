*[docs_md](../../index.md) / [generator](../index.md) / [breadcrumbs](index.md)*

---

# Module `breadcrumbs`

Breadcrumb navigation generation for nested module pages.

This module provides [`BreadcrumbGenerator`](#breadcrumbgenerator) which creates navigation
links showing the path from crate root to the current module.

## Structs

### `BreadcrumbGenerator<'a>`

```rust
struct BreadcrumbGenerator<'a> {
    module_path: &'a [String],
    crate_name: &'a str,
}
```

Generates breadcrumb navigation for nested module pages.

Creates a navigation line showing the path from the crate root to
the current module, with each segment being a clickable link.

#### Fields

- **`module_path`**: `&'a [String]`

  The module path segments (e.g., `["error", "types"]`).

- **`crate_name`**: `&'a str`

  The name of the crate for the root link.

#### Implementations

- `const fn new(module_path: &'a [String], crate_name: &'a str) -> Self`

- `fn generate(self: &Self) -> String`

#### Trait Implementations

##### `impl IntoEither<T>`

##### `impl OwoColorize<D>`

##### `impl Pointable<T>`

- `const ALIGN: usize`

- `type Init = T`

- `unsafe fn init(init: <T as Pointable>::Init) -> usize`

- `unsafe fn deref<'a>(ptr: usize) -> &'a T`

- `unsafe fn deref_mut<'a>(ptr: usize) -> &'a mut T`

- `unsafe fn drop(ptr: usize)`

